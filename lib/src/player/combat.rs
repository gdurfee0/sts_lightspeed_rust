use anyhow::Error;

use crate::data::Card;
use crate::enemy::{Enemy, EnemyStatus, EnemyType};
use crate::rng::StsRandom;
use crate::{
    AttackDamage, Block, Buff, Debuff, Effect, EnemyIndex, Energy, HandIndex, Hp, StackCount,
};

use super::action::{
    Action, CardDetails, EffectChain, EnemyEffectChain, PlayerEffectChain, Target,
};
use super::comms::Comms;
use super::message::CardPlay;
use super::state::PlayerState;

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct CombatController<'a> {
    shuffle_rng: StsRandom,
    energy: Energy,
    block: Block,
    buffs: Vec<(Buff, StackCount)>,
    debuffs: Vec<(Debuff, StackCount)>,
    hand: Vec<Card>,
    card_just_played: Option<HandIndex>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
    exhaust_pile: Vec<Card>,

    state: &'a mut PlayerState,
    comms: &'a mut Comms,
}

impl<'a> CombatController<'a> {
    pub fn new(
        mut shuffle_rng: StsRandom,
        state: &'a mut PlayerState,
        comms: &'a mut Comms,
    ) -> Self {
        let hand = Vec::new();
        let mut draw_pile = state.deck().to_vec();
        shuffle_rng.java_compat_shuffle(&mut draw_pile);
        let discard_pile = Vec::new();
        let exhaust_pile = Vec::new();
        let debuffs = Vec::new();
        Self {
            shuffle_rng,
            energy: 3,
            block: 0,
            buffs: Vec::new(),
            debuffs,
            hand,
            card_just_played: None,
            draw_pile,
            discard_pile,
            exhaust_pile,
            state,
            comms,
        }
    }

    pub fn hp(&self) -> Hp {
        self.state.hp()
    }

    pub fn start_turn(&mut self) -> Result<(), Error> {
        // Reset energy
        self.energy = 3;

        // Draw cards
        self.draw_cards()?;

        // Set block to 0
        if self.block > 0 {
            self.block = 0;
            self.comms.send_block(self.block)?;
        }

        // TODO: Apply other start-of-turn effects
        Ok(())
    }

    pub fn end_turn(&mut self) -> Result<(), Error> {
        self.discard_hand()?;

        // Tick down debuffs
        for (_, stacks) in self.debuffs.iter_mut() {
            *stacks = (*stacks - 1).max(0);
        }
        self.debuffs.retain(|(_, stacks)| *stacks > 0);
        self.comms.send_debuffs(&self.debuffs)?;

        // TODO: Apply other end-of-turn effects
        Ok(())
    }

    pub fn take_damage(&mut self, amount: AttackDamage) -> Result<(), Error> {
        if amount <= self.block {
            self.block -= amount;
            self.comms.send_damage_blocked(amount)?;
            self.comms.send_block_lost(amount)?;
            self.comms.send_block(self.block)
        } else if self.block > 0 {
            let remaining_damage = amount - self.block;
            self.block = 0;
            self.comms.send_damage_blocked(amount)?;
            self.comms.send_block_lost(amount)?;
            self.comms.send_block(self.block)?;
            self.comms.send_damage_taken(remaining_damage)?;
            self.state.decrease_hp(remaining_damage);
            self.comms.send_health_changed(self.state.health())
        } else {
            self.state.decrease_hp(amount);
            self.comms.send_damage_taken(amount)?;
            self.comms.send_health_changed(self.state.health())
        }
    }

    pub fn draw_cards(&mut self) -> Result<(), Error> {
        // Draw new cards
        let draw_count = 5;
        for i in 0..draw_count {
            if let Some(card) = self.draw_pile.pop() {
                self.hand.push(card);
                self.comms.send_card_drawn(i, card)?;
            } else {
                // Shuffle discard pile into draw pile
                self.comms.send_shuffling_discard_to_draw()?;
                self.shuffle_rng.java_compat_shuffle(&mut self.discard_pile);
                self.draw_pile.append(&mut self.discard_pile);
                if let Some(card) = self.draw_pile.pop() {
                    self.hand.push(card);
                    self.comms.send_card_drawn(i, card)?;
                }
            }
        }
        Ok(())
    }

    pub fn add_to_discard_pile(&mut self, cards: &[Card]) -> Result<(), Error> {
        for card in cards {
            self.discard_pile.push(*card);
        }
        self.comms.send_add_to_discard_pile(cards)
    }

    fn discard_hand(&mut self) -> Result<(), Error> {
        // Emulating the game's behavior
        while let Some(card) = self.hand.pop() {
            self.discard_pile.push(card);
        }
        self.comms.send_hand_discarded()
    }

    pub fn enemy_died(&self, index: EnemyIndex, enemy_type: EnemyType) -> Result<(), Error> {
        self.comms.send_enemy_died(index, enemy_type)
    }

    fn available_card_plays(&self) -> Vec<CardPlay> {
        self.hand
            .iter()
            .enumerate()
            .map(|(hand_index, card)| {
                let card_details = CardDetails::for_card(*card);
                CardPlay {
                    hand_index,
                    card: *card,
                    cost: card_details.cost,
                    effect_chain: self.to_player_effect_chain(&card_details.effect_chain),
                    target: card_details.target,
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn choose_next_action(&mut self, enemies: &[Option<Enemy>]) -> Result<Action, Error> {
        // TODO: drink a potion, discard a potion
        // TODO: check for unwinnable situations

        let enemy_statuses = enemies
            .iter()
            .map(|maybe_enemy| maybe_enemy.as_ref().map(|enemy| enemy.status()))
            .collect::<Vec<_>>();
        self.comms.send_enemy_party(enemy_statuses)?;
        self.comms.send_energy(self.energy)?;
        let card_plays = self.available_card_plays();
        match self.comms.choose_card_to_play(&card_plays)? {
            Some(card_play) => {
                self.card_just_played = Some(card_play.hand_index);
                if card_play.target == Target::Player {
                    Ok(Action::ApplyEffectChainToPlayer(card_play.effect_chain))
                } else {
                    self.resolve_card_play_against_enemies(card_play, enemies)
                }
            }
            None => Ok(Action::EndTurn),
        }
    }

    fn resolve_card_play_against_enemies(
        &mut self,
        card_play: CardPlay,
        enemies: &[Option<Enemy>],
    ) -> Result<Action, Error> {
        let mut enemy_effect_chains = enemies
            .iter()
            .map(|maybe_enemy| {
                maybe_enemy
                    .as_ref()
                    .map(|enemy| self.to_enemy_effect_chain(&card_play.effect_chain, enemy))
            })
            .collect::<Vec<_>>();
        match card_play.target {
            Target::OneEnemy => {
                let enemy_index = self
                    .comms
                    .choose_enemy_to_target(enemies, &enemy_effect_chains)?;
                Ok(Action::ApplyEffectChainToEnemy(
                    enemy_effect_chains[enemy_index]
                        .take()
                        .unwrap_or_else(|| panic!("No enemy at index {}", enemy_index)),
                    enemy_index,
                ))
            }
            Target::AllEnemies => Ok(Action::ApplyEffectChainToAllEnemies(enemy_effect_chains)),
            _ => unreachable!(),
        }
    }

    pub fn discard_card_just_played(&mut self) -> Result<(), Error> {
        if let Some(hand_index) = self.card_just_played {
            let card = self.hand.remove(hand_index);
            self.discard_pile.push(card);
            self.comms.send_card_discarded(hand_index, card)?;
        }
        Ok(())
    }

    pub fn apply_debuff(&mut self, debuff: Debuff, stacks: StackCount) -> Result<(), Error> {
        if let Some((_, c)) = self.debuffs.iter_mut().find(|(d, _)| *d == debuff) {
            *c += stacks;
        } else {
            self.debuffs.push((debuff, stacks));
        }
        self.comms.send_debuffs(&self.debuffs)
    }

    pub fn gain_block(&mut self, amount: Block) -> Result<(), Error> {
        self.comms.send_block_gained(amount)?;
        self.block = self.block.saturating_add(amount);
        self.comms.send_block(self.block)
    }

    pub fn has_debuff(&self, debuff: Debuff) -> bool {
        self.debuffs.iter().any(|(d, _)| *d == debuff)
    }

    pub fn update_enemy_status(&self, index: EnemyIndex, status: EnemyStatus) -> Result<(), Error> {
        self.comms.send_enemy_status(index, status)
    }

    pub fn to_player_effect_chain(&self, effect_chain: &EffectChain) -> PlayerEffectChain {
        PlayerEffectChain::new(
            effect_chain
                .iter()
                .map(|effect| match effect {
                    Effect::AttackDamage(_) => {
                        // TODO: Strength etc
                        effect.clone()
                    }
                    Effect::GainBlock(amount) => {
                        if self.has_debuff(Debuff::Frail) {
                            Effect::GainBlock((*amount as f32 * 0.75).floor() as Block)
                        } else {
                            Effect::GainBlock(*amount)
                        }
                    }
                    Effect::Inflict(_, _) => effect.clone(),
                    _ => todo!("{:?}", effect),
                })
                .collect(),
        )
    }

    pub fn to_enemy_effect_chain(
        &self,
        player_effect_chain: &PlayerEffectChain,
        enemy: &Enemy,
    ) -> EnemyEffectChain {
        EnemyEffectChain::new(
            player_effect_chain
                .iter()
                .cloned()
                .map(|effect| enemy.account_for_buffs_and_debuffs(effect))
                .collect(),
        )
    }
}
