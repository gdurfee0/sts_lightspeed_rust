use std::iter::repeat;
use std::ops::RangeInclusive;

use crate::data::{Card, EnemyType, Intent};
use crate::rng::StsRandom;
use crate::{Hp, HpMax, StackCount};

use super::{Debuff, Effect};

/// This file introduces the basic datastructures for enemies and enemy action.
///
/// An `EnemyMove` consists of a list of `Effect`s which are enacted in order.
/// An `EnemyMove` can be resolved to an `Intent` (depending on context, such as player debuffs)
/// which can be displayed to the user.
#[derive(Debug)]
pub struct EnemyMove {
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

// rng, last_action, run_length
type NextMoveFn = fn(&mut StsRandom, Option<&'static EnemyMove>, u8) -> &'static EnemyMove;

/// The `Enemy` is the basic unit representing enemy combatants in the game. Callers will be
/// primarily interested in the class-level `Enemy::new` constructor and the instance-level
/// `Enemy::next_move` method, which advances the enemy's AI state machine according to the
/// official game mechanics, returning the enemy's next move to be performed.
#[derive(Debug)]
pub struct Enemy {
    enemy_type: EnemyType,
    hp: u32,
    hp_max: u32,
    debuffs: Vec<(Debuff, StackCount)>,
    next_move_fn: NextMoveFn,
    next_move: &'static EnemyMove,
    run_length: u8,
}

/// `EnemyStatus` is a small bundle of information about the enemy that is made available to
/// the player. The player is not allowed to know anything else about the enemy, such as its
/// internal state or future moves.
#[derive(Debug)]
pub struct EnemyStatus {
    enemy_type: EnemyType,
    intent: Intent,
    hp: Hp,
    hp_max: HpMax,
    debuffs: Vec<(Debuff, StackCount)>,
}

impl EnemyMove {
    fn deal_damage(amount: u32, times: u32) -> EnemyMove {
        EnemyMove {
            effects: repeat(Effect::DealDamage(amount))
                .take(times as usize)
                .collect(),
            intent: Intent::Aggressive(amount, times),
        }
    }

    fn inflict(debuff: Debuff, stacks: u32) -> EnemyMove {
        EnemyMove {
            effects: vec![Effect::Inflict(debuff, stacks)],
            intent: Intent::StrategicDebuff,
        }
    }

    fn then(self) -> EnemyMoveBuilder {
        EnemyMoveBuilder {
            effects: self.effects,
            intent: self.intent,
        }
    }
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Self {
        let (health_range, next_action_fn) = match enemy_type {
            EnemyType::AcidSlimeM => AcidSlimeM::params(),
            EnemyType::AcidSlimeS => AcidSlimeS::params(),
            EnemyType::SpikeSlimeM => SpikeSlimeM::params(),
            EnemyType::SpikeSlimeS => SpikeSlimeS::params(),
            _ => todo!(),
        };
        let hp = hp_rng.gen_range(health_range);
        let hp_max = hp;
        let next_action = next_action_fn(ai_rng, None, 0);
        Self {
            enemy_type,
            hp,
            hp_max,
            next_move_fn: next_action_fn,
            next_move: next_action,
            run_length: 1,
            debuffs: Vec::new(),
        }
    }

    pub fn enemy_type(&self) -> EnemyType {
        self.enemy_type
    }

    pub fn status(&self) -> EnemyStatus {
        EnemyStatus {
            enemy_type: self.enemy_type,
            intent: self.next_move.intent,
            hp: self.hp,
            hp_max: self.hp_max,
            debuffs: self.debuffs.clone(),
        }
    }

    pub fn next_move(&mut self, ai_rng: &mut StsRandom) -> &'static EnemyMove {
        let current_move = self.next_move;
        self.next_move = (self.next_move_fn)(ai_rng, Some(current_move), self.run_length);
        if self.next_move == current_move {
            self.run_length = self.run_length.saturating_add(1);
        } else {
            self.run_length = 1;
        }
        current_move
    }

    pub fn apply_effect(&mut self, effect: Effect) -> bool {
        match effect {
            Effect::AddToDiscardPile(_) => unreachable!(),
            Effect::DealDamage(mut amount) => {
                if self
                    .debuffs
                    .iter()
                    .any(|(debuff, _)| *debuff == Debuff::Vulnerable)
                {
                    amount = (amount as f32 * 1.5).floor() as u32;
                }
                println!("Enemy dealt {} damage", amount);
                self.hp = self.hp.saturating_sub(amount);
                self.hp > 0
            }
            Effect::GainBlock(_) => unreachable!(),
            Effect::Inflict(debuff, stack_count) => self.apply_debuff(debuff, stack_count),
        }
    }

    pub fn start_turn(&mut self) -> bool {
        for (_, stacks) in self.debuffs.iter_mut() {
            *stacks = stacks.saturating_sub(1);
        }
        self.debuffs.retain(|(_, stacks)| *stacks > 0);
        true
    }

    fn apply_debuff(&mut self, debuff: Debuff, stack_count: StackCount) -> bool {
        if let Some((_, c)) = self.debuffs.iter_mut().find(|(d, _)| *d == debuff) {
            *c += stack_count;
        } else {
            self.debuffs.push((debuff, stack_count));
        }
        true
    }
}

impl PartialEq for EnemyMove {
    fn eq(&self, other: &Self) -> bool {
        // Only references to `&'static EnemyMove`s are passed around, so this is safe and fast.
        std::ptr::eq(self, other)
    }
}

impl Eq for EnemyMove {}

struct EnemyMoveBuilder {
    effects: Vec<Effect>,
    intent: Intent,
}

impl EnemyMoveBuilder {
    pub fn add_to_discard_pile(mut self, cards: &'static [Card]) -> EnemyMove {
        self.effects.push(Effect::AddToDiscardPile(cards));
        EnemyMove {
            effects: self.effects,
            intent: add_debuff_to_intent(self.intent),
        }
    }
}

fn add_debuff_to_intent(intent: Intent) -> Intent {
    match intent {
        Intent::Aggressive(amount, times) => Intent::AggressiveDebuff(amount, times),
        Intent::AggressiveBuff(_, _) => todo!(),
        Intent::AggressiveDebuff(amount, times) => Intent::AggressiveDebuff(amount, times),
        Intent::AggressiveDefensive(_, _) => todo!(),
        Intent::Cowardly => todo!(),
        Intent::Defensive => Intent::DefensiveDebuff,
        Intent::DefensiveBuff => todo!(),
        Intent::DefensiveDebuff => Intent::DefensiveDebuff,
        Intent::Sleeping => todo!(),
        Intent::StrategicBuff => todo!(),
        Intent::StrategicDebuff => todo!(),
        Intent::Stunned => todo!(),
        Intent::Unknown => Intent::Unknown,
    }
}

// Convenience macros
macro_rules! define_move {
    ($name:ident, $enemy_move:expr) => {
        static $name: once_cell::sync::Lazy<EnemyMove> = once_cell::sync::Lazy::new(|| $enemy_move);
    };
}

macro_rules! define_enemy {
    ($name:ident, $hprange:expr, $($body:tt)*) => {
        #[derive(Debug)]
        struct $name;

        impl $name {
            fn params() -> (RangeInclusive<u32>, NextMoveFn) {
                ($hprange, $name::next_move)
            }

            $($body)*
        }
    };
}

// ACID_SLIME_M
define_move!(
    ACID_SLIME_M_CORROSIVE_SPIT,
    EnemyMove::deal_damage(7, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
);
define_move!(ACID_SLIME_M_LICK, EnemyMove::inflict(Debuff::Weak, 1));
define_move!(ACID_SLIME_M_TACKLE, EnemyMove::deal_damage(10, 1));
define_enemy!(
    AcidSlimeM,
    28..=32,
    #[allow(clippy::explicit_auto_deref)]
    fn next_move(
        ai_rng: &mut StsRandom,
        last_move: Option<&'static EnemyMove>,
        run_length: u8,
    ) -> &'static EnemyMove {
        match ai_rng.gen_range(0..100) {
            0..30 if last_move != Some(&ACID_SLIME_M_CORROSIVE_SPIT) || run_length < 2 => {
                &ACID_SLIME_M_CORROSIVE_SPIT
            }
            0..30 => {
                if ai_rng.next_bool() {
                    &ACID_SLIME_M_TACKLE
                } else {
                    &ACID_SLIME_M_LICK
                }
            }
            30..70 if last_move != Some(&ACID_SLIME_M_TACKLE) => &ACID_SLIME_M_TACKLE,
            30..70 => *ai_rng.weighted_choose(&[
                (&ACID_SLIME_M_CORROSIVE_SPIT, 0.5),
                (&ACID_SLIME_M_LICK, 0.5),
            ]),
            _ if last_move != Some(&ACID_SLIME_M_LICK) || run_length < 2 => &ACID_SLIME_M_LICK,
            _ => *ai_rng.weighted_choose(&[
                (&ACID_SLIME_M_CORROSIVE_SPIT, 0.4),
                (&ACID_SLIME_M_TACKLE, 0.6),
            ]),
        }
    }
);

// ACID_SLIME_S
define_move!(ACID_SLIME_S_LICK, EnemyMove::inflict(Debuff::Weak, 1));
define_move!(ACID_SLIME_S_TACKLE, EnemyMove::deal_damage(3, 1));
define_enemy!(
    AcidSlimeS,
    8..=12,
    fn next_move(
        ai_rng: &mut StsRandom,
        last_move: Option<&'static EnemyMove>,
        _run_length: u8,
    ) -> &'static EnemyMove {
        if last_move.is_none() {
            // The game burns an extra roll on the first turn then eschews the rng thereafter.
            let _ = ai_rng.gen_range(0..100);
            if ai_rng.next_bool() {
                &ACID_SLIME_S_TACKLE
            } else {
                &ACID_SLIME_S_LICK
            }
        } else if last_move == Some(&ACID_SLIME_S_LICK) {
            &ACID_SLIME_S_TACKLE
        } else {
            &ACID_SLIME_S_LICK
        }
    }
);

// SPIKE_SLIME_M
define_move!(SPIKE_SLIME_M_LICK, EnemyMove::inflict(Debuff::Frail, 1));
define_move!(
    SPIKE_SLIME_M_FLAME_TACKLE,
    EnemyMove::deal_damage(8, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
);
define_enemy!(
    SpikeSlimeM,
    28..=32,
    fn next_move(
        ai_rng: &mut StsRandom,
        last_move: Option<&'static EnemyMove>,
        run_length: u8,
    ) -> &'static EnemyMove {
        match ai_rng.gen_range(0..100) {
            0..30 if last_move != Some(&SPIKE_SLIME_M_FLAME_TACKLE) || run_length < 2 => {
                &SPIKE_SLIME_M_FLAME_TACKLE
            }
            0..30 => &SPIKE_SLIME_M_LICK,
            _ if last_move != Some(&SPIKE_SLIME_M_LICK) || run_length < 2 => &SPIKE_SLIME_M_LICK,
            _ => &SPIKE_SLIME_M_FLAME_TACKLE,
        }
    }
);

// SPIKE_SLIME_S
define_move!(SPIKE_SLIME_S_TACKLE, EnemyMove::deal_damage(5, 1));
define_enemy!(
    SpikeSlimeS,
    10..=14,
    fn next_move(
        ai_rng: &mut StsRandom,
        _last_move: Option<&'static EnemyMove>,
        _run_length: u8,
    ) -> &'static EnemyMove {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        &SPIKE_SLIME_S_TACKLE
    }
);

#[cfg(test)]
mod test {
    use crate::rng::Seed;

    use super::*;

    #[test]
    fn test_acid_slime() {
        let seed: Seed = 3u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = Enemy::new(EnemyType::AcidSlimeS, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::AcidSlimeS);
        assert_eq!(status.hp, 12);
        assert_eq!(status.hp_max, 12);
        assert_eq!(status.debuffs, Vec::new());
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_S_TACKLE);

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = Enemy::new(EnemyType::AcidSlimeM, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::AcidSlimeM);
        assert_eq!(status.hp, 32);
        assert_eq!(status.hp_max, 32);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_CORROSIVE_SPIT);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_CORROSIVE_SPIT);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_CORROSIVE_SPIT);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*ACID_SLIME_M_LICK);
    }

    #[test]
    fn test_spike_slime() {
        let seed: Seed = 8u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = Enemy::new(EnemyType::SpikeSlimeS, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::SpikeSlimeS);
        assert_eq!(status.hp, 13);
        assert_eq!(status.hp_max, 13);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = Enemy::new(EnemyType::SpikeSlimeM, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::SpikeSlimeM);
        assert_eq!(status.hp, 31);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_move(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
    }
}
