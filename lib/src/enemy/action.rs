use std::iter::repeat;

use crate::{rng::StsRandom, AttackCount, AttackDamage, Card, Debuff, Effect, StackCount};

use super::intent::Intent;

pub type NextActionFn = fn(&mut StsRandom, Option<&'static Action>, u8) -> &'static Action;

/// An enemy `Action` consists of a list of `Effect`s which are enacted in order.
/// An `Action` can be resolved to an `Intent` (depending on context, such as player debuffs)
/// which can be displayed to the user.
#[derive(Debug)]
pub struct Action {
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

impl Action {
    fn deal_damage(amount: AttackDamage, times: AttackCount) -> Action {
        Action {
            effects: repeat(Effect::AttackDamage(amount))
                .take(times as usize)
                .collect(),
            intent: Intent::Aggressive(amount, times),
        }
    }

    fn inflict(debuff: Debuff, stacks: StackCount) -> Action {
        Action {
            effects: vec![Effect::Inflict(debuff, stacks)],
            intent: Intent::StrategicDebuff,
        }
    }

    fn then(self) -> ActionBuilder {
        ActionBuilder {
            effects: self.effects,
            intent: self.intent,
        }
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        // Only references to `&'static Action`s are passed around, so this is safe and fast.
        std::ptr::eq(self, other)
    }
}

impl Eq for Action {}

struct ActionBuilder {
    effects: Vec<Effect>,
    intent: Intent,
}

impl ActionBuilder {
    pub fn add_to_discard_pile(mut self, cards: &'static [Card]) -> Action {
        self.effects.push(Effect::AddToDiscardPile(cards));
        Action {
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
macro_rules! define_action {
    ($name:ident, $action:expr) => {
        static $name: once_cell::sync::Lazy<Action> = once_cell::sync::Lazy::new(|| $action);
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
    Action::deal_damage(7, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
);
define_action!(ACID_SLIME_M_LICK, Action::inflict(Debuff::Weak, 1));
define_action!(ACID_SLIME_M_TACKLE, Action::deal_damage(10, 1));
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
define_action!(ACID_SLIME_S_LICK, Action::inflict(Debuff::Weak, 1));
define_action!(ACID_SLIME_S_TACKLE, Action::deal_damage(3, 1));
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
    Action::deal_damage(8, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
);
define_action!(SPIKE_SLIME_M_LICK, Action::inflict(Debuff::Frail, 1));
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
define_action!(SPIKE_SLIME_S_TACKLE, Action::deal_damage(5, 1));
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
    use crate::enemy::{Enemy, EnemyType};
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
        let mut enemy = Enemy::new(EnemyType::AcidSlimeM, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::AcidSlimeM);
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
        let mut enemy = Enemy::new(EnemyType::SpikeSlimeS, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::SpikeSlimeS);
        assert_eq!(status.hp, 13);
        assert_eq!(status.hp_max, 13);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);
        assert_eq!(enemy.next_action(&mut ai_rng), &*SPIKE_SLIME_S_TACKLE);

        let mut hp_rng = StsRandom::from(seed.with_offset(1));
        let mut ai_rng = StsRandom::from(seed.with_offset(1));
        let mut enemy = Enemy::new(EnemyType::SpikeSlimeM, &mut hp_rng, &mut ai_rng);
        let status = enemy.status();
        assert_eq!(status.enemy_type, EnemyType::SpikeSlimeM);
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
