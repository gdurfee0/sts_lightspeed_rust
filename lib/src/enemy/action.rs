use std::ops::RangeInclusive;

use crate::data::{Card, Enemy, EnemyCondition, EnemyEffect, Intent, PlayerCondition};
use crate::rng::StsRandom;
use crate::types::Hp;

/// An enemy `Action` consists of a list of `Effect`s which are enacted in order.
#[derive(Debug)]
pub struct Action {
    pub effect_chain: Vec<EnemyEffect>,
    pub intent: Intent,
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        // Only references to `&'static Action`s are passed around, so this is safe and fast.
        std::ptr::eq(self, other)
    }
}

impl Eq for Action {}

struct ActionBuilder {
    pub effect_chain: Vec<EnemyEffect>,
}

impl ActionBuilder {
    pub fn new() -> Self {
        Self {
            effect_chain: Vec::new(),
        }
    }

    pub fn push(mut self, effect: EnemyEffect) -> Self {
        self.effect_chain.push(effect);
        self
    }

    pub fn build(self) -> Action {
        let intent = self.effect_chain.as_slice().into();
        Action {
            effect_chain: self.effect_chain,
            intent,
        }
    }
}

pub fn enemy_params(enemy_type: Enemy) -> (RangeInclusive<Hp>, NextActionFn) {
    match enemy_type {
        Enemy::AcidSlimeM => AcidSlimeM::params(),
        Enemy::AcidSlimeS => AcidSlimeS::params(),
        Enemy::Cultist => Cultist::params(),
        Enemy::SpikeSlimeM => SpikeSlimeM::params(),
        Enemy::SpikeSlimeS => SpikeSlimeS::params(),
        _ => todo!(),
    }
}

// Convenience macros
macro_rules! define_action {
    ($name:ident, [$($eff:ident($($param:expr),*)),*]) => {
        static $name: once_cell::sync::Lazy<Action> = once_cell::sync::Lazy::new(
            || ActionBuilder::new()
                $(.push(EnemyEffect::$eff($($param),*)))*.build()
        );
    };
}

macro_rules! define_enemy {
    ($name:ident, $hprange:expr, $($body:tt)*) => {
        #[derive(Debug)]
        pub struct $name;

        impl $name {
            pub fn params() -> (std::ops::RangeInclusive<u32>, NextActionFn) {
                ($hprange, $name::next_action)
            }

            $($body)*
        }
    };
}

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
define_action!(
    ACID_SLIME_M_CORROSIVE_SPIT,
    [DealDamage(7), AddToDiscardPile(&[Card::Slimed])]
);
define_action!(ACID_SLIME_M_LICK, [Apply(PlayerCondition::Weak(1))]);
define_action!(ACID_SLIME_M_TACKLE, [DealDamage(10)]);
define_enemy!(
    AcidSlimeM,
    28..=32,
    #[allow(clippy::explicit_auto_deref)]
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        run_length: u8,
    ) -> &'static Action {
        match ai_rng.gen_range(0..100) {
            0..30 if last_action != Some(&ACID_SLIME_M_CORROSIVE_SPIT) || run_length < 2 => {
                &ACID_SLIME_M_CORROSIVE_SPIT
            }
            0..30 => {
                if ai_rng.next_bool() {
                    &ACID_SLIME_M_TACKLE
                } else {
                    &ACID_SLIME_M_LICK
                }
            }
            30..70 if last_action != Some(&ACID_SLIME_M_TACKLE) => &ACID_SLIME_M_TACKLE,
            30..70 => *ai_rng.weighted_choose(&[
                (&ACID_SLIME_M_CORROSIVE_SPIT, 0.5),
                (&ACID_SLIME_M_LICK, 0.5),
            ]),
            _ if last_action != Some(&ACID_SLIME_M_LICK) || run_length < 2 => &ACID_SLIME_M_LICK,
            _ => *ai_rng.weighted_choose(&[
                (&ACID_SLIME_M_CORROSIVE_SPIT, 0.4),
                (&ACID_SLIME_M_TACKLE, 0.6),
            ]),
        }
    }
);

////////////////////////////////////////////////////////////////////////////////////////////////////
// AcidSlimeS
// - 8 to 12 HP
// - Lick: Inflict 1 Weak
// - Tackle: Deal 3 damage
// - 50% Lick, 50% Tackle for initial action; alternates attacks thereafter
// - https://slay-the-spire.fandom.com/wiki/Acid_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
define_action!(ACID_SLIME_S_LICK, [Apply(PlayerCondition::Weak(1))]);
define_action!(ACID_SLIME_S_TACKLE, [DealDamage(3)]);
define_enemy!(
    AcidSlimeS,
    8..=12,
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        _run_length: u8,
    ) -> &'static Action {
        if last_action.is_none() {
            // The game burns an extra roll on the first turn then eschews the rng thereafter.
            let _ = ai_rng.gen_range(0..100);
            if ai_rng.next_bool() {
                &ACID_SLIME_S_TACKLE
            } else {
                &ACID_SLIME_S_LICK
            }
        } else if last_action == Some(&ACID_SLIME_S_LICK) {
            &ACID_SLIME_S_TACKLE
        } else {
            &ACID_SLIME_S_LICK
        }
    }
);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cultist
// - 48 to 52 HP
// - Incantation: Gain 3 Ritual (first turn only)
// - Dark Strike: Deal 6 damage (all turns after the first)
////////////////////////////////////////////////////////////////////////////////////////////////////
define_action!(
    CULTIST_INCANTATION,
    [ApplyToSelf(EnemyCondition::Ritual(3, true))]
);
define_action!(CULTIST_DARK_STRIKE, [DealDamage(6)]);
define_enemy!(
    Cultist,
    48..=54,
    fn next_action(
        _ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        _run_length: u8,
    ) -> &'static Action {
        // TODO: check rng behavior
        if last_action.is_none() {
            &CULTIST_INCANTATION
        } else {
            &CULTIST_DARK_STRIKE
        }
    }
);

////////////////////////////////////////////////////////////////////////////////////////////////////
// SpikeSlimeM
// - 28 to 32 HP
// - Flame Tackle: Deal 8 damage, add a Slimed to the discard pile
// - Lick: Inflict 1 Frail
// - 30% Flame Tackle, 70% Lick
//  -- Cannot use Flame Tackle or Lick three times in a row
// - https://slay-the-spire.fandom.com/wiki/Spike_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
define_action!(
    SPIKE_SLIME_M_FLAME_TACKLE,
    [DealDamage(8), AddToDiscardPile(&[Card::Slimed])]
);
define_action!(SPIKE_SLIME_M_LICK, [Apply(PlayerCondition::Frail(1))]);
define_enemy!(
    SpikeSlimeM,
    28..=32,
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        run_length: u8,
    ) -> &'static Action {
        match ai_rng.gen_range(0..100) {
            0..30 if last_action != Some(&SPIKE_SLIME_M_FLAME_TACKLE) || run_length < 2 => {
                &SPIKE_SLIME_M_FLAME_TACKLE
            }
            0..30 => &SPIKE_SLIME_M_LICK,
            _ if last_action != Some(&SPIKE_SLIME_M_LICK) || run_length < 2 => &SPIKE_SLIME_M_LICK,
            _ => &SPIKE_SLIME_M_FLAME_TACKLE,
        }
    }
);

////////////////////////////////////////////////////////////////////////////////////////////////////
// SpikeSlimeS
// - 10 to 14 HP
// - Tackle: Deal 5 damage
// - 100% Tackle
// - https://slay-the-spire.fandom.com/wiki/Spike_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
define_action!(SPIKE_SLIME_S_TACKLE, [DealDamage(5)]);
define_enemy!(
    SpikeSlimeS,
    10..=14,
    fn next_action(
        ai_rng: &mut StsRandom,
        _last_action: Option<&'static Action>,
        _run_length: u8,
    ) -> &'static Action {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        &SPIKE_SLIME_S_TACKLE
    }
);

#[cfg(test)]
mod test {
    use super::super::state::EnemyState;
    use super::super::status::EnemyStatus;

    use crate::data::Enemy;
    use crate::rng::Seed;

    use super::*;

    #[test]
    fn test_acid_slime() {
        let seed: Seed = 3u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyState::new(Enemy::AcidSlimeS, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::AcidSlimeS);
        assert_eq!(status.hp, 12);
        assert_eq!(status.hp_max, 12);
        assert_eq!(status.conditions, Vec::new());
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_S_TACKLE);

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyState::new(Enemy::AcidSlimeM, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::AcidSlimeM);
        assert_eq!(status.hp, 32);
        assert_eq!(status.hp_max, 32);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            &*ACID_SLIME_M_CORROSIVE_SPIT
        );
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_M_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_M_LICK);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            &*ACID_SLIME_M_CORROSIVE_SPIT
        );
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_M_TACKLE);
        assert_eq!(
            enemy.next_action(&mut ai_rng),
            &*ACID_SLIME_M_CORROSIVE_SPIT
        );
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_M_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*ACID_SLIME_M_LICK);
    }

    #[test]
    fn test_spike_slime() {
        let seed: Seed = 8u64.into();
        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyState::new(Enemy::SpikeSlimeS, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::SpikeSlimeS);
        assert_eq!(status.hp, 13);
        assert_eq!(status.hp_max, 13);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = EnemyState::new(Enemy::SpikeSlimeM, &mut hp_rng, &mut ai_rng);
        let status = EnemyStatus::from(&enemy);
        assert_eq!(status.enemy_type, Enemy::SpikeSlimeM);
        assert_eq!(status.hp, 31);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_LICK);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_M_FLAME_TACKLE);
    }
}
