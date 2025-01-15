use std::iter::{once, repeat};
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::data::{Card, Character, EnemyType, Intent, NeowBlessing, Potion, Relic};
use crate::rng::StsRandom;

use super::action::{Debuff, Effect};
use super::message::{Choice, Prompt, StsMessage};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct Player {
    hp: u32,
    hp_max: u32,
    gold: u32,
    relics: Vec<Relic>,
    deck: Vec<Card>,
    potions: Vec<Option<Potion>>,

    // Communication channels
    input_rx: Receiver<usize>,
    output_tx: Sender<StsMessage>,
}

#[derive(Debug)]
pub struct PlayerInCombat<'a> {
    player: &'a mut Player,
    shuffle_rng: StsRandom,

    // Combat state
    energy: u32,
    debuffs: Vec<(Debuff, u32)>,
    hand: Vec<Card>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
    //exhaust_pile: Vec<Card>,
}

#[derive(Clone, Debug)]
pub enum PlayerAction {
    EndTurn,
    PlayerMove(&'static PlayerMove, usize),
    PlayerMoveWithTarget(&'static PlayerMove, usize, usize),
}

/// Some convenience methods for Player interaction.
impl Player {
    pub fn new(
        character: &'static Character,
        input_rx: Receiver<usize>,
        output_tx: Sender<StsMessage>,
    ) -> Self {
        let relics = vec![character.starting_relic];
        let deck = character.starting_deck.to_vec();
        let potions = [None; 3].to_vec();
        Self {
            hp: character.starting_hp,
            hp_max: character.starting_hp,
            gold: 99,
            relics,
            deck,
            potions,
            input_rx,
            output_tx,
        }
    }

    pub fn hp_max(&self) -> u32 {
        self.hp_max
    }

    pub fn hp(&self) -> u32 {
        self.hp
    }

    pub fn gold(&self) -> u32 {
        self.gold
    }

    pub fn take_damage(&mut self, amount: u32) -> Result<(), Error> {
        self.hp = self.hp.saturating_sub(amount);
        self.output_tx
            .send(StsMessage::HealthChanged(self.hp, self.hp_max))?;
        if self.hp == 0 {
            self.output_tx.send(StsMessage::GameOver(false))?;
            Err(anyhow!("Player died"))
        } else {
            Ok(())
        }
    }

    pub fn increase_hp_max(&mut self, amount: u32) -> Result<(), Error> {
        self.hp_max = self.hp_max.saturating_add(amount);
        self.hp = self.hp.saturating_add(amount);
        self.output_tx
            .send(StsMessage::HealthChanged(self.hp, self.hp_max))?;
        Ok(())
    }

    pub fn decrease_hp_max(&mut self, amount: u32) -> Result<(), Error> {
        self.hp_max = self.hp_max.saturating_sub(amount);
        self.hp = self.hp.min(self.hp_max);
        self.output_tx
            .send(StsMessage::HealthChanged(self.hp, self.hp_max))?;
        Ok(())
    }

    pub fn decrease_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.gold = self.gold.saturating_sub(amount);
        self.output_tx.send(StsMessage::GoldChanged(self.gold))?;
        Ok(())
    }

    pub fn increase_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.gold = self.gold.saturating_add(amount);
        self.output_tx.send(StsMessage::GoldChanged(self.gold))?;
        Ok(())
    }

    pub fn send_initial_state(&self) -> Result<(), Error> {
        self.output_tx.send(StsMessage::Deck(self.deck.clone()))?;
        self.output_tx
            .send(StsMessage::Potions(self.potions.clone()))?;
        self.output_tx
            .send(StsMessage::Relics(self.relics.clone()))?;
        Ok(())
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(map_string))?;
        Ok(())
    }

    pub fn obtain_card(&mut self, card: Card) -> Result<(), Error> {
        self.deck.push(card);
        self.output_tx.send(StsMessage::CardObtained(card))?;
        Ok(())
    }

    pub fn obtain_relic(&mut self, relic: Relic) -> Result<(), Error> {
        self.relics.push(relic);
        self.output_tx.send(StsMessage::RelicObtained(relic))?;
        Ok(())
    }

    pub fn choose_neow_blessing(
        &mut self,
        blessings: &[NeowBlessing; 4],
    ) -> Result<NeowBlessing, Error> {
        let choices = blessings
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::ChooseNeow, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::NeowBlessing(blessing)) => Ok(*blessing),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_movement_option(&mut self, options: Vec<u8>) -> Result<u8, Error> {
        let choices = options
            .iter()
            .map(|col| Choice::ClimbFloor(*col))
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::ClimbFloor, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::ClimbFloor(col)) => Ok(*col),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_card_to_obtain(&mut self, card_vec: Vec<Card>) -> Result<(), Error> {
        let choices = card_vec
            .into_iter()
            .map(Choice::ObtainCard)
            .chain(once(Choice::Skip))
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::ChooseOne, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::ObtainCard(card)) => {
                self.obtain_card(*card)?;
                Ok(())
            }
            Some(Choice::Skip) => Ok(()),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_potions_to_obtain(&mut self, mut choices_vec: Vec<Potion>) -> Result<(), Error> {
        loop {
            let next_available_slot = self.potions.iter().position(Option::is_none);
            if let Some(slot) = next_available_slot {
                let choices = choices_vec
                    .clone()
                    .into_iter()
                    .map(Choice::ObtainPotion)
                    .chain(once(Choice::Skip))
                    .collect::<Vec<_>>();
                self.output_tx
                    .send(StsMessage::Choices(Prompt::ChooseNext, choices.clone()))?;
                let choice_index = self.input_rx.recv()?;
                match choices.get(choice_index) {
                    Some(Choice::ObtainPotion(potion)) => {
                        self.potions[slot] = Some(*potion);
                        self.output_tx
                            .send(StsMessage::PotionObtained(*potion, slot as u8))?;
                    }
                    Some(Choice::Skip) => break,
                    _ => return Err(anyhow!("Invalid choice")),
                }
                choices_vec.remove(choice_index);
            } else {
                // No available slots.
                break;
            }
        }
        self.output_tx
            .send(StsMessage::Potions(self.potions.clone()))?;
        Ok(())
    }

    pub fn choose_card_to_remove(&mut self) -> Result<(), Error> {
        let choices = self
            .deck
            .iter()
            .copied()
            .map(Choice::RemoveCard)
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::RemoveCard, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::RemoveCard(card)) => {
                let index = self
                    .deck
                    .iter()
                    .position(|&c| c == *card)
                    .expect("Card not found");
                self.deck.remove(index);
                self.output_tx.send(StsMessage::CardRemoved(*card))?;
                Ok(())
            }
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn enter_combat(&mut self, shuffle_rng: StsRandom) -> PlayerInCombat {
        PlayerInCombat::begin_combat(self, shuffle_rng)
    }
}

impl<'a> PlayerInCombat<'a> {
    fn begin_combat(player: &'a mut Player, mut shuffle_rng: StsRandom) -> Self {
        let hand = Vec::new();
        let mut draw_pile = player.deck.clone();
        shuffle_rng.java_compat_shuffle(&mut draw_pile);
        let discard_pile = Vec::new();
        //let exhaust_pile = Vec::new();
        let debuffs = Vec::new();
        Self {
            player,
            shuffle_rng,
            hand,
            draw_pile,
            discard_pile,
            //exhaust_pile,
            debuffs,
            energy: 3,
        }
    }

    pub fn start_turn(&mut self) -> Result<(), Error> {
        // Reset energy
        self.energy = 3;

        // Draw cards
        self.draw_cards()?;

        // Tick down debuffs
        for (_, stacks) in self.debuffs.iter_mut() {
            *stacks = stacks.saturating_sub(1);
        }
        self.debuffs.retain(|(_, stacks)| *stacks > 0);
        self.player
            .output_tx
            .send(StsMessage::DebuffsChanged(self.debuffs.clone()))?;

        // Apply any other start-of-turn effects
        Ok(())
    }

    fn draw_cards(&mut self) -> Result<(), Error> {
        // Draw new cards
        let draw_count = 5;
        for i in 0..draw_count {
            if let Some(card) = self.draw_pile.pop() {
                self.hand.push(card);
                self.player.output_tx.send(StsMessage::CardDrawn(card, i))?;
            } else {
                // Shuffle discard pile into draw pile
                self.player
                    .output_tx
                    .send(StsMessage::ShufflingDiscardToDraw)?;
                self.shuffle_rng.java_compat_shuffle(&mut self.discard_pile);
                println!(
                    "Shuffled discard pile, rng {}: {:?}",
                    self.shuffle_rng.get_counter(),
                    self.discard_pile
                );
                self.draw_pile.append(&mut self.discard_pile);
                if let Some(card) = self.draw_pile.pop() {
                    self.hand.push(card);
                    self.player.output_tx.send(StsMessage::CardDrawn(card, i))?;
                }
            }
        }
        Ok(())
    }

    fn discard_hand(&mut self) -> Result<(), Error> {
        // Emulating the game's behavior
        while let Some(card) = self.hand.pop() {
            self.discard_pile.push(card);
        }
        self.player.output_tx.send(StsMessage::HandDiscarded)?;
        Ok(())
    }

    pub fn choose_next_action(
        &mut self,
        enemy_party_view: Vec<(EnemyType, Intent, (u32, u32))>,
    ) -> Result<PlayerAction, Error> {
        self.player
            .output_tx
            .send(StsMessage::EnemyParty(enemy_party_view.clone()))?;

        // TODO: drink a potion, discard a potion
        // TODO: check for unwinnable situations

        // Playable cards
        let mut choices = self
            .hand
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, card)| Choice::PlayCardFromHand(card, idx))
            .collect::<Vec<_>>();
        choices.push(Choice::EndTurn);
        self.player
            .output_tx
            .send(StsMessage::Choices(Prompt::CombatAction, choices.clone()))?;
        let choice_index = self.player.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::PlayCardFromHand(card, card_index)) => {
                // TODO: discard the card - but after its effects are applied to enemies?
                let player_move = PlayerMove::for_card(*card);
                if player_move.target == Target::OneEnemy {
                    self.choose_enemy_target(player_move, *card_index, enemy_party_view)
                } else {
                    Ok(PlayerAction::PlayerMove(player_move, *card_index))
                }
            }
            Some(Choice::EndTurn) => {
                self.discard_hand()?;
                Ok(PlayerAction::EndTurn)
            }
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_enemy_target(
        &mut self,
        player_move: &'static PlayerMove,
        card_index: usize,
        enemy_party_view: Vec<(EnemyType, Intent, (u32, u32))>,
    ) -> Result<PlayerAction, Error> {
        let mut target_choices = Vec::new();
        for (i, (enemy_type, _, _)) in enemy_party_view.iter().enumerate() {
            target_choices.push(Choice::TargetEnemy(*enemy_type, i));
        }
        self.player.output_tx.send(StsMessage::Choices(
            Prompt::TargetEnemy,
            target_choices.clone(),
        ))?;
        let target_index = self.player.input_rx.recv()?;
        match target_choices.get(target_index) {
            Some(Choice::TargetEnemy(_, target_index)) => Ok(PlayerAction::PlayerMoveWithTarget(
                player_move,
                *target_index,
                card_index,
            )),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn discard_card_at_hand_index(&mut self, index: usize) -> Result<(), Error> {
        let card = self.hand.remove(index);
        self.discard_pile.push(card);
        self.player
            .output_tx
            .send(StsMessage::CardDiscarded(card, index))?;
        Ok(())
    }

    // TODO: Return any reaction that might have been triggered by this effect.
    pub fn apply_effect(&mut self, effect: Effect) -> Result<(), Error> {
        // TODO: Take into account any modifiers on the player's side, such as buffs, debuffs, etc.
        match effect {
            Effect::AddToDiscardPile(cards) => {
                self.discard_pile.extend_from_slice(cards);
                self.player
                    .output_tx
                    .send(StsMessage::DiscardPile(self.discard_pile.clone()))?;
            }
            Effect::DealDamage(amount) => {
                self.player.take_damage(amount)?;
            }
            Effect::Inflict(debuff, stacks) => self.apply_debuff(debuff, stacks)?,
            Effect::GainBlock(_) => todo!(),
        }
        Ok(())
    }

    pub fn apply_debuff(&mut self, debuff: Debuff, stacks: u32) -> Result<(), Error> {
        if let Some((_, c)) = self.debuffs.iter_mut().find(|(d, _)| *d == debuff) {
            *c += stacks;
        } else {
            self.debuffs.push((debuff, stacks));
        }
        self.player
            .output_tx
            .send(StsMessage::DebuffsChanged(self.debuffs.clone()))?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    AllEnemies,
    OneEnemy,
    Player,
}

#[derive(Debug)]
pub struct PlayerMove {
    pub effects: Vec<Effect>,
    pub target: Target,
}

impl PlayerMove {
    fn deal_damage(amount: u32, times: u32) -> PlayerMoveBuilder {
        PlayerMoveBuilder {
            effects: repeat(Effect::DealDamage(amount))
                .take(times as usize)
                .collect(),
        }
    }

    fn gain_block(amount: u32) -> PlayerMove {
        PlayerMove {
            effects: vec![Effect::GainBlock(amount)],
            target: Target::Player,
        }
    }
}

struct PlayerMoveBuilder {
    effects: Vec<Effect>,
}

impl PlayerMoveBuilder {
    fn then_inflict(mut self, debuff: Debuff, stacks: u32) -> Self {
        self.effects.push(Effect::Inflict(debuff, stacks));
        self
    }

    fn to_all_enemies(&self) -> PlayerMove {
        PlayerMove {
            effects: self.effects.clone(),
            target: Target::AllEnemies,
        }
    }

    fn to_one_enemy(&self) -> PlayerMove {
        PlayerMove {
            effects: self.effects.clone(),
            target: Target::OneEnemy,
        }
    }
}

// Convenience macros
macro_rules! define_move {
    ($name:ident, $player_move:expr) => {
        static $name: once_cell::sync::Lazy<PlayerMove> =
            once_cell::sync::Lazy::new(|| $player_move);
    };
}

define_move!(
    BASH,
    PlayerMove::deal_damage(8, 1)
        .then_inflict(Debuff::Vulnerable, 2)
        .to_one_enemy()
);
define_move!(DEFEND, PlayerMove::gain_block(5));
define_move!(STRIKE, PlayerMove::deal_damage(6, 1).to_one_enemy());

impl PlayerMove {
    pub fn for_card(card: Card) -> &'static PlayerMove {
        match card {
            Card::Bash => &BASH,
            Card::Defend => &DEFEND,
            Card::Strike => &STRIKE,
            _ => todo!(),
        }
    }
}
