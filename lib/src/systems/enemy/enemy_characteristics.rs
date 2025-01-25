use std::fmt;
use std::ops::RangeInclusive;

use crate::data::{Enemy, EnemyAction, EnemyCondition};
use crate::systems::rng::StsRandom;
use crate::types::Hp;

pub trait EnemyCharacteristics: fmt::Debug {
    fn powers(&mut self) -> Vec<EnemyCondition> {
        vec![]
    }

    fn first_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction;

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction;
}

macro_rules! define_enemy {
    ($enemy:ident, $hprange:expr) => {
        ($hprange, Box::new($enemy) as Box<dyn EnemyCharacteristics>)
    };
}

macro_rules! define_enemies {
    ($enemy:expr, $($enemy_ident:ident => $hprange:expr,)*) => {
        match $enemy {
            $(Enemy::$enemy_ident => define_enemy!($enemy_ident, $hprange),)*
            unfinished => panic!("Unfinished enemy: {:?}", unfinished),
        }
    }
}

pub fn characteristics_for_enemy(
    enemy: Enemy,
) -> (RangeInclusive<Hp>, Box<dyn EnemyCharacteristics>) {
    define_enemies!(
        enemy,
        AcidSlimeM => 28..=32,
        AcidSlimeS => 8..=12,
        Cultist => 48..=54,
        FungiBeast => 22..=28,
        JawWorm => 40..=44,
        SpikeSlimeM => 28..=32,
        SpikeSlimeS => 10..=14,
    )
}

///////////////////////////////////////////////////////////////////////////////////////////////////
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
#[derive(Debug)]
struct AcidSlimeM;

impl AcidSlimeM {
    fn next_action_helper(
        ai_rng: &mut StsRandom,
        last_action: Option<EnemyAction>,
        run_length: usize,
    ) -> EnemyAction {
        match ai_rng.gen_range(0..100) {
            0..30
                if last_action != Some(EnemyAction::AcidSlimeMCorrosiveSpit) || run_length < 2 =>
            {
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
}

impl EnemyCharacteristics for AcidSlimeM {
    fn first_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        Self::next_action_helper(ai_rng, None, 0)
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction {
        Self::next_action_helper(ai_rng, Some(last_action), run_length)
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
#[derive(Debug)]
struct AcidSlimeS;

impl EnemyCharacteristics for AcidSlimeS {
    fn first_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        // The game burns an extra roll on the first turn then eschews the rng thereafter.
        let _ = ai_rng.gen_range(0..100);
        if ai_rng.next_bool() {
            EnemyAction::AcidSlimeSTackle
        } else {
            EnemyAction::AcidSlimeSLick
        }
    }

    fn next_action(
        &mut self,
        _: &mut StsRandom,
        last_action: EnemyAction,
        _: usize,
    ) -> EnemyAction {
        if last_action == EnemyAction::AcidSlimeSLick {
            EnemyAction::AcidSlimeSTackle
        } else {
            EnemyAction::AcidSlimeSLick
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cultist
// - 48 to 54 HP
// - Incantation: Gain 3 Ritual (first turn only)
// - Dark Strike: Deal 6 damage (all turns after the first)
// - https://slay-the-spire.fandom.com/wiki/Cultist
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Cultist;

impl EnemyCharacteristics for Cultist {
    fn first_action(&mut self, _: &mut StsRandom) -> EnemyAction {
        EnemyAction::CultistIncantation
    }

    fn next_action(&mut self, _: &mut StsRandom, _: EnemyAction, _: usize) -> EnemyAction {
        EnemyAction::CultistDarkStrike
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fungi Beast
// - 22 to 28 HP
// - Bite: Deal 6 damage
// - Grow: Gain 3 Strength
// - Spore Cloud: On death, applies 2 Vulnerable to the player.
// - https://slay-the-spire.fandom.com/wiki/Fungi_Beast
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct FungiBeast;

impl FungiBeast {
    fn next_action_helper(
        ai_rng: &mut StsRandom,
        last_action: Option<EnemyAction>,
        run_length: usize,
    ) -> EnemyAction {
        match ai_rng.gen_range(0..100) {
            0..60 if last_action != Some(EnemyAction::FungiBeastBite) || run_length < 2 => {
                EnemyAction::FungiBeastBite
            }
            0..60 => EnemyAction::FungiBeastGrow,
            _ if last_action != Some(EnemyAction::FungiBeastGrow) => EnemyAction::FungiBeastGrow,
            _ => EnemyAction::FungiBeastBite,
        }
    }
}

impl EnemyCharacteristics for FungiBeast {
    fn powers(&mut self) -> Vec<EnemyCondition> {
        vec![EnemyCondition::SporeCloud(2)]
    }

    fn first_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        Self::next_action_helper(ai_rng, None, 0)
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction {
        Self::next_action_helper(ai_rng, Some(last_action), run_length)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Green Louse
// - 11 to 17 HP
// - Bite: Deal D damage (D between 5 and 7, chosen upon spawning)
// - Spit Web: Apply 2 Weak
// - https://slay-the-spire.fandom.com/wiki/Louses#Green_Louse
////////////////////////////////////////////////////////////////////////////////////////////////////

/*
#[derive(Debug)]
struct GreenLouse {
    bite_damage: AttackDamage,
}

impl GreenLouse {

}



fn green_louse(
    ai_rng: &mut StsRandom,
    last_action: Option<EnemyAction>,
    run_length: usize,
) -> EnemyAction {
    match ai_rng.gen_range(0..100) {
        0..25 if last_action != Some(EnemyAction::GreenLouseSpitWeb) || run_length < 2 => {
            EnemyAction::GreenLouseSpitWeb
        }
        0..25 => EnemyAction::GreenLouseBite,
        _ if last_action != Some(EnemyAction::GreenLouseBite) || run_length < 2 => {
            EnemyAction::GreenLouseBite
        }
        _ => EnemyAction::GreenLouseSpitWeb,
    }
}

#[derive(Debug)]
struct GreenLouse {
    bite_damage: AttackDamage,
}

impl EnemyCharacteristics for GreenLouse {
    fn on_spawn(&mut self, enemy_in_combat: &mut EnemyInCombat) {
        enemy_in_combat.state.next_action = EnemyAction::GreenLouseBite;
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: Option<EnemyAction>,
        run_length: usize,
    ) -> EnemyAction {
        green_louse(ai_rng, last_action, run_length)
    }
}
*/

////////////////////////////////////////////////////////////////////////////////////////////////////
// Jaw Worm
// - 40 to 44 HP
// - Chomp: Deal 11 damage
// - Thrash: Deal 7 damage and gain 5 Block
// - Bellow: Gain 3 Strength and 6 Block
// - https://slay-the-spire.fandom.com/wiki/Jaw_Worm
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct JawWorm;

impl EnemyCharacteristics for JawWorm {
    fn first_action(&mut self, _: &mut StsRandom) -> EnemyAction {
        EnemyAction::JawWormChomp
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction {
        match ai_rng.gen_range(0..100) {
            0..25 if last_action != EnemyAction::JawWormChomp => EnemyAction::JawWormChomp,
            0..25 => *ai_rng.weighted_choose(&[
                (EnemyAction::JawWormBellow, 0.5625),
                (EnemyAction::JawWormThrash, 1. - 0.5625),
            ]),
            25..55 if last_action != EnemyAction::JawWormThrash || run_length < 2 => {
                EnemyAction::JawWormThrash
            }
            25..55 => *ai_rng.weighted_choose(&[
                (EnemyAction::JawWormChomp, 0.357),
                (EnemyAction::JawWormBellow, 1. - 0.357),
            ]),
            _ if last_action != EnemyAction::JawWormBellow || run_length < 2 => {
                EnemyAction::JawWormBellow
            }
            _ => *ai_rng.weighted_choose(&[
                (EnemyAction::JawWormChomp, 0.416),
                (EnemyAction::JawWormThrash, 1. - 0.416),
            ]),
        }
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
#[derive(Debug)]
struct SpikeSlimeM;

impl SpikeSlimeM {
    fn next_action_helper(
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
}

impl EnemyCharacteristics for SpikeSlimeM {
    fn first_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        Self::next_action_helper(ai_rng, None, 0)
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction {
        Self::next_action_helper(ai_rng, Some(last_action), run_length)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// SpikeSlimeS
// - 10 to 14 HP
// - Tackle: Deal 5 damage
// - 100% Tackle
// - https://slay-the-spire.fandom.com/wiki/Spike_Slime
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct SpikeSlimeS;

impl EnemyCharacteristics for SpikeSlimeS {
    fn first_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        EnemyAction::SpikeSlimeSTackle
    }

    fn next_action(&mut self, ai_rng: &mut StsRandom, _: EnemyAction, _: usize) -> EnemyAction {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        EnemyAction::SpikeSlimeSTackle
    }
}

#[cfg(test)]
mod test {
    use super::super::super::rng::Seed;

    use crate::components::EnemyStatus;
    use crate::data::Enemy;
    use crate::systems::enemy::EnemyInCombat;

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
