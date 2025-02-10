use crate::data::{EnemyCondition, PlayerCondition};

pub trait Condition: Sized {
    fn merge(&mut self, other: &Self) -> bool;
    fn start_turn(&mut self) -> bool;
    fn end_turn(&mut self) -> bool;
}

impl Condition for EnemyCondition {
    fn merge(&mut self, other: &Self) -> bool {
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

    fn start_turn(&mut self) -> bool {
        true
    }

    fn end_turn(&mut self) -> bool {
        match self {
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
}

impl Condition for PlayerCondition {
    fn merge(&mut self, other: &Self) -> bool {
        match other {
            PlayerCondition::Artifact(incoming_counter) => {
                if let PlayerCondition::Artifact(counter) = self {
                    *counter += incoming_counter;
                    return true;
                }
            }
            PlayerCondition::Barricade => {
                if let PlayerCondition::Barricade = self {
                    return true;
                }
            }
            PlayerCondition::Berserk(incoming_energy) => {
                if let PlayerCondition::Berserk(energy) = self {
                    *energy += incoming_energy;
                    return true;
                }
            }
            PlayerCondition::Brutality(incoming_draw_count) => {
                if let PlayerCondition::Brutality(draw_count) = self {
                    *draw_count += incoming_draw_count;
                    return true;
                }
            }
            PlayerCondition::Combust(incoming_damage_to_self, incoming_damage_to_enemies) => {
                if let PlayerCondition::Combust(damage_to_self, damage_to_enemies) = self {
                    *damage_to_self += incoming_damage_to_self;
                    *damage_to_enemies += incoming_damage_to_enemies;
                    return true;
                }
            }
            PlayerCondition::Confused => {
                if let PlayerCondition::Confused = self {
                    return true;
                }
            }
            PlayerCondition::Corruption => {
                if let PlayerCondition::Corruption = self {
                    return true;
                }
            }
            PlayerCondition::DarkEmbrace(incoming_draw_count) => {
                if let PlayerCondition::DarkEmbrace(draw_count) = self {
                    *draw_count += incoming_draw_count;
                    return true;
                }
            }
            PlayerCondition::DemonForm(incoming_strength) => {
                if let PlayerCondition::DemonForm(strength) = self {
                    *strength += incoming_strength;
                    return true;
                }
            }
            PlayerCondition::DoubleTap(incoming_attack_count) => {
                if let PlayerCondition::DoubleTap(attack_count) = self {
                    *attack_count += incoming_attack_count;
                    return true;
                }
            }
            PlayerCondition::Evolve(incoming_draw_count) => {
                if let PlayerCondition::Evolve(draw_count) = self {
                    *draw_count += incoming_draw_count;
                    return true;
                }
            }
            PlayerCondition::FeelNoPain(incoming_block) => {
                if let PlayerCondition::FeelNoPain(block) = self {
                    *block += incoming_block;
                    return true;
                }
            }
            PlayerCondition::FireBreathing(incoming_damage_to_enemies) => {
                if let PlayerCondition::FireBreathing(damage_to_enemies) = self {
                    *damage_to_enemies += incoming_damage_to_enemies;
                    return true;
                }
            }
            PlayerCondition::FlameBarrier(incoming_damage) => {
                if let PlayerCondition::FlameBarrier(damage) = self {
                    *damage += incoming_damage;
                    return true;
                }
            }
            PlayerCondition::Frail(incoming_turns) => {
                if let PlayerCondition::Frail(turns) = self {
                    *turns += incoming_turns;
                    return true;
                }
            }
            PlayerCondition::Intangible(incoming_turns) => {
                if let PlayerCondition::Intangible(turns) = self {
                    *turns += incoming_turns;
                    return true;
                }
            }
            PlayerCondition::Juggernaut(incoming_damage) => {
                if let PlayerCondition::Juggernaut(damage) = self {
                    *damage += incoming_damage;
                    return true;
                }
            }
            PlayerCondition::Magnetism(incoming_stack_count) => {
                if let PlayerCondition::Magnetism(stack_count) = self {
                    *stack_count += incoming_stack_count;
                    return true;
                }
            }
            PlayerCondition::Mayhem(incoming_stack_count) => {
                if let PlayerCondition::Mayhem(stack_count) = self {
                    *stack_count += incoming_stack_count;
                    return true;
                }
            }
            PlayerCondition::Metallicize(incoming_block) => {
                if let PlayerCondition::Metallicize(block) = self {
                    *block += incoming_block;
                    return true;
                }
            }
            PlayerCondition::NoBlock(incoming_turns) => {
                if let PlayerCondition::NoBlock(turns) = self {
                    *turns += incoming_turns;
                    return true;
                }
            }
            PlayerCondition::NoDraw => {
                if let PlayerCondition::NoDraw = self {
                    return true;
                }
            }
            PlayerCondition::Panache(incoming_stack_count, incoming_damage) => {
                if let PlayerCondition::Panache(stack_count, damage) = self {
                    *stack_count = (*stack_count).max(*incoming_stack_count);
                    *damage += incoming_damage;
                    return true;
                }
            }
            PlayerCondition::Rage(incoming_block) => {
                if let PlayerCondition::Rage(block) = self {
                    *block += incoming_block;
                    return true;
                }
            }
            PlayerCondition::Rupture(incoming_strength) => {
                if let PlayerCondition::Rupture(strength) = self {
                    *strength += incoming_strength;
                    return true;
                }
            }
            PlayerCondition::Sadistic(incoming_damage) => {
                if let PlayerCondition::Sadistic(damage) = self {
                    *damage += incoming_damage;
                    return true;
                }
            }
            PlayerCondition::StrengthDown(incoming_strength) => {
                if let PlayerCondition::StrengthDown(strength) = self {
                    *strength += incoming_strength;
                    return true;
                }
            }
            PlayerCondition::TheBomb(_, _) => {
                return false; // Doesn't stack
            }
            PlayerCondition::Vulnerable(incoming_turns) => {
                if let PlayerCondition::Vulnerable(turns) = self {
                    *turns += incoming_turns;
                    return true;
                }
            }
            PlayerCondition::Weak(incoming_turns) => {
                if let PlayerCondition::Weak(turns) = self {
                    *turns += incoming_turns;
                    return true;
                }
            }
        }
        false
    }

    fn start_turn(&mut self) -> bool {
        match self {
            PlayerCondition::FlameBarrier(_) => false,
            _ => true,
        }
    }

    fn end_turn(&mut self) -> bool {
        match self {
            PlayerCondition::Artifact(_) => true,
            PlayerCondition::Barricade => true,
            PlayerCondition::Berserk(_) => true,
            PlayerCondition::Brutality(_) => true,
            PlayerCondition::Combust(_, _) => true,
            PlayerCondition::Confused => true,
            PlayerCondition::Corruption => true,
            PlayerCondition::DarkEmbrace(_) => true,
            PlayerCondition::DemonForm(_) => true,
            PlayerCondition::DoubleTap(_) => false, // This turn only
            PlayerCondition::Evolve(_) => true,
            PlayerCondition::FeelNoPain(_) => true,
            PlayerCondition::FireBreathing(_) => true,
            PlayerCondition::FlameBarrier(_) => true,
            PlayerCondition::Frail(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
            PlayerCondition::Intangible(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
            PlayerCondition::Juggernaut(_) => true,
            PlayerCondition::Magnetism(_) => true,
            PlayerCondition::Mayhem(_) => true,
            PlayerCondition::Metallicize(_) => true,
            PlayerCondition::NoBlock(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
            PlayerCondition::NoDraw => false, // This turn only
            PlayerCondition::Panache(stack_count, _) => {
                *stack_count = 5; // Always rests to 5 at end of turn
                true
            }
            PlayerCondition::Rage(_) => false, // This turn only
            PlayerCondition::Rupture(_) => true,
            PlayerCondition::Sadistic(_) => true,
            PlayerCondition::StrengthDown(_) => false, // This turn only
            PlayerCondition::TheBomb(turns, damage) => {
                *turns = turns.saturating_sub(1);
                if *turns == 0 {
                    todo!("implement bomb damage {}", damage)
                }
                *turns > 0
            }
            PlayerCondition::Vulnerable(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
            PlayerCondition::Weak(turns) => {
                *turns = turns.saturating_sub(1);
                *turns > 0
            }
        }
    }
}
