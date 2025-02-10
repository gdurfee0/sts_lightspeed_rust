use std::fmt;

use anyhow::Error;

use crate::components::{
    CardCombatState, EnemyStatus, Notification, PlayerCombatState, PlayerPersistentState,
    PotionAction,
};
use crate::data::{
    Card, CardDestination, CardPool, CardSelection, CardType, CostModifier, EnergyCost,
    PlayerCondition, Potion, Relic,
};
use crate::systems::damage::FinalCalculatedDamage;
use crate::systems::effects::Condition;
use crate::systems::enemy::EnemyState;
use crate::systems::rng::StsRandom;
use crate::types::{Block, Dexterity, EnemyIndex, HandIndex, StackCount};
use crate::{Choice, Prompt, Seed};

use super::energy::PlayerEnergy;
use super::player_controller::PlayerController;

pub trait PlayerCombatSystem: fmt::Debug {
    //fn start_combat(&self, enemies: &[Option<EnemyStatus>]) -> Result<(), Error>;
    //fn start_turn(&self) -> Result<(), Error>;
    //fn end_turn(&self) -> Result<(), Error>;
    //fn end_combat(&self) -> Result<(), Error>;

    fn apply_condition(
        &self,
        combat_state: &mut PlayerCombatState,
        condition: &PlayerCondition,
    ) -> Result<(), Error>;

    fn create_cards(
        &self,
        combat_state: &mut PlayerCombatState,
        card_pool: CardPool,
        card_selection: CardSelection,
        card_destination: CardDestination,
        cost_modifier: CostModifier,
    ) -> Result<(), Error>;

    fn take_damage(
        &self,
        persistent_state: &mut PlayerPersistentState,
        combat_state: &mut PlayerCombatState,
        enemy_state: &mut EnemyState,
        damage: FinalCalculatedDamage,
    ) -> Result<(), Error>;

    fn take_retaliatory_damage(
        &self,
        persistent_state: &mut PlayerPersistentState,
        combat_state: &mut PlayerCombatState,
        damage: FinalCalculatedDamage,
    ) -> Result<(), Error>;
}

/*
#[derive(Debug)]
pub struct PlayerInCombat<'a> {
    pub player: &'a mut PlayerController,
    pub state: PlayerCombatState,
    turn_number: usize,
    shuffle_rng: StsRandom,
    card_randomizer_rng: StsRandom,
    cards_drawn_each_turn: usize,
    card_just_played: Option<HandIndex>,
}

/// Returned by various methods to indicate the player's choice of action in combat.
#[derive(Debug)]
pub enum CombatAction<'a> {
    EndTurn,
    PlayCard(&'a mut CardCombatState),
    PlayCardAgainstEnemy(&'a mut CardCombatState, EnemyIndex),
}
*/

impl<'a> PlayerInCombat<'a> {
    pub fn new(player: &'a mut PlayerController, seed_for_floor: Seed) -> Self {
        let mut state = PlayerCombatState::new(&player.player_persistent_state.deck);
        let mut shuffle_rng = StsRandom::from(seed_for_floor);
        shuffle_rng.java_compat_shuffle(&mut state.draw_pile);
        let card_randomizer_rng = StsRandom::from(seed_for_floor);
        // Move innate cards to the top of the draw pile
        state.draw_pile.sort_by_key(|card| card.details.innate);

        // TODO: Draw more than 5 cards if there are more than 5 innate cards
        let cards_drawn_each_turn = if player.player_persistent_state.has_relic(Relic::SneckoEye) {
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
            self.state.conditions.push(PlayerCondition::Confused);
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

    pub fn start_turn(&mut self) -> Result<(), Error> {
        self.state.conditions.retain_mut(|c| c.start_turn());
        self.state.energy = 3;
        self.draw_cards()?;
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
        self.discard_hand()?; // TODO: handle on-discard effects
        for condition in self.state.conditions.iter() {
            match condition {
                PlayerCondition::Combust(_, _) => todo!(),
                PlayerCondition::StrengthDown(_) => todo!(),
                _ => {}
            }
        }
        self.state.conditions.retain_mut(|c| c.end_turn());
        self.player
            .comms
            .send_notification(Notification::Conditions(self.state.conditions.clone()));
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

    pub fn dispose_of_card_just_played(&mut self) -> Result<(), Error> {
        let mut rage_stacks: StackCount = 0;
        for condition in self.state.conditions.iter() {
            if let PlayerCondition::Rage(stacks) = condition {
                rage_stacks += *stacks;
            }
        }
        if let Some(hand_index) = self.card_just_played {
            let card_in_combat = self.state.hand.remove(hand_index);
            if card_in_combat.details.type_ == CardType::Attack && rage_stacks > 0 {
                self.gain_block(rage_stacks)?;
            }
            if card_in_combat.details.exhaust {
                self.exhaust_card(hand_index, card_in_combat)
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

    fn exhaust_card(&mut self, hand_index: HandIndex, card: CardCombatState) -> Result<(), Error> {}

    fn discard_hand(&mut self) -> Result<(), Error> {}

    fn adjust_dexterity(&mut self, amount: Dexterity) -> Result<(), Error> {
        self.state.dexterity = self.state.dexterity.saturating_add(amount);
        self.player
            .comms
            .send_notification(Notification::Dexterity(self.state.dexterity))
    }

    pub fn add_cards_to_discard_pile(&mut self, cards: &[Card]) -> Result<(), Error> {
        for card in cards {
            self.state.discard_pile.push(CardCombatState::new(*card));
        }
        self.player
            .comms
            .send_notification(Notification::AddToDiscardPile(cards.to_vec()))
    }

    pub fn add_card_to_discard_pile(&mut self, card: &CardCombatState) -> Result<(), Error> {
        let mut new_card = CardCombatState::new(card.card);
        new_card.cost_this_combat = card.cost_this_combat;
        new_card.cost_this_turn = card.cost_this_turn;
        self.state.discard_pile.push(new_card);
        self.player
            .comms
            .send_notification(Notification::AddToDiscardPile(vec![new_card.card]))
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

    pub fn can_play_card(&self, card: &CardCombatState) -> bool {
        PlayerEnergy::can_afford(&self.state, card.cost_this_turn)
        // TODO: custom requirements
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
                .filter(|card| self.can_play_card(card))
                .copied()
                .enumerate()
                .map(|(hand_index, card)| {
                    Choice::PlayCardFromHand(hand_index, card.card, card.cost_this_turn)
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
                    let cost = self.state.hand[*hand_index].cost_this_turn;
                    PlayerEnergy::spend(&mut self.state, &self.player.comms, cost)?;
                    if self.state.hand[*hand_index].details.requires_target {
                        let enemy_index = self.choose_enemy_to_target(enemies)?;
                        return Ok(CombatAction::PlayCardAgainstEnemy(
                            &mut self.state.hand[*hand_index],
                            enemy_index,
                        ));
                    } else {
                        return Ok(CombatAction::PlayCard(&mut self.state.hand[*hand_index]));
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

    pub fn put_card_from_discard_pile_on_top_of_draw_pile(&mut self) -> Result<(), Error> {
        if !self.state.discard_pile.is_empty() {
            let choices = self
                .state
                .discard_pile
                .iter()
                .copied()
                .enumerate()
                .map(|(discard_index, card)| Choice::PutOnTopOfDrawPile(discard_index, card.card))
                .collect::<Vec<_>>();
            match self
                .player
                .comms
                .prompt_for_choice(Prompt::ChooseCardToPutOnTopOfDrawPile, &choices)?
            {
                Choice::PutOnTopOfDrawPile(discard_index, _) => {
                    self.state
                        .draw_pile
                        .push(self.state.discard_pile[*discard_index]);
                    self.state.discard_pile.remove(*discard_index);
                }
                invalid => unreachable!("{:?}", invalid),
            }
        }
        Ok(())
    }
}
