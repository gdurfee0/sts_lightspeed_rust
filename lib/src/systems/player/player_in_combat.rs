use anyhow::Error;

use crate::components::{CardInCombat, EnemyStatus, Notification, PlayerCombatState, PotionAction};
use crate::data::{Card, CardType, PlayerCondition, Potion, Relic};
use crate::systems::rng::StsRandom;
use crate::types::{AttackDamage, Block, Dexterity, EnemyIndex, HandIndex, StackCount};
use crate::{Choice, Prompt, Seed};

use super::player::Player;

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.  TODO: lock down field visibility
#[derive(Debug)]
pub struct PlayerInCombat<'a> {
    pub player: &'a mut Player,
    pub state: PlayerCombatState,

    turn_number: usize,
    shuffle_rng: StsRandom,
    card_randomizer_rng: StsRandom,
    cards_drawn_each_turn: usize,
    card_just_played: Option<HandIndex>,
}

/// Returned by various methods to indicate the player's choice of action in combat.
#[derive(Clone, Debug)]
pub enum CombatAction {
    EndTurn,
    PlayCard(CardInCombat),
    PlayCardAgainstEnemy(CardInCombat, EnemyIndex),
}

impl<'a> PlayerInCombat<'a> {
    pub fn new(player: &'a mut Player, seed_for_floor: Seed) -> Self {
        let mut state = PlayerCombatState::new(&player.state.deck);
        let mut shuffle_rng = StsRandom::from(seed_for_floor);
        let card_randomizer_rng = StsRandom::from(seed_for_floor);
        shuffle_rng.java_compat_shuffle(&mut state.draw_pile);
        // Move innate cards to the top of the draw pile
        state.draw_pile.sort_by_key(|card| card.details.innate);
        println!("Draw pile at start of combat:");
        for (i, card) in state.draw_pile.iter().enumerate() {
            println!("    {}: {:?}", i, card.card);
        }

        // TODO: Draw more than 5 cards if there are more than 5 innate cards
        let cards_drawn_each_turn = if player.state.has_relic(Relic::SneckoEye) {
            7
        } else {
            5
        };
        Self {
            player,
            state,
            turn_number: 0,
            shuffle_rng,
            card_randomizer_rng,
            cards_drawn_each_turn,
            card_just_played: None,
        }
    }

    pub fn start_combat(&mut self, enemies: &[Option<EnemyStatus>]) -> Result<(), Error> {
        if self.player.state.has_relic(Relic::SneckoEye) {
            self.state.conditions.push(PlayerCondition::Confused());
        }
        self.player
            .comms
            .send_notification(Notification::StartingCombat)?;
        self.player
            .comms
            .send_notification(Notification::Energy(self.state.energy))?;
        self.player
            .comms
            .send_notification(Notification::Health(self.player.state.health()))?;
        self.player
            .comms
            .send_notification(Notification::Strength(self.state.strength))?;
        self.player
            .comms
            .send_notification(Notification::Dexterity(self.state.dexterity))?;
        self.player
            .comms
            .send_notification(Notification::EnemyParty(enemies.to_vec()))
    }

    pub fn debug_self(&self, message: &str) {
        println!("PlayerInCombat, turn {}, {}", self.turn_number, message);
        println!("    draw_pile:");
        for (i, card) in self.state.draw_pile.iter().enumerate() {
            println!("        {}: {:?}", i, card.card);
        }
        println!("    discard_pile:");
        for (i, card) in self.state.discard_pile.iter().enumerate() {
            println!("        {}: {:?}", i, card.card);
        }
        println!("    hand:");
        for (i, card) in self.state.hand.iter().enumerate() {
            println!("        {}: {:?}", i, card.card);
        }
    }

    pub fn start_turn(&mut self) -> Result<(), Error> {
        // Reset energy
        // TODO: energy conservation
        self.state.energy = 3;

        // Draw cards
        self.debug_self("before drawing cards");
        self.draw_cards()?;

        // Set block to 0
        if self.state.block > 0 {
            self.state.block = 0;
            self.player
                .comms
                .send_notification(Notification::Block(self.state.block))?;
        }

        // TODO: Apply other start-of-turn effects
        Ok(())
    }

    pub fn end_turn(&mut self) -> Result<(), Error> {
        self.discard_hand()?;
        self.tick_down_conditions()?;
        for card in self.state.cards_iter_mut() {
            card.cost_this_turn = card.cost_this_combat;
        }

        self.turn_number += 1;
        // TODO: Apply other end-of-turn effects
        Ok(())
    }

    pub fn end_combat(self) -> Result<(), Error> {
        if self.player.state.has_relic(Relic::BurningBlood) {
            self.player.increase_hp(6)?;
        }
        self.player
            .comms
            .send_notification(Notification::EndingCombat)
    }

    pub fn draw_cards(&mut self) -> Result<(), Error> {
        // Draw new cards
        for _ in 0..self.cards_drawn_each_turn {
            self.draw_card()?;
        }
        Ok(())
    }

    fn draw_card(&mut self) -> Result<(), Error> {
        if let Some(card) = self.state.draw_pile.pop() {
            self.put_card_in_hand(card)
        } else {
            // Shuffle discard pile into draw pile
            self.player
                .comms
                .send_notification(Notification::ShufflingDiscardToDraw)?;
            self.debug_self("before shuffle");
            self.shuffle_rng
                .java_compat_shuffle(&mut self.state.discard_pile);
            self.debug_self("after shuffle");
            self.state.draw_pile.append(&mut self.state.discard_pile);
            if let Some(card) = self.state.draw_pile.pop() {
                self.put_card_in_hand(card)
            } else {
                Ok(())
            }
        }
    }

    fn put_card_in_hand(&mut self, mut card: CardInCombat) -> Result<(), Error> {
        let mut extra_cards_to_draw = 0;
        for condition in self.state.conditions.iter() {
            match condition {
                PlayerCondition::Confused() => {
                    card.cost_this_combat = self.card_randomizer_rng.gen_range(0..=3);
                    card.cost_this_turn = card.cost_this_combat;
                }
                PlayerCondition::Evolve(stacks) => {
                    if matches!(card.details.type_, CardType::Status) {
                        extra_cards_to_draw += *stacks;
                    }
                }
                PlayerCondition::FireBreathing(_) => todo!(),
                PlayerCondition::Frail(_) => {}
                PlayerCondition::Rage(_) => {}
                PlayerCondition::Vulnerable(_) => {}
                PlayerCondition::Weak(_) => {}
            }
        }
        self.state.hand.push(card);
        for _ in 0..extra_cards_to_draw {
            self.draw_card()?;
        }
        self.player.comms.send_notification(Notification::CardDrawn(
            self.state.hand.len() - 1,
            card.card,
            card.cost_this_combat,
        ))
    }

    pub fn dispose_of_card_just_played(&mut self) -> Result<(), Error> {
        let mut rage_stacks: Option<StackCount> = None;
        for condition in self.state.conditions.iter() {
            match condition {
                PlayerCondition::Confused() => {}
                PlayerCondition::Evolve(_) => {}
                PlayerCondition::FireBreathing(_) => todo!(),
                PlayerCondition::Frail(_) => {}
                PlayerCondition::Rage(stacks) => {
                    rage_stacks = Some(*stacks);
                }
                PlayerCondition::Vulnerable(_) => {}
                PlayerCondition::Weak(_) => {}
            }
        }
        if let Some(hand_index) = self.card_just_played {
            let card_in_combat = self.state.hand.remove(hand_index);
            if let (CardType::Attack, Some(stacks)) = (card_in_combat.details.type_, rage_stacks) {
                self.gain_block(stacks)?;
            }
            self.state.energy = self
                .state
                .energy
                .saturating_sub(card_in_combat.cost_this_combat);
            if card_in_combat.details.exhaust {
                self.state.exhaust_pile.push(card_in_combat);
                self.player
                    .comms
                    .send_notification(Notification::CardExhausted(hand_index, card_in_combat.card))
            } else {
                self.state.discard_pile.push(card_in_combat);
                self.player
                    .comms
                    .send_notification(Notification::CardDiscarded(hand_index, card_in_combat.card))
            }
        } else {
            Ok(())
        }
    }

    fn discard_hand(&mut self) -> Result<(), Error> {
        // Emulating the game's behavior
        while let Some(card) = self.state.hand.pop() {
            self.state.discard_pile.push(card);
        }
        self.player
            .comms
            .send_notification(Notification::HandDiscarded)
    }

    fn adjust_dexterity(&mut self, amount: Dexterity) -> Result<(), Error> {
        self.state.dexterity = self.state.dexterity.saturating_add(amount);
        self.player
            .comms
            .send_notification(Notification::Dexterity(self.state.dexterity))
    }

    pub fn apply_condition(&mut self, condition: &PlayerCondition) -> Result<(), Error> {
        for preexisting_condition in self.state.conditions.iter_mut() {
            if Self::maybe_merge_conditions(preexisting_condition, condition) {
                return self
                    .player
                    .comms
                    .send_notification(Notification::Conditions(self.state.conditions.clone()));
            }
        }
        // If we make it here, we didn't have this condition already.
        self.state.conditions.push(condition.clone());
        self.player
            .comms
            .send_notification(Notification::Conditions(self.state.conditions.clone()))
    }

    fn maybe_merge_conditions(
        existing_condition: &mut PlayerCondition,
        incoming_condition: &PlayerCondition,
    ) -> bool {
        match (existing_condition, incoming_condition) {
            (PlayerCondition::Confused(), PlayerCondition::Confused()) => true,
            (PlayerCondition::Evolve(stacks), PlayerCondition::Evolve(additional_stacks)) => {
                *stacks = stacks.saturating_add(*additional_stacks);
                true
            }
            (PlayerCondition::Frail(turns), PlayerCondition::Frail(additional_turns)) => {
                *turns = turns.saturating_add(*additional_turns);
                true
            }
            (PlayerCondition::Rage(stacks), PlayerCondition::Rage(additional_stacks)) => {
                *stacks = stacks.saturating_add(*additional_stacks);
                true
            }
            (PlayerCondition::Vulnerable(turns), PlayerCondition::Vulnerable(additional_turns)) => {
                *turns = turns.saturating_add(*additional_turns);
                true
            }
            (PlayerCondition::Weak(turns), PlayerCondition::Weak(additional_turns)) => {
                *turns = turns.saturating_add(*additional_turns);
                true
            }
            _ => false,
        }
    }

    fn tick_down_conditions(&mut self) -> Result<(), Error> {
        for condition in self.state.conditions.iter_mut() {
            match condition {
                PlayerCondition::Confused() => {}
                PlayerCondition::Evolve(_) => {}
                PlayerCondition::FireBreathing(_) => todo!(),
                PlayerCondition::Frail(turns) => *turns = turns.saturating_sub(1),
                PlayerCondition::Rage(_) => {}
                PlayerCondition::Vulnerable(turns) => *turns = turns.saturating_sub(1),
                PlayerCondition::Weak(turns) => *turns = turns.saturating_sub(1),
            }
        }
        self.state.conditions.retain(|c| {
            !matches!(
                c,
                PlayerCondition::Frail(0)
                    | PlayerCondition::Vulnerable(0)
                    | PlayerCondition::Weak(0)
            )
        });
        self.player
            .comms
            .send_notification(Notification::Strength(self.state.strength))?;
        self.player
            .comms
            .send_notification(Notification::Dexterity(self.state.dexterity))?;
        self.player
            .comms
            .send_notification(Notification::Conditions(self.state.conditions.clone()))
    }

    pub fn add_cards_to_discard_pile(&mut self, cards: &[Card]) -> Result<(), Error> {
        for card in cards {
            self.state.discard_pile.push(CardInCombat::new(None, *card));
        }
        self.player
            .comms
            .send_notification(Notification::AddToDiscardPile(cards.to_vec()))
    }

    pub fn add_card_to_discard_pile(&mut self, card: &CardInCombat) -> Result<(), Error> {
        let mut new_card = CardInCombat::new(None, card.card);
        new_card.cost_this_combat = card.cost_this_combat;
        new_card.cost_this_turn = card.cost_this_turn;
        self.state.discard_pile.push(new_card);
        self.player
            .comms
            .send_notification(Notification::AddToDiscardPile(vec![new_card.card]))
    }

    pub fn take_blockable_damage(&mut self, amount: AttackDamage) -> Result<(), Error> {
        if amount <= self.state.block {
            self.state.block -= amount;
            self.player
                .comms
                .send_notification(Notification::DamageBlocked(amount))?;
            self.player
                .comms
                .send_notification(Notification::BlockLost(amount))?;
            self.player
                .comms
                .send_notification(Notification::Block(self.state.block))
        } else if self.state.block > 0 {
            let remaining_damage = amount - self.state.block;
            self.player
                .comms
                .send_notification(Notification::DamageBlocked(self.state.block))?;
            self.player
                .comms
                .send_notification(Notification::BlockLost(self.state.block))?;
            self.state.block = 0;
            self.player
                .comms
                .send_notification(Notification::Block(0))?;
            self.take_unblockable_damage(remaining_damage)
        } else {
            self.take_unblockable_damage(amount)
        }
    }

    pub fn take_unblockable_damage(&mut self, amount: AttackDamage) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::DamageTaken(amount))?;
        if amount > 0 {
            self.state.hp_loss_count += 1;
            // TODO: Lookup instead of linear pass?
            for card in self.state.cards_iter_mut() {
                if matches!(card.card, Card::BloodForBlood(_)) {
                    card.cost_this_combat = card.cost_this_combat.saturating_sub(1);
                }
            }
        }
        self.player.decrease_hp(amount)
    }

    pub fn gain_block(&mut self, amount: Block) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::BlockGained(amount))?;
        self.state.block = self.state.block.saturating_add(amount);
        self.player
            .comms
            .send_notification(Notification::Block(self.state.block))
    }

    fn expend_potion(&mut self, potion_action: &PotionAction) -> Result<(), Error> {
        match potion_action {
            PotionAction::Discard(_, _) => self.player.expend_potion(potion_action),
            PotionAction::Drink(potion_index, potion) => {
                self.player.state.potions[*potion_index] = None;
                self.player
                    .comms
                    .send_notification(Notification::Potions(self.player.state.potions.to_vec()))?;
                match *potion {
                    Potion::Ambrosia => todo!(),
                    Potion::AncientPotion => todo!(),
                    Potion::AttackPotion => todo!(),
                    Potion::BlessingOfTheForge => todo!(),
                    Potion::BlockPotion => todo!(),
                    Potion::BloodPotion => self.player.expend_potion(potion_action),
                    Potion::BottledMiracle => todo!(),
                    Potion::ColorlessPotion => todo!(),
                    Potion::CultistPotion => todo!(),
                    Potion::CunningPotion => todo!(),
                    Potion::DexterityPotion => self.adjust_dexterity(2),
                    Potion::DistilledChaos => todo!(),
                    Potion::DuplicationPotion => todo!(),
                    Potion::Elixir => todo!(),
                    Potion::EnergyPotion => todo!(),
                    Potion::EntropicBrew => self.player.expend_potion(potion_action),
                    Potion::EssenceOfDarkness => todo!(),
                    Potion::EssenceOfSteel => todo!(),
                    Potion::ExplosivePotion => todo!(),
                    Potion::FairyInABottle => todo!(),
                    Potion::FearPotion => todo!(),
                    Potion::FirePotion => todo!(),
                    Potion::FlexPotion => todo!(),
                    Potion::FocusPotion => todo!(),
                    Potion::FruitJuice => self.player.expend_potion(potion_action),
                    Potion::GamblersBrew => todo!(),
                    Potion::GhostInAJar => todo!(),
                    Potion::HeartOfIron => todo!(),
                    Potion::LiquidBronze => todo!(),
                    Potion::LiquidMemories => todo!(),
                    Potion::PoisonPotion => todo!(),
                    Potion::PotionOfCapacity => todo!(),
                    Potion::PowerPotion => todo!(),
                    Potion::RegenPotion => todo!(),
                    Potion::SkillPotion => todo!(),
                    Potion::SmokeBomb => todo!(),
                    Potion::SneckoOil => todo!(),
                    Potion::SpeedPotion => todo!(),
                    Potion::StancePotion => todo!(),
                    Potion::StrengthPotion => todo!(),
                    Potion::SwiftPotion => todo!(),
                    Potion::WeakPotion => todo!(),
                }
            }
        }
    }

    pub fn choose_next_action(
        &mut self,
        enemies: &[Option<EnemyStatus>],
    ) -> Result<CombatAction, Error> {
        // TODO: check for unwinnable situations
        self.player
            .comms
            .send_notification(Notification::EnemyParty(enemies.to_vec()))?;
        self.player
            .comms
            .send_notification(Notification::Health(self.player.state.health()))?;
        self.player
            .comms
            .send_notification(Notification::Energy(self.state.energy))?;
        loop {
            let mut choices = self
                .state
                .hand
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(hand_index, card)| {
                    if card.cost_this_combat > self.state.energy {
                        None
                    } else {
                        Some(Choice::PlayCardFromHand(
                            hand_index,
                            card.card,
                            card.cost_this_turn,
                        ))
                    }
                })
                .collect::<Vec<_>>();
            self.player.extend_with_potion_choices(&mut choices, true);
            choices.push(Choice::EndTurn);
            match self
                .player
                .comms
                .prompt_for_choice(Prompt::CombatAction, &choices)?
            {
                Choice::PlayCardFromHand(hand_index, _, _) => {
                    self.card_just_played = Some(*hand_index);
                    let card = self.state.hand[*hand_index];
                    if card.details.requires_target {
                        let enemy_index = self.choose_enemy_to_target(enemies)?;
                        return Ok(CombatAction::PlayCardAgainstEnemy(card, enemy_index));
                    } else {
                        return Ok(CombatAction::PlayCard(card));
                    }
                }
                Choice::ExpendPotion(potion_action) => self.expend_potion(potion_action)?,
                Choice::EndTurn => return Ok(CombatAction::EndTurn),
                invalid => unreachable!("{:?}", invalid),
            }
        }
    }

    fn choose_enemy_to_target(&self, enemies: &[Option<EnemyStatus>]) -> Result<EnemyIndex, Error> {
        let choices = enemies
            .iter()
            .enumerate()
            .filter_map(|(index, maybe_enemy)| {
                maybe_enemy
                    .as_ref()
                    .map(|enemy| Choice::TargetEnemy(index, enemy.enemy_type))
            })
            .collect::<Vec<_>>();
        match self
            .player
            .comms
            .prompt_for_choice(Prompt::TargetEnemy, &choices)?
        {
            Choice::TargetEnemy(enemy_index, _) => Ok(*enemy_index),
            invalid => unreachable!("{:?}", invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::data::IRONCLAD;
    use crate::Seed;

    use super::*;

    #[test]
    fn test_blood_for_blood() {
        let (to_server, from_client) = channel();
        let (to_client, from_server) = channel();

        let mut player = Player::new(IRONCLAD, from_client, to_client);
        player.state.deck = vec![Card::BloodForBlood(false)];
        let mut player_in_combat = PlayerInCombat::new(&mut player, Seed::from(3));

        assert_eq!(
            4,
            player_in_combat
                .state
                .draw_pile
                .iter()
                .find(|card| matches!(card.card, Card::BloodForBlood(false)))
                .unwrap()
                .cost_this_combat
        );
        player_in_combat.take_blockable_damage(5).unwrap();
        assert_eq!(
            3,
            player_in_combat
                .state
                .draw_pile
                .iter()
                .find(|card| matches!(card.card, Card::BloodForBlood(false)))
                .unwrap()
                .cost_this_combat
        );
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        assert_eq!(
            0,
            player_in_combat
                .state
                .draw_pile
                .iter()
                .find(|card| matches!(card.card, Card::BloodForBlood(false)))
                .unwrap()
                .cost_this_combat
        );

        drop(to_server);
        drop(from_server);
    }
}
