use crate::data::Encounter;
use crate::systems::combat::{BlockSystem, EnemyConditionSystem};
use crate::systems::rng::{Seed, StsRandom};

use super::enemy_party::EnemyParty;
use super::enemy_state::EnemyState;

pub struct EnemySystem {
    seed_for_floor: Seed,
    ai_rng: StsRandom,
}

impl EnemySystem {
    /// Creates a new enemy system with the specified seed for the floor.
    pub fn new(seed_for_floor: Seed) -> Self {
        let ai_rng = StsRandom::from(seed_for_floor);
        Self {
            seed_for_floor,
            ai_rng,
        }
    }

    /// Creates a new enemy party for the specified encounter.
    pub fn create_enemy_party(
        &mut self,
        encounter: Encounter,
        misc_rng: &mut StsRandom,
    ) -> EnemyParty {
        EnemyParty::generate(self.seed_for_floor, encounter, &mut self.ai_rng, misc_rng)
    }

    /// Starts the turn for the enemy party.
    pub fn start_turn(&mut self, enemy_party: &mut EnemyParty) {
        EnemyConditionSystem::start_turn(enemy_party);
        BlockSystem::start_enemy_turn(enemy_party);
    }

    /// Ends the turn for the enemy party.
    pub fn end_turn(&mut self, enemy_party: &mut EnemyParty) {
        EnemyConditionSystem::end_turn(enemy_party);
        for enemy_state in enemy_party.0.iter_mut().filter_map(|e| e.as_mut()) {
            self.advance_action(enemy_state);
        }
    }

    /// Computes the next action for the supplied enemy, incrementing the run length if the action
    /// is the same as the previous action.
    fn advance_action(&mut self, enemy_state: &mut EnemyState) {
        let action = enemy_state.next_action;
        enemy_state.next_action = enemy_state.characteristics.next_action(
            &mut self.ai_rng,
            action,
            enemy_state.run_length,
        );
        if enemy_state.next_action == action {
            enemy_state.run_length = enemy_state.run_length.saturating_add(1);
        } else {
            enemy_state.run_length = 1;
        }
    }
}

/*
impl<'a, P: PlayerCombatSystem> EnemySystem for StsEnemySystem<'a, P> {
    fn conduct_turn(
        &mut self,
        enemy_state: &mut EnemyState,
        player_persistent_state: &mut PlayerPersistentState,
        player_combat_state: &mut PlayerCombatState,
    ) -> Result<bool, Error> {
        let next_action = enemy_state.next_action;
        for effect in next_action.effect_chain().iter() {
            match effect {
                EnemyEffect::Apply(condition) => {
                    self.apply_condition(enemy_state, condition);
                }
                EnemyEffect::CreateCards(
                    card_pool,
                    card_selection,
                    card_destination,
                    cost_modifier,
                ) => {
                    self.player_combat_system.create_cards(
                        player_combat_state,
                        *card_pool,
                        *card_selection,
                        *card_destination,
                        *cost_modifier,
                    )?;
                }
                EnemyEffect::Deal(damage) => {
                    let final_calculated_damage = DamageCalculator::calculate_damage(
                        enemy_state,
                        player_combat_state,
                        damage,
                    );
                    self.player_combat_system.take_damage(
                        player_persistent_state,
                        player_combat_state,
                        enemy_state,
                        final_calculated_damage,
                    )?;
                }
                EnemyEffect::Gain(Resource::Block(block)) => {
                    let final_block = DamageCalculator::calculate_block(enemy_state, *block);
                    enemy_state.block = enemy_state.block.saturating_add(final_block);
                }

                EnemyEffect::Gain(Resource::Strength(strength)) => {
                    enemy_state.strength += enemy_state.strength.saturating_add(*strength);
                }
                EnemyEffect::Gain(invalid) => unreachable!("{:?}", invalid),
                EnemyEffect::Inflict(player_condition) => {
                    self.player_combat_system
                        .apply_condition(player_combat_state, player_condition)?;
                }
            }
            if enemy_state.hp == 0 {
                return Ok(false);
            }
            if player_persistent_state.hp == 0 {
                return Ok(true);
            }
        }
        self.advance_action(enemy_state);
        Ok(true)
    }



    fn take_damage(
        &self,
        enemy_state: &mut EnemyState,
        persistent_state: &mut PlayerPersistentState,
        combat_state: &mut PlayerCombatState,
        damage: FinalCalculatedDamage,
    ) -> Result<bool, Error> {
        let damage_taken =
            DamageSystem::take_damage(&mut enemy_state.hp, &mut enemy_state.block, damage);
        enemy_state.conditions.retain(|c| match c {
            EnemyCondition::CurlUp(block) => {
                if damage_taken.hp_lost > 0 {
                    enemy_state.block = enemy_state.block.saturating_add(*block);
                    false
                } else {
                    true
                }
            }
            _ => true,
        });
        for condition in enemy_state.conditions.iter() {
            match condition {
                EnemyCondition::SporeCloud(stack_count) if enemy_state.hp == 0 => {
                    self.player_combat_system.apply_condition(
                        combat_state,
                        &PlayerCondition::Vulnerable(*stack_count),
                    )?;
                }
                EnemyCondition::Thorns(hp) if damage_taken.provokes_thorns => {
                    self.player_combat_system.take_retaliatory_damage(
                        persistent_state,
                        combat_state,
                        FinalCalculatedDamage::Thorns(*hp),
                    )?;
                }
                _ => {}
            }
        }
        Ok(enemy_state.hp > 0)
    }

*/

#[cfg(test)]
mod test {
    use super::super::super::rng::Seed;

    use crate::components::EnemyStatus;
    use crate::data::{Enemy, EnemyAction};
    use crate::systems::enemy::enemy_characteristics::gen_characteristics;

    use super::*;

    #[test]
    fn test_acid_slime() {
        let seed: Seed = 3u64.into();
        let seed_for_floor = seed.with_offset(1);
        let mut system = EnemySystem::new(seed_for_floor);
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyState::new(
            Enemy::AcidSlimeS,
            gen_characteristics(Enemy::AcidSlimeS, &mut hp_rng),
            &mut system.ai_rng,
        );
        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::AcidSlimeS);
        assert_eq!(status.hp, 12);
        assert_eq!(status.hp_max, 12);
        assert_eq!(status.conditions, Vec::new());
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeSLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeSTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeSLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeSTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeSLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeSTackle);

        let mut system = EnemySystem::new(seed_for_floor);
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyState::new(
            Enemy::AcidSlimeM,
            gen_characteristics(Enemy::AcidSlimeM, &mut hp_rng),
            &mut system.ai_rng,
        );

        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::AcidSlimeM);
        assert_eq!(status.hp, 32);
        assert_eq!(status.hp_max, 32);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMCorrosiveSpit);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMCorrosiveSpit);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMCorrosiveSpit);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::AcidSlimeMLick);
    }

    #[test]
    fn test_spike_slime() {
        let seed: Seed = 8u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut system = EnemySystem::new(seed.with_offset(1));
        let mut enemy = EnemyState::new(
            Enemy::SpikeSlimeS,
            gen_characteristics(Enemy::SpikeSlimeS, &mut hp_rng),
            &mut system.ai_rng,
        );

        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::SpikeSlimeS);
        assert_eq!(status.hp, 13);
        assert_eq!(status.hp_max, 13);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeSTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeSTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeSTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeSTackle);

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut system = EnemySystem::new(seed.with_offset(1));
        let mut enemy = EnemyState::new(
            Enemy::SpikeSlimeM,
            gen_characteristics(Enemy::SpikeSlimeM, &mut hp_rng),
            &mut system.ai_rng,
        );

        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::SpikeSlimeM);
        assert_eq!(status.hp, 31);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMFlameTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMFlameTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMFlameTackle);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMLick);
        system.advance_action(&mut enemy);
        assert_eq!(enemy.next_action, EnemyAction::SpikeSlimeMFlameTackle);
    }
}
