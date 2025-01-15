use std::iter::repeat;

use anyhow::{anyhow, Error};

use crate::data::{AttackAmount, AttackCount, Card, EnemyType};
use crate::rng::StsRandom;

use super::enemy::EnemyStatus;
use super::message::{Choice, Prompt, StsMessage};
use super::player::Player;
use super::{BlockAmount, Debuff, Effect, EnemyIndex, HandIndex, StackCount};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct PlayerInCombat<'a> {
    player: &'a mut Player,
    shuffle_rng: StsRandom,

    // Combat state
    energy: u32,
    debuffs: Vec<(Debuff, StackCount)>,
    hand: Vec<Card>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
    //exhaust_pile: Vec<Card>,
}

#[derive(Clone, Debug)]
pub enum PlayerAction {
    EndTurn,
    PlayerMove(&'static PlayerMove, HandIndex),
    PlayerMoveWithTarget(&'static PlayerMove, HandIndex, EnemyIndex),
}

impl Player {
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
            .send(StsMessage::Debuffs(self.debuffs.clone()))?;

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

    pub fn update_enemy_status(
        &mut self,
        status: EnemyStatus,
        index: EnemyIndex,
    ) -> Result<(), Error> {
        self.player
            .output_tx
            .send(StsMessage::EnemyStatus(status, index))?;
        Ok(())
    }

    pub fn enemy_died(&mut self, enemy_type: EnemyType, index: EnemyIndex) -> Result<(), Error> {
        self.player
            .output_tx
            .send(StsMessage::EnemyDied(enemy_type, index))?;
        Ok(())
    }

    pub fn choose_next_action(&mut self, enemies: &[EnemyType]) -> Result<PlayerAction, Error> {
        // TODO: drink a potion, discard a potion
        // TODO: check for unwinnable situations

        self.player
            .output_tx
            .send(StsMessage::Energy(self.energy))?;
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
                    self.choose_enemy_target(player_move, *card_index, enemies)
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
        hand_index: HandIndex,
        enemies: &[EnemyType],
    ) -> Result<PlayerAction, Error> {
        let mut target_choices = Vec::new();
        for (i, enemy_type) in enemies.iter().enumerate() {
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
                hand_index,
            )),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn discard_card(&mut self, hand_index: HandIndex) -> Result<(), Error> {
        let card = self.hand.remove(hand_index);
        self.discard_pile.push(card);
        self.player
            .output_tx
            .send(StsMessage::CardDiscarded(card, hand_index))?;
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
            Effect::Inflict(debuff, stack_count) => self.apply_debuff(debuff, stack_count)?,
            Effect::GainBlock(_) => todo!(),
        }
        Ok(())
    }

    pub fn apply_debuff(&mut self, debuff: Debuff, stack_count: StackCount) -> Result<(), Error> {
        if let Some((_, c)) = self.debuffs.iter_mut().find(|(d, _)| *d == debuff) {
            *c += stack_count;
        } else {
            self.debuffs.push((debuff, stack_count));
        }
        self.player
            .output_tx
            .send(StsMessage::Debuffs(self.debuffs.clone()))?;
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
    fn deal_damage(amount: AttackAmount, times: AttackCount) -> PlayerMoveBuilder {
        PlayerMoveBuilder {
            effects: repeat(Effect::DealDamage(amount))
                .take(times as usize)
                .collect(),
        }
    }

    fn gain_block(amount: BlockAmount) -> PlayerMove {
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
    fn then_inflict(mut self, debuff: Debuff, stack_count: StackCount) -> Self {
        self.effects.push(Effect::Inflict(debuff, stack_count));
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
