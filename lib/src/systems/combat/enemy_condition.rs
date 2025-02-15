use crate::{
    components::{DamageTaken, Effect, EffectQueue},
    data::{Damage, EnemyCondition, EnemyEffect, PlayerCondition},
    systems::enemy::EnemyState,
    types::Strength,
};

impl EnemyCondition {
    /// Attempts to merge the supplied condition into self, returning true iff the conditions
    /// were merged.
    pub fn merge(&mut self, other: &Self) -> bool {
        match other {
            EnemyCondition::CurlUp(incoming_block) => {
                if let EnemyCondition::CurlUp(block) = self {
                    *block += incoming_block;
                    return true;
                }
            }
            EnemyCondition::Enrage(incoming_strength) => {
                if let EnemyCondition::Enrage(strength) = self {
                    *strength += incoming_strength;
                    return true;
                }
            }
            EnemyCondition::Ritual(incoming_strength, incoming_just_applied) => {
                if let EnemyCondition::Ritual(strength, just_applied) = self {
                    *strength += incoming_strength;
                    *just_applied = *just_applied || *incoming_just_applied;
                    return true;
                }
            }
            EnemyCondition::SporeCloud(incoming_stacks) => {
                if let EnemyCondition::SporeCloud(stacks) = self {
                    *stacks += incoming_stacks;
                    return true;
                }
            }
            EnemyCondition::StrengthLossThisTurn(incoming_strength) => {
                if let EnemyCondition::StrengthLossThisTurn(strength) = self {
                    *strength += incoming_strength;
                    return true;
                }
            }
            EnemyCondition::Thorns(incoming_hp) => {
                if let EnemyCondition::Thorns(hp) = self {
                    *hp += incoming_hp;
                    return true;
                }
            }
            EnemyCondition::Vulnerable(incoming_turns) => {
                if let EnemyCondition::Vulnerable(turns) = self {
                    *turns += incoming_turns;

                    return true;
                }
            }
            EnemyCondition::Weak(incoming_turns) => {
                if let EnemyCondition::Weak(turns) = self {
                    *turns += incoming_turns;
                    return true;
                }
            }
        }
        false
    }

    /// Ticks down a condition's turn counter at the start of the enemies' turn.
    /// Returns true iff the condition is still active.
    pub fn start_turn(&mut self) -> bool {
        true
    }

    /// Ticks down the conditions at the end of the enemies' turn.
    /// Returns true iff the condition is still active.
    pub fn end_turn(&mut self, enemy_strength: &mut Strength) -> bool {
        match self {
            EnemyCondition::Ritual(strength, just_applied) => {
                if *just_applied {
                    *just_applied = false;
                } else {
                    *enemy_strength += *strength;
                }
                true
            }
            EnemyCondition::Vulnerable(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
            EnemyCondition::Weak(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
            _ => true,
        }
    }

    /// Queues any effects triggered by the enemy taking damage.
    pub fn on_damage_taken(
        &mut self,
        enemy_state: &mut EnemyState,
        damage_taken: &DamageTaken,
        effect_queue: &mut EffectQueue,
    ) -> bool {
        match self {
            EnemyCondition::CurlUp(block) => {
                enemy_state.block = enemy_state.block.saturating_add(*block);
                false
            }
            EnemyCondition::Thorns(hp) if damage_taken.provokes_thorns => {
                effect_queue.push_front(Effect::EnemyState(EnemyEffect::Deal(
                    Damage::BlockableNonAttack(*hp),
                )));
                true
            }
            _ => true,
        }
    }

    /// Queues any effects triggered by the enemy dying.
    pub fn on_death(&mut self, effect_queue: &mut EffectQueue) -> bool {
        match self {
            EnemyCondition::SporeCloud(stacks) => {
                effect_queue.push_front(Effect::EnemyState(EnemyEffect::Inflict(
                    PlayerCondition::Vulnerable(*stacks),
                )));
                true
            }
            _ => true,
        }
    }
}
