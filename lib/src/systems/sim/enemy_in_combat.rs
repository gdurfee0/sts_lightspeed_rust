use std::collections::HashMap;
use std::ops::RangeInclusive;

use once_cell::sync::Lazy;

use crate::components::{EnemyState, EnemyStatus};
use crate::data::{Enemy, EnemyAction, EnemyCondition};
use crate::types::{AttackDamage, Block, Hp};

use super::super::rng::StsRandom;

// rng, last_action, run_length
pub type NextActionFn = fn(&mut StsRandom, Option<EnemyAction>, usize) -> EnemyAction;

#[derive(Debug)]
pub struct EnemyInCombat {
    pub state: EnemyState,
    next_action_fn: NextActionFn,
}

impl EnemyInCombat {
    pub fn new(enemy: Enemy, hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Self {
        let (health_range, next_action_fn) = ALL_ENEMIES
            .get(&enemy)
            .unwrap_or_else(|| panic!("Unknown enemy {:?}", enemy));
        let hp_max = hp_rng.gen_range(health_range.clone());
        let next_action = next_action_fn(ai_rng, None, 0);
        Self {
            state: EnemyState::new(enemy, hp_max, next_action),
            next_action_fn: *next_action_fn,
        }
    }

    pub fn next_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        let action = self.state.next_action;
        self.state.next_action = (self.next_action_fn)(ai_rng, Some(action), self.state.run_length);
        if self.state.next_action == action {
            self.state.run_length = self.state.run_length.saturating_add(1);
        } else {
            self.state.run_length = 1;
        }
        action
    }

    pub fn end_turn(&mut self) {
        for condition in self.state.conditions.iter_mut() {
            match condition {
                EnemyCondition::Ritual(intensity, just_applied) => {
                    if !*just_applied {
                        self.state.strength += *intensity as i32;
                    }
                    *just_applied = false;
                }
                EnemyCondition::Vulnerable(turns) => *turns = turns.saturating_sub(1),
                EnemyCondition::Weak(turns) => *turns = turns.saturating_sub(1),
            }
        }
        self.state.conditions.retain(|c| match c {
            EnemyCondition::Vulnerable(turns) => *turns > 0,
            EnemyCondition::Weak(turns) => *turns > 0,
            _ => true,
        });
    }

    pub fn apply_condition(&mut self, condition: &EnemyCondition) {
        for preexisting_condition in self.state.conditions.iter_mut() {
            if Self::maybe_merge_conditions(preexisting_condition, condition) {
                return;
            }
        }
        // If we make it here, we didn't have this condition already.
        self.state.conditions.push(condition.clone());
    }

    fn maybe_merge_conditions(
        existing_condition: &mut EnemyCondition,
        incoming_condition: &EnemyCondition,
    ) -> bool {
        match existing_condition {
            EnemyCondition::Ritual(intensity, just_applied) => {
                if let EnemyCondition::Ritual(additional_intensity, _) = incoming_condition {
                    *intensity = intensity.saturating_add(*additional_intensity);
                    *just_applied = true;
                    return true;
                }
            }
            EnemyCondition::Vulnerable(turns) => {
                if let EnemyCondition::Vulnerable(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
            EnemyCondition::Weak(turns) => {
                if let EnemyCondition::Weak(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
        }
        false
    }

    /// Damage amount must already have player and enemy conditions applied.
    pub fn take_damage(&mut self, amount: AttackDamage) -> (Block, AttackDamage) {
        let block = self.state.block;
        let remaining_damage = amount.saturating_sub(block);
        self.state.block = self.state.block.saturating_sub(amount);
        self.state.hp = self.state.hp.saturating_sub(remaining_damage);
        (block, remaining_damage)
    }
}

impl From<&EnemyInCombat> for EnemyStatus {
    fn from(enemy: &EnemyInCombat) -> Self {
        (&enemy.state).into()
    }
}

macro_rules! define_enemy {
    ($enemy:ident => $hprange:expr, $next_action_fn:expr) => {
        (Enemy::$enemy, ($hprange, $next_action_fn as NextActionFn))
    };
}

macro_rules! define_enemies {
    ($($enemy:ident => $hprange:expr, $next_action_fn:expr,)*) => {
        Lazy::new(
            || vec![$(define_enemy!($enemy => $hprange, $next_action_fn)),*]
                .into_iter()
                .collect::<HashMap<_, _>>()
        )
    }
}

static ALL_ENEMIES: Lazy<HashMap<Enemy, (RangeInclusive<Hp>, NextActionFn)>> = define_enemies!(
    AcidSlimeM => 28..=32, acid_slime_m,
    AcidSlimeS => 8..=12, acid_slime_s,
    Cultist => 48..=54, cultist,
    SpikeSlimeM => 28..=32, spike_slime_m,
    SpikeSlimeS => 10..=14, spike_slime_s,
);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source: Slay the Spire wiki:
// - https://slay-the-spire.fandom.com/wiki/Category:Monster
// - https://slay-the-spire.fandom.com/wiki/Category:Elite
// - https://slay-the-spire.fandom.com/wiki/Category:Boss_Monster
//
// Special thanks to gamerpuppy for figuring out the quirks of the game's use of rng.
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
// AcidSlimeM
// - 28 to 32 HP
// - Corrosive Spit: Deal 7 damage, add a Slimed to the discard pile
// - Lick: Inflict 1 Weak
// - Tackle: Deal 10 damage
// - 30% Corrosive Spit, 40% Tackle, 30% Lick
//  -- Cannot use Corrosive Spit or Lick three times in a row
//  -- Cannot use Tackle twice in a row
// - https://slay-the-spire.fandom.com/wiki/Acid_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
fn acid_slime_m(
    ai_rng: &mut StsRandom,
    last_action: Option<EnemyAction>,
    run_length: usize,
) -> EnemyAction {
    match ai_rng.gen_range(0..100) {
        0..30 if last_action != Some(EnemyAction::AcidSlimeMCorrosiveSpit) || run_length < 2 => {
            EnemyAction::AcidSlimeMCorrosiveSpit
        }
        0..30 => {
            if ai_rng.next_bool() {
                EnemyAction::AcidSlimeMTackle
            } else {
                EnemyAction::AcidSlimeMLick
            }
        }
        30..70 if last_action != Some(EnemyAction::AcidSlimeMTackle) => {
            EnemyAction::AcidSlimeMTackle
        }
        30..70 => *ai_rng.weighted_choose(&[
            (EnemyAction::AcidSlimeMCorrosiveSpit, 0.5),
            (EnemyAction::AcidSlimeMLick, 0.5),
        ]),
        _ if last_action != Some(EnemyAction::AcidSlimeMLick) || run_length < 2 => {
            EnemyAction::AcidSlimeMLick
        }
        _ => *ai_rng.weighted_choose(&[
            (EnemyAction::AcidSlimeMCorrosiveSpit, 0.4),
            (EnemyAction::AcidSlimeMTackle, 0.6),
        ]),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// AcidSlimeS
// - 8 to 12 HP
// - Lick: Inflict 1 Weak
// - Tackle: Deal 3 damage
// - 50% Lick, 50% Tackle for initial action; alternates attacks thereafter
// - https://slay-the-spire.fandom.com/wiki/Acid_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
fn acid_slime_s(ai_rng: &mut StsRandom, last_action: Option<EnemyAction>, _: usize) -> EnemyAction {
    if last_action.is_none() {
        // The game burns an extra roll on the first turn then eschews the rng thereafter.
        let _ = ai_rng.gen_range(0..100);
        if ai_rng.next_bool() {
            EnemyAction::AcidSlimeSTackle
        } else {
            EnemyAction::AcidSlimeSLick
        }
    } else if last_action == Some(EnemyAction::AcidSlimeSLick) {
        EnemyAction::AcidSlimeSTackle
    } else {
        EnemyAction::AcidSlimeSLick
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cultist
// - 48 to 54 HP
// - Incantation: Gain 3 Ritual (first turn only)
// - Dark Strike: Deal 6 damage (all turns after the first)
////////////////////////////////////////////////////////////////////////////////////////////////////
fn cultist(_: &mut StsRandom, last_action: Option<EnemyAction>, _: usize) -> EnemyAction {
    // TODO: check rng behavior
    if last_action.is_none() {
        EnemyAction::CultistIncantation
    } else {
        EnemyAction::CultistDarkStrike
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// SpikeSlimeM
// - 28 to 32 HP
// - Flame Tackle: Deal 8 damage, add a Slimed to the discard pile
// - Lick: Inflict 1 Frail
// - 30% Flame Tackle, 70% Lick
//  -- Cannot use Flame Tackle or Lick three times in a row
// - https://slay-the-spire.fandom.com/wiki/Spike_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
fn spike_slime_m(
    ai_rng: &mut StsRandom,
    last_action: Option<EnemyAction>,
    run_length: usize,
) -> EnemyAction {
    match ai_rng.gen_range(0..100) {
        0..30 if last_action != Some(EnemyAction::SpikeSlimeMFlameTackle) || run_length < 2 => {
            EnemyAction::SpikeSlimeMFlameTackle
        }
        0..30 => EnemyAction::SpikeSlimeMLick,
        _ if last_action != Some(EnemyAction::SpikeSlimeMLick) || run_length < 2 => {
            EnemyAction::SpikeSlimeMLick
        }
        _ => EnemyAction::SpikeSlimeMFlameTackle,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// SpikeSlimeS
// - 10 to 14 HP
// - Tackle: Deal 5 damage
// - 100% Tackle
// - https://slay-the-spire.fandom.com/wiki/Spike_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
fn spike_slime_s(ai_rng: &mut StsRandom, _: Option<EnemyAction>, _: usize) -> EnemyAction {
    let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
    EnemyAction::SpikeSlimeSTackle
}

#[cfg(test)]
mod test {
    use super::super::super::rng::Seed;

    use crate::components::EnemyStatus;
    use crate::data::Enemy;

    use super::*;

    #[test]
    fn test_acid_slime() {
        let seed: Seed = 3u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyInCombat::new(Enemy::AcidSlimeS, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy.state);
        assert_eq!(status.enemy_type, Enemy::AcidSlimeS);
        assert_eq!(status.hp, 12);
        assert_eq!(status.hp_max, 12);
        assert_eq!(status.conditions, Vec::new());
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::AcidSlimeSLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeSTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::AcidSlimeSLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeSTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::AcidSlimeSLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeSTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::AcidSlimeSLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeSTackle
        );

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyInCombat::new(Enemy::AcidSlimeM, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy.state);
        assert_eq!(status.enemy_type, Enemy::AcidSlimeM);
        assert_eq!(status.hp, 32);
        assert_eq!(status.hp_max, 32);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeMCorrosiveSpit
        );
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeMTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::AcidSlimeMLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeMCorrosiveSpit
        );
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeMTackle
        );
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeMCorrosiveSpit
        );
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::AcidSlimeMTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::AcidSlimeMLick);
    }

    #[test]
    fn test_spike_slime() {
        let seed: Seed = 8u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyInCombat::new(Enemy::SpikeSlimeS, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy.state);
        assert_eq!(status.enemy_type, Enemy::SpikeSlimeS);
        assert_eq!(status.hp, 13);
        assert_eq!(status.hp_max, 13);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeSTackle
        );
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeSTackle
        );
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeSTackle
        );

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyInCombat::new(Enemy::SpikeSlimeM, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy.state);
        assert_eq!(status.enemy_type, Enemy::SpikeSlimeM);
        assert_eq!(status.hp, 31);
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeMFlameTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeMFlameTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeMFlameTackle
        );
        assert_eq!(enemy.next_action(&mut ai_rng), EnemyAction::SpikeSlimeMLick);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            EnemyAction::SpikeSlimeMFlameTackle
        );
    }
}
