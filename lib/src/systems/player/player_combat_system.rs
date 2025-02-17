use anyhow::Error;

use crate::components::{
    CardCombatState, Choice, EffectQueue, EnemyStatus, Interaction, Notification,
    PlayerCombatState, PlayerPersistentState, Prompt,
};
use crate::data::CardType;
use crate::systems::base::{PotionSystem, RelicSystem};
use crate::systems::combat::{
    BlockSystem, DiscardSystem, DrawSystem, EnergySystem, ExhaustSystem, PlayerConditionSystem,
};
use crate::systems::enemy::EnemyParty;
use crate::systems::rng::Seed;
use crate::types::EnemyIndex;

use super::player_combat_action::CombatAction;

pub struct PlayerCombatSystem {
    pub draw_system: DrawSystem,
}

impl PlayerCombatSystem {
    /// Creates a new player combat system.
    pub fn new(seed_for_floor: Seed) -> Self {
        Self {
            draw_system: DrawSystem::new(seed_for_floor),
        }
    }

    /// Notifies the player of their current combat state and the enemy party.
    pub fn notify_player<I: Interaction>(
        &self,
        comms: &I,
        pcs: &PlayerCombatState,
        enemy_party: &EnemyParty,
    ) -> Result<(), Error> {
        comms.send_notification(Notification::StartingCombat)?;
        comms.send_notification(Notification::EnemyParty(
            enemy_party
                .0
                .iter()
                .map(|enemy| enemy.as_ref().map(EnemyStatus::from))
                .collect(),
        ))?;
        comms.send_notification(Notification::Health((pcs.pps.hp, pcs.pps.hp_max)))?;
        comms.send_notification(Notification::Energy(pcs.energy))?;
        comms.send_notification(Notification::Strength(pcs.strength))?;
        comms.send_notification(Notification::Dexterity(pcs.dexterity))?;
        comms.send_notification(Notification::Conditions(pcs.conditions.clone()))
    }

    /// Kicks off combat by triggering start-of-combat effects and notifying the player of their
    /// combat state as well as the enemy party.
    pub fn start_combat<I: Interaction>(
        &mut self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        enemy_party: &mut EnemyParty,
    ) -> Result<(), Error> {
        RelicSystem::on_start_combat(comms, pcs)?;
        self.draw_system.start_combat(pcs);
        self.notify_player(comms, pcs, enemy_party)
    }

    /// Notifies the player that combat has ended.
    pub fn end_combat<I: Interaction>(
        &self,
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        RelicSystem::on_end_combat(comms, pps)?;
        comms.send_notification(Notification::EndingCombat)
    }

    /// Triggers start-of-turn effects.
    pub fn start_turn<I: Interaction>(
        &mut self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        PlayerConditionSystem::start_turn(comms, pcs)?;
        BlockSystem::start_player_turn(comms, pcs)?;
        EnergySystem::start_turn(comms, pcs)?;
        self.draw_system.start_turn(comms, pcs, effect_queue)
    }

    /// Triggers end-of-turn effects.
    pub fn end_turn<I: Interaction>(
        &mut self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        DiscardSystem::end_turn(comms, pcs, effect_queue)?;
        PlayerConditionSystem::end_turn(comms, pcs)
    }

    /// Prompts the player for their next action.
    pub fn choose_next_action<I: Interaction>(
        &self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        enemy_party: &EnemyParty,
    ) -> Result<CombatAction, Error> {
        self.notify_player(comms, pcs, enemy_party)?;
        loop {
            let mut choices = pcs
                .cards
                .hand
                .iter()
                .enumerate()
                .filter(|(_, combat_card)| Self::can_play_card(pcs, combat_card))
                .map(|(hand_index, combat_card)| {
                    Choice::PlayCardFromHand(
                        hand_index,
                        combat_card.card,
                        combat_card.cost_this_turn,
                    )
                })
                .collect::<Vec<_>>();
            PotionSystem::extend_with_potion_actions(pcs.pps, true, &mut choices);
            choices.push(Choice::EndTurn);
            match comms.prompt_for_choice(Prompt::CombatAction, &choices)? {
                Choice::PlayCardFromHand(hand_index, _, _) => {
                    pcs.cards.card_in_play = Some(*hand_index);
                    let combat_card = pcs.cards.hand[*hand_index];
                    EnergySystem::spend(comms, pcs, combat_card.cost_this_turn)?;
                    if combat_card.details.requires_target {
                        let enemy_index = Self::choose_enemy_to_target(comms, enemy_party)?;
                        return Ok(CombatAction::PlayCard(combat_card, Some(enemy_index)));
                    } else {
                        return Ok(CombatAction::PlayCard(combat_card, None));
                    }
                }
                Choice::ExpendPotion(potion_action) => {
                    PotionSystem::expend_potion_in_combat(comms, pcs, potion_action)?
                }
                Choice::EndTurn => return Ok(CombatAction::EndTurn),
                invalid => unreachable!("{:?}", invalid),
            }
        }
    }

    /// Disposes of the card just played.
    pub fn dispose_of_card_just_played<I: Interaction>(
        &self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        let Some(hand_index) = pcs.cards.card_in_play else {
            panic!("No card in play");
        };
        let combat_card = pcs.cards.hand.remove(hand_index);
        PlayerConditionSystem::on_some_card_played(comms, pcs, &combat_card, effect_queue)?;
        if combat_card.details.exhaust {
            ExhaustSystem::push(comms, pcs, hand_index, combat_card, effect_queue)
        } else {
            DiscardSystem::push(comms, pcs, hand_index, combat_card)
        }
    }

    /// Prompts the player to choose an enemy to target.
    fn choose_enemy_to_target<I: Interaction>(
        comms: &I,
        enemy_party: &EnemyParty,
    ) -> Result<EnemyIndex, Error> {
        let choices = enemy_party
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, maybe_enemy)| {
                maybe_enemy
                    .as_ref()
                    .map(|enemy| Choice::TargetEnemy(index, enemy.enemy))
            })
            .collect::<Vec<_>>();
        match comms.prompt_for_choice(Prompt::TargetEnemy, &choices)? {
            Choice::TargetEnemy(enemy_index, _) => Ok(*enemy_index),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Returns true iff the player can play the given card.
    fn can_play_card(pcs: &PlayerCombatState, combat_card: &CardCombatState) -> bool {
        EnergySystem::can_afford(pcs, combat_card.cost_this_turn)
            && (!combat_card
                .details
                .playable_only_if_all_cards_in_hand_are_attacks
                || pcs
                    .cards
                    .hand
                    .iter()
                    .all(|c| matches!(c.details.type_, CardType::Attack)))
    }
}
