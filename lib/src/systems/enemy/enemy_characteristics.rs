use std::fmt;

use crate::data::{Enemy, EnemyAction, EnemyCondition};
use crate::systems::rng::StsRandom;
use crate::types::{Hp, HpMax, StackCount};

pub trait EnemyCharacteristics: fmt::Debug {
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction);
    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction;
}

/// Generates a new enemy characteristics object for the specified enemy type.
pub fn gen_characteristics(enemy: Enemy, hp_rng: &mut StsRandom) -> Box<dyn EnemyCharacteristics> {
    match enemy {
        Enemy::AcidSlimeM => Box::new(AcidSlimeM::new(hp_rng)),
        Enemy::AcidSlimeS => Box::new(AcidSlimeS::new(hp_rng)),
        Enemy::Cultist => Box::new(Cultist::new(hp_rng)),
        Enemy::FungiBeast => Box::new(FungiBeast::new(hp_rng)),
        Enemy::GreenLouse => Box::new(GreenLouse::new(hp_rng)),
        Enemy::GremlinNob => Box::new(GremlinNob::new(hp_rng)),
        Enemy::JawWorm => Box::new(JawWorm::new(hp_rng)),
        Enemy::SpikeSlimeM => Box::new(SpikeSlimeM::new(hp_rng)),
        Enemy::SpikeSlimeS => Box::new(SpikeSlimeS::new(hp_rng)),
        unavailable => todo!("Unavailable enemy: {:?}", unavailable),
    }
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
struct AcidSlimeM {
    hp_max: HpMax,
}

impl AcidSlimeM {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(28..=32),
        }
    }

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
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        (
            self.hp_max,
            vec![],
            Self::next_action_helper(ai_rng, None, 0),
        )
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
struct AcidSlimeS {
    hp_max: HpMax,
}

impl AcidSlimeS {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(8..=12),
        }
    }
}

impl EnemyCharacteristics for AcidSlimeS {
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        let _ = ai_rng.gen_range(0..100);
        let first_action = if ai_rng.next_bool() {
            EnemyAction::AcidSlimeSTackle
        } else {
            EnemyAction::AcidSlimeSLick
        };
        (self.hp_max, vec![], first_action)
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
struct Cultist {
    hp_max: HpMax,
}

impl Cultist {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(48..=54),
        }
    }
}

impl EnemyCharacteristics for Cultist {
    fn on_spawn(&self, _: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        (self.hp_max, vec![], EnemyAction::CultistIncantation)
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
struct FungiBeast {
    hp_max: HpMax,
}

impl FungiBeast {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(22..=28),
        }
    }

    fn next_action_helper(
        ai_rng: &mut StsRandom,
        last_action: Option<EnemyAction>,
        run_length: usize,
    ) -> EnemyAction {
        let result = match ai_rng.gen_range(0..100) {
            0..60 if last_action != Some(EnemyAction::FungiBeastBite) || run_length < 2 => {
                EnemyAction::FungiBeastBite
            }
            0..60 => EnemyAction::FungiBeastGrow,
            _ if last_action != Some(EnemyAction::FungiBeastGrow) => EnemyAction::FungiBeastGrow,
            _ => EnemyAction::FungiBeastBite,
        };
        println!(
            "FungiBeast ai_rng {} next_action: {:?}",
            ai_rng.get_counter(),
            result
        );
        result
    }
}

impl EnemyCharacteristics for FungiBeast {
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        let first_action = Self::next_action_helper(ai_rng, None, 0);
        (
            self.hp_max,
            vec![EnemyCondition::SporeCloud(2)],
            first_action,
        )
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
// - Spawns with 3-7 Curl Up
// - https://slay-the-spire.fandom.com/wiki/Louses#Green_Louse
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct GreenLouse {
    hp_max: HpMax,
    bite_damage: Hp,
    curl_up_stacks: StackCount,
}

impl GreenLouse {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(11..=17),
            bite_damage: hp_rng.gen_range(5..=7),
            curl_up_stacks: hp_rng.gen_range(3..=7),
        }
    }

    fn next_action_helper(
        &self,
        ai_rng: &mut StsRandom,
        last_action: Option<EnemyAction>,
        run_length: usize,
    ) -> EnemyAction {
        match ai_rng.gen_range(0..100) {
            0..25 if last_action != Some(EnemyAction::GreenLouseSpitWeb) || run_length < 2 => {
                EnemyAction::GreenLouseSpitWeb
            }
            0..25 => EnemyAction::GreenLouseBite(self.bite_damage),
            _ if matches!(last_action, Some(EnemyAction::GreenLouseBite(_))) || run_length < 2 => {
                EnemyAction::GreenLouseBite(self.bite_damage)
            }
            _ => EnemyAction::GreenLouseSpitWeb,
        }
    }
}

impl EnemyCharacteristics for GreenLouse {
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        let first_action = self.next_action_helper(ai_rng, None, 0);
        (
            self.hp_max,
            vec![EnemyCondition::CurlUp(self.curl_up_stacks)],
            first_action,
        )
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction {
        self.next_action_helper(ai_rng, Some(last_action), run_length)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Gremlin Nob
// - 82 to 86 HP
// - Bellow: Gains 2 Enrage
// - Rush: Deal 14 damage
// - Skull Bash: Deal 6 damage and apply 2 Vulnerable
// - Always starts with Bellow
// - 33% Skull Bash, 67% Rush. Cannot use Rush three times in a row.
// - https://slay-the-spire.fandom.com/wiki/Gremlin_Nob
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct GremlinNob {
    hp_max: HpMax,
}

impl GremlinNob {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(82..=86),
        }
    }
}

impl EnemyCharacteristics for GremlinNob {
    fn on_spawn(&self, _: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        (self.hp_max, vec![], EnemyAction::GremlinNobBellow)
    }

    fn next_action(
        &mut self,
        ai_rng: &mut StsRandom,
        last_action: EnemyAction,
        run_length: usize,
    ) -> EnemyAction {
        match ai_rng.gen_range(0..100) {
            0..33 => EnemyAction::GremlinNobSkullBash,
            _ if last_action != EnemyAction::GremlinNobRush || run_length < 2 => {
                EnemyAction::GremlinNobRush
            }
            _ => EnemyAction::GremlinNobSkullBash,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Jaw Worm
// - 40 to 44 HP
// - Chomp: Deal 11 damage
// - Thrash: Deal 7 damage and gain 5 Block
// - Bellow: Gain 3 Strength and 6 Block
// - https://slay-the-spire.fandom.com/wiki/Jaw_Worm
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct JawWorm {
    hp_max: HpMax,
}

impl JawWorm {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(40..=44),
        }
    }
}

impl EnemyCharacteristics for JawWorm {
    fn on_spawn(&self, _: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        (self.hp_max, vec![], EnemyAction::JawWormChomp)
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
struct SpikeSlimeM {
    hp_max: HpMax,
}

impl SpikeSlimeM {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(28..=32),
        }
    }

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
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        (
            self.hp_max,
            vec![],
            Self::next_action_helper(ai_rng, None, 0),
        )
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
struct SpikeSlimeS {
    hp_max: HpMax,
}

impl SpikeSlimeS {
    fn new(hp_rng: &mut StsRandom) -> Self {
        Self {
            hp_max: hp_rng.gen_range(10..=14),
        }
    }
}

impl EnemyCharacteristics for SpikeSlimeS {
    fn on_spawn(&self, ai_rng: &mut StsRandom) -> (HpMax, Vec<EnemyCondition>, EnemyAction) {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        (self.hp_max, vec![], EnemyAction::SpikeSlimeSTackle)
    }

    fn next_action(&mut self, ai_rng: &mut StsRandom, _: EnemyAction, _: usize) -> EnemyAction {
        let _ = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        EnemyAction::SpikeSlimeSTackle
    }
}
