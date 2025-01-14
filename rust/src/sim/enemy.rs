use std::ops::RangeInclusive;

use once_cell::sync::Lazy;

use crate::data::{Card, EnemyType, Intent};
use crate::rng::StsRandom;

use super::action::{Action, Debuff};

pub struct Enemy {
    enemy_type: EnemyType,
    hp: u32,
    hp_max: u32,
    next_action_fn: NextActionFn,
    next_action: &'static Action,
    run_length: u32,
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
            next_action_fn,
            next_action,
            run_length: 0,
        }
    }

    pub fn enemy_type(&self) -> EnemyType {
        self.enemy_type
    }

    pub fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }

    pub fn intent(&self) -> Intent {
        self.next_action.intent
    }

    pub fn next_action(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        let current_action = self.next_action;
        self.next_action = (self.next_action_fn)(ai_rng, Some(current_action), self.run_length);
        if self.next_action == current_action {
            self.run_length = self.run_length.saturating_add(1);
        } else {
            self.run_length = 1;
        }
        current_action
    }
}

macro_rules! define_action {
    ($name:ident, $action:expr) => {
        static $name: Lazy<Action> = Lazy::new(|| $action);
    };
}

define_action!(
    ACID_SLIME_M_CORROSIVE_SPIT,
    Action::deal_damage(7, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
);
define_action!(ACID_SLIME_M_LICK, Action::inflict(Debuff::Weak, 1));
define_action!(ACID_SLIME_M_TACKLE, Action::deal_damage(10, 1));
define_action!(ACID_SLIME_S_LICK, Action::inflict(Debuff::Weak, 1));
define_action!(ACID_SLIME_S_TACKLE, Action::deal_damage(3, 1));

define_action!(SPIKE_SLIME_M_LICK, Action::inflict(Debuff::Frail, 1));
define_action!(
    SPIKE_SLIME_M_FLAME_TACKLE,
    Action::deal_damage(8, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
);
define_action!(SPIKE_SLIME_S_TACKLE, Action::deal_damage(5, 1));

// rng, last_action, run_length
type NextActionFn = fn(&mut StsRandom, Option<&'static Action>, u32) -> &'static Action;

#[derive(Debug)]
struct AcidSlimeM;

impl AcidSlimeM {
    fn params() -> (RangeInclusive<u32>, NextActionFn) {
        (28..=32, Self::next_action as NextActionFn)
    }

    #[allow(clippy::explicit_auto_deref)]
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        run_length: u32,
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
}

#[derive(Debug)]
struct AcidSlimeS;

impl AcidSlimeS {
    fn params() -> (RangeInclusive<u32>, NextActionFn) {
        (8..=12, Self::next_action as NextActionFn)
    }

    #[allow(unused_variables)]
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        run_length: u32,
    ) -> &'static Action {
        // Does not seem to burn an extra random number except for the very first call.
        if last_action.is_none() {
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
}

#[derive(Debug)]
struct SpikeSlimeM;

impl SpikeSlimeM {
    fn params() -> (RangeInclusive<u32>, NextActionFn) {
        (28..=32, Self::next_action as NextActionFn)
    }

    #[allow(unused_variables)]
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        run_length: u32,
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
}

#[derive(Debug)]
struct SpikeSlimeS;

impl SpikeSlimeS {
    fn params() -> (RangeInclusive<u32>, NextActionFn) {
        (10..=14, Self::next_action as NextActionFn)
    }

    #[allow(unused_variables)]
    fn next_action(
        ai_rng: &mut StsRandom,
        last_action: Option<&'static Action>,
        repeat_count: u32,
    ) -> &'static Action {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        &SPIKE_SLIME_S_TACKLE
    }
}

/*
macro_rules! impl_enemy {
    ($name:ident, $hp:expr, $( $pattern:pat $( if $pred:expr )? => $expr:expr ),+ $(,)?) => {
        #[derive(Debug)]
        struct $name;

        impl $name {
            fn params() -> (RangeInclusive<u32>, NextActionFn) {
                ($hp, $name::next_action)
            }

            fn next_action(
                ai_rng: &mut StsRandom,
                past_actions: &VecDeque<&'static Action>
            ) -> &'static Action {
                let roll = ai_rng.gen_range(0..100);
                match roll {
                    $(
                        $pattern $(if $pred)? => $expr,
                    )+
                }
            }
        }
    };
}


impl_enemy!(SpikeSlimeM, 28..=32,
    0..=30 if not_thrice(&SPIKE_SLIME_M_FLAME_TACKLE, past_actions) => &SPIKE_SLIME_M_FLAME_TACKLE,
    0..=30 => &SPIKE_SLIME_M_LICK,
    _ if not_thrice(&SPIKE_SLIME_M_LICK, past_actions) => &SPIKE_SLIME_M_LICK,
    _ => &SPIKE_SLIME_M_FLAME_TACKLE
);
impl_enemy!(SpikeSlimeS, 10..=14, _ => &SPIKE_SLIME_S_TACKLE);
*/
