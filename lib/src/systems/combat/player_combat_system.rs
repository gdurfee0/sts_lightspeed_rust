use anyhow::Error;

use crate::components::{
    CardCombatState, Choice, EnemyStatus, Interaction, Notification, PlayerCombatState,
    PlayerPersistentState, Prompt,
};
use crate::data::CardType;
use crate::systems::base::{CombatContext, HealthSystem, PotionSystem, RelicSystem};
use crate::systems::combat::{
    BlockSystem, DiscardSystem, DrawSystem, EnergySystem, ExhaustSystem, PlayerConditionSystem,
};
use crate::types::EnemyIndex;

use super::player_combat_action::PlayerCombatAction;

pub struct PlayerCombatSystem;

impl PlayerCombatSystem {
    pub fn notify_player<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        ctx.comms.send_notification(Notification::EnemyParty(
            ctx.enemy_party
                .0
                .iter()
                .map(|enemy| enemy.as_ref().map(EnemyStatus::from))
                .collect(),
        ))?;
        HealthSystem::notify_player(ctx.comms, ctx.pcs.pps)?;
        EnergySystem::notify_player(ctx)?;
        ctx.comms
            .send_notification(Notification::Strength(ctx.pcs.strength))?;
        ctx.comms
            .send_notification(Notification::Dexterity(ctx.pcs.dexterity))?;
        PlayerConditionSystem::notify_player(ctx)
    }

    /// Kicks off combat by triggering start-of-combat effects and notifying the player of their
    /// combat state as well as the enemy party.
    pub fn on_combat_started<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        RelicSystem::on_combat_started(ctx)?;
        DrawSystem::on_combat_started(ctx);
        PlayerCombatSystem::notify_player(ctx)?;
        ctx.comms.send_notification(Notification::StartingCombat)
    }

    /// Notifies the player that combat has ended.
    pub fn on_combat_finished<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        RelicSystem::on_combat_finished(comms, pps)?;
        comms.send_notification(Notification::EndingCombat)
    }

    /// Triggers start-of-turn effects.
    pub fn on_player_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        PlayerConditionSystem::on_player_turn_started(ctx)?;
        DrawSystem::on_player_turn_started(ctx);
        BlockSystem::on_player_turn_started(ctx)?;
        EnergySystem::on_player_turn_started(ctx)
    }

    /// Triggers end-of-turn effects.
    pub fn on_player_turn_finished<I: Interaction>(
        ctx: &mut CombatContext<I>,
    ) -> Result<(), Error> {
        DiscardSystem::on_player_turn_finished(ctx)?;
        PlayerConditionSystem::on_player_turn_finished(ctx)
    }

    /// Prompts the player for their next action.
    pub fn choose_next_action<I: Interaction>(
        ctx: &mut CombatContext<I>,
    ) -> Result<PlayerCombatAction, Error> {
        PlayerCombatSystem::notify_player(ctx)?;
        loop {
            let mut choices = ctx
                .pcs
                .cards
                .hand
                .iter()
                .enumerate()
                .filter(|(_, combat_card)| Self::can_play_card(&ctx.pcs, combat_card))
                .map(|(hand_index, combat_card)| {
                    Choice::PlayCardFromHand(
                        hand_index,
                        combat_card.card,
                        combat_card.cost_this_turn,
                    )
                })
                .collect::<Vec<_>>();
            PotionSystem::extend_with_potion_actions(ctx.pcs.pps, true, &mut choices);
            choices.push(Choice::EndTurn);
            match ctx
                .comms
                .prompt_for_choice(Prompt::CombatAction, &choices)?
            {
                Choice::PlayCardFromHand(hand_index, _, _) => {
                    ctx.pcs.cards.card_in_play = Some(*hand_index);
                    let combat_card = ctx.pcs.cards.hand[*hand_index];
                    EnergySystem::spend(ctx, combat_card.cost_this_turn)?;
                    if combat_card.details.requires_target {
                        let enemy_index = Self::choose_enemy_to_target(ctx)?;
                        return Ok(PlayerCombatAction::PlayCard(combat_card, Some(enemy_index)));
                    } else {
                        return Ok(PlayerCombatAction::PlayCard(combat_card, None));
                    }
                }
                Choice::ExpendPotion(potion_action) => {
                    PotionSystem::expend_potion_in_combat(ctx, potion_action)?
                }
                Choice::EndTurn => return Ok(PlayerCombatAction::EndTurn),
                invalid => unreachable!("{:?}", invalid),
            }
        }
    }

    /// Disposes of the card just played.
    pub fn dispose_of_card_just_played<I: Interaction>(
        ctx: &mut CombatContext<I>,
    ) -> Result<(), Error> {
        let Some(hand_index) = ctx.pcs.cards.card_in_play else {
            panic!("No card in play");
        };
        let combat_card = ctx.pcs.cards.hand.remove(hand_index);
        PlayerConditionSystem::on_some_card_played(ctx, &combat_card)?;
        if combat_card.details.exhaust {
            ExhaustSystem::push(ctx, hand_index, combat_card)
        } else {
            DiscardSystem::push(ctx, hand_index, combat_card)
        }
    }

    /// Prompts the player to choose an enemy to target.
    fn choose_enemy_to_target<I: Interaction>(
        ctx: &mut CombatContext<I>,
    ) -> Result<EnemyIndex, Error> {
        let choices = ctx
            .enemy_party
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, maybe_enemy)| {
                maybe_enemy
                    .as_ref()
                    .map(|enemy| Choice::TargetEnemy(index, enemy.enemy))
            })
            .collect::<Vec<_>>();
        match ctx.comms.prompt_for_choice(Prompt::TargetEnemy, &choices)? {
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
