use anyhow::anyhow;

use super::card::Card;
use super::relic::Relic;

#[derive(Debug, PartialEq)]
pub struct Character {
    /// The character's starting max hit points.
    pub starting_hp: u32,

    /// The character's starting relic.
    pub starting_relic: Relic,

    /// The character's starting deck in the order displayed in-game.
    pub starting_deck: &'static [Card],

    /// The character's common cards, available from shops, encounters, etc. Ordered appropriately
    /// for fidelity with the game's rng.
    pub common_card_pool: &'static [Card],

    /// Ordered appropriately for fidelity with the game's rng.
    pub uncommon_card_pool: &'static [Card],

    /// Rewards if the rng is feeling really nice. Ordered appropriately for fidelity with the
    /// game's rng.
    pub rare_card_pool: &'static [Card],
}

impl TryFrom<&str> for &'static Character {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().chars().next() {
            Some('i') => Ok(&CHARACTERS[0]),
            Some('s') => Ok(&CHARACTERS[1]),
            Some('d') => Ok(&CHARACTERS[2]),
            Some('w') => Ok(&CHARACTERS[3]),
            _ => Err(anyhow!(
                "Character options are (I)ronclad, (S)ilent, (D)efect, and (W)atcher"
            )),
        }
    }
}

static CHARACTERS: &[Character] = &[
    // Ironclad
    Character {
        starting_hp: 80,
        starting_relic: Relic::BurningBlood,
        starting_deck: &[
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Bash,
        ],
        /*
        foo: [

            CardId::WHEEL_KICK,
            CardId::SIMMERING_FURY,
            CardId::FORESIGHT,
            CardId::SANCTITY,
            CardId::TALK_TO_THE_HAND,
            CardId::BATTLE_HYMN,
            CardId::INDIGNATION,
            CardId::WINDMILL_STRIKE,
            CardId::FOREIGN_INFLUENCE,
            CardId::LIKE_WATER,
            CardId::FASTING,
            CardId::CARVE_REALITY,
            CardId::WALLOP,
            CardId::WREATH_OF_FLAME,
            CardId::COLLECT,
            CardId::INNER_PEACE,
            CardId::RUSHDOWN,
            CardId::DECEIVE_REALITY,
            CardId::MENTAL_FORTRESS,
            CardId::REACH_HEAVEN,
            CardId::FEAR_NO_EVIL,
            CardId::SANDS_OF_TIME,
            CardId::WAVE_OF_THE_HAND,
            CardId::STUDY,
            CardId::MEDITATE,
            CardId::PERSEVERANCE,
            CardId::SWIVEL,
            CardId::WORSHIP,
            CardId::CONCLUDE,
            CardId::TANTRUM,
            CardId::NIRVANA,
            CardId::EMPTY_MIND,
            CardId::WEAVE,
            CardId::SIGNATURE_MOVE,
            CardId::PRAY,

            CardId::DEUS_EX_MACHINA,
            CardId::DEVA_FORM,
            CardId::SPIRIT_SHIELD,
            CardId::ESTABLISHMENT,
            CardId::OMNISCIENCE,
            CardId::WISH,
            CardId::ALPHA,
            CardId::VAULT,
            CardId::SCRAWL,
            CardId::LESSON_LEARNED,
            CardId::RAGNAROK,
            CardId::BLASPHEMY,
            CardId::DEVOTION,
            CardId::BRILLIANCE,
            CardId::MASTER_REALITY,
            CardId::CONJURE_BLADE,
            CardId::JUDGMENT,
        ],
        */
        common_card_pool: &[
            Card::Anger,
            Card::Cleave,
            Card::Warcry,
            Card::Flex,
            Card::IronWave,
            Card::BodySlam,
            Card::TrueGrit,
            Card::ShrugItOff,
            Card::Clash,
            Card::Thunderclap,
            Card::PommelStrike,
            Card::TwinStrike,
            Card::Clothesline,
            Card::Armaments,
            Card::Havoc,
            Card::Headbutt,
            Card::WildStrike,
            Card::HeavyBlade,
            Card::PerfectedStrike,
            Card::SwordBoomerang,
        ],
        uncommon_card_pool: &[
            Card::SpotWeakness,
            Card::Inflame,
            Card::PowerThrough,
            Card::DualWield,
            Card::InfernalBlade,
            Card::RecklessCharge,
            Card::Hemokinesis,
            Card::Intimidate,
            Card::BloodForBlood,
            Card::FlameBarrier,
            Card::Pummel,
            Card::BurningPact,
            Card::Metallicize,
            Card::Shockwave,
            Card::Rampage,
            Card::SeverSoul,
            Card::Whirlwind,
            Card::Combust,
            Card::DarkEmbrace,
            Card::SeeingRed,
            Card::Disarm,
            Card::FeelNoPain,
            Card::Rage,
            Card::Entrench,
            Card::Sentinel,
            Card::BattleTrance,
            Card::SearingBlow,
            Card::SecondWind,
            Card::Rupture,
            Card::Bloodletting,
            Card::Carnage,
            Card::Dropkick,
            Card::FireBreathing,
            Card::GhostlyArmor,
            Card::Uppercut,
            Card::Evolve,
        ],
        rare_card_pool: &[
            Card::Immolate,
            Card::Offering,
            Card::Exhume,
            Card::Reaper,
            Card::Brutality,
            Card::Juggernaut,
            Card::Impervious,
            Card::Berserk,
            Card::FiendFire,
            Card::Barricade,
            Card::Corruption,
            Card::LimitBreak,
            Card::Feed,
            Card::Bludgeon,
            Card::DemonForm,
            Card::DoubleTap,
        ],
    },
    // Silent
    Character {
        starting_hp: 70,
        starting_relic: Relic::RingOfTheSnake,
        starting_deck: &[
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Survivor,
            Card::Neutralize,
        ],
        common_card_pool: &[
            Card::CloakAndDagger,
            Card::SneakyStrike,
            Card::DeadlyPoison,
            Card::DaggerSpray,
            Card::Bane,
            Card::BladeDance,
            Card::Deflect,
            Card::DaggerThrow,
            Card::PoisonedStab,
            Card::Acrobatics,
            Card::QuickSlash,
            Card::Slice,
            Card::Backflip,
            Card::Outmaneuver,
            Card::Prepared,
            Card::PiercingWail,
            Card::SuckerPunch,
            Card::DodgeAndRoll,
            Card::FlyingKnee,
        ],
        uncommon_card_pool: &[
            Card::CripplingCloud,
            Card::LegSweep,
            Card::Catalyst,
            Card::Tactician,
            Card::Expertise,
            Card::Choke,
            Card::Caltrops,
            Card::Blur,
            Card::Setup,
            Card::EndlessAgony,
            Card::RiddleWithHoles,
            Card::Skewer,
            Card::CalculatedGamble,
            Card::EscapePlan,
            Card::Finisher,
            Card::WellLaidPlans,
            Card::Terror,
            Card::HeelHook,
            Card::NoxiousFumes,
            Card::InfiniteBlades,
            Card::Reflex,
            Card::Eviscerate,
            Card::Dash,
            Card::Backstab,
            Card::BouncingFlask,
            Card::Concentrate,
            Card::Flechettes,
            Card::MasterfulStab,
            Card::Accuracy,
            Card::Footwork,
            Card::Distraction,
            Card::AllOutAttack,
            Card::Predator,
        ],
        rare_card_pool: &[
            Card::GrandFinale,
            Card::AThousandCuts,
            Card::GlassKnife,
            Card::StormOfSteel,
            Card::BulletTime,
            Card::AfterImage,
            Card::Unload,
            Card::Nightmare,
            Card::ToolsOfTheTrade,
            Card::WraithForm,
            Card::Burst,
            Card::Doppelganger,
            Card::Envenom,
            Card::Adrenaline,
            Card::DieDieDie,
            Card::PhantasmalKiller,
            Card::Malaise,
            Card::CorpseExplosion,
            Card::Alchemize,
        ],
    },
    // Defect
    Character {
        starting_hp: 75,
        starting_relic: Relic::CrackedCore,
        starting_deck: &[
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Zap,
            Card::Dualcast,
        ],
        common_card_pool: &[
            Card::GoForTheEyes,
            Card::BallLightning,
            Card::Streamline,
            Card::Recursion,
            Card::CompileDriver,
            Card::Barrage,
            Card::Stack,
            Card::Rebound,
            Card::Claw,
            Card::Coolheaded,
            Card::Turbo,
            Card::SweepingBeam,
            Card::ChargeBattery,
            Card::Hologram,
            Card::BeamCell,
            Card::Leap,
            Card::ColdSnap,
            Card::SteamBarrier,
        ],
        uncommon_card_pool: &[
            Card::Storm,
            Card::GeneticAlgorithm,
            Card::Overclock,
            Card::HelloWorld,
            Card::Sunder,
            Card::Glacier,
            Card::Consume,
            Card::Fusion,
            Card::Aggregate,
            Card::Blizzard,
            Card::Chaos,
            Card::Melter,
            Card::SelfRepair,
            Card::Loop,
            Card::Chill,
            Card::BootSequence,
            Card::StaticDischarge,
            Card::Heatsinks,
            Card::Tempest,
            Card::Equilibrium,
            Card::ForceField,
            Card::Ftl,
            Card::RipAndTear,
            Card::Darkness,
            Card::DoubleEnergy,
            Card::ReinforcedBody,
            Card::AutoShields,
            Card::Reprogram,
            Card::Bullseye,
            Card::Scrape,
            Card::Recycle,
            Card::Skim,
            Card::WhiteNoise,
            Card::Capacitor,
            Card::Defragment,
            Card::DoomAndGloom,
        ],
        rare_card_pool: &[
            Card::CoreSurge,
            Card::Fission,
            Card::CreativeAi,
            Card::Amplify,
            Card::Reboot,
            Card::AllForOne,
            Card::EchoForm,
            Card::MeteorStrike,
            Card::Seek,
            Card::Rainbow,
            Card::Buffer,
            Card::Electrodynamics,
            Card::MachineLearning,
            Card::BiasedCognition,
            Card::ThunderStrike,
            Card::Hyperbeam,
            Card::MultiCast,
        ],
    },
    // Watcher
    Character {
        starting_hp: 72,
        starting_relic: Relic::PureWater,
        starting_deck: &[
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Eruption,
            Card::Vigilance,
        ],
        common_card_pool: &[
            Card::Consecrate,
            Card::BowlingBash,
            Card::FlyingSleeves,
            Card::Halt,
            Card::JustLucky,
            Card::FlurryOfBlows,
            Card::Protect,
            Card::ThirdEye,
            Card::Crescendo,
            Card::Tranquility,
            Card::EmptyBody,
            Card::SashWhip,
            Card::CutThroughFate,
            Card::FollowUp,
            Card::PressurePoints,
            Card::CrushJoints,
            Card::Evaluate,
            Card::Prostrate,
            Card::EmptyFist,
        ],
        uncommon_card_pool: &[
            Card::WheelKick,
            Card::SimmeringFury,
            Card::Foresight,
            Card::Sanctity,
            Card::TalkToTheHand,
            Card::BattleHymn,
            Card::Indignation,
            Card::WindmillStrike,
            Card::ForeignInfluence,
            Card::LikeWater,
            Card::Fasting,
            Card::CarveReality,
            Card::Wallop,
            Card::WreathOfFlame,
            Card::Collect,
            Card::InnerPeace,
            Card::Rushdown,
            Card::DeceiveRealiry,
            Card::MentalFortress,
            Card::ReachHeaven,
            Card::FearNoEvil,
            Card::SandsOfTime,
            Card::WaveOfTheHand,
            Card::Study,
            Card::Meditate,
            Card::Perserverance,
            Card::Swivel,
            Card::Worship,
            Card::Conclude,
            Card::Tantrum,
            Card::Nirvana,
            Card::EmptyMind,
            Card::Weave,
            Card::SignatureMove,
            Card::Pray,
        ],
        rare_card_pool: &[
            Card::DeusExMachina,
            Card::DevaForm,
            Card::SpiritShield,
            Card::Establishment,
            Card::Omniscience,
            Card::Wish,
            Card::Alpha,
            Card::Vault,
            Card::Scrawl,
            Card::LessonLearned,
            Card::Ragnarok,
            Card::Blasphemy,
            Card::Devotion,
            Card::Brilliance,
            Card::MasterReality,
            Card::ConjureBlade,
            Card::Judgment,
        ],
    },
];

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::data::card::{
        CURSE_CARDS, RARE_COLORLESS_CARDS, SPECIAL_COLORLESS_CARDS, STATUS_CARDS,
        UNCOMMON_COLORLESS_CARDS,
    };

    use super::*;

    #[test]
    fn test_character_try_from() {
        assert_eq!(
            <&'static Character>::try_from("Ironclad").unwrap(),
            &CHARACTERS[0]
        );
        assert_eq!(
            <&'static Character>::try_from("Silent").unwrap(),
            &CHARACTERS[1]
        );
        assert_eq!(
            <&'static Character>::try_from("Defect").unwrap(),
            &CHARACTERS[2]
        );
        assert_eq!(
            <&'static Character>::try_from("Watcher").unwrap(),
            &CHARACTERS[3]
        );
        assert_eq!(
            <&'static Character>::try_from("watcher").unwrap(),
            &CHARACTERS[3]
        );
        assert!(<&'static Character>::try_from("Unknown").is_err());
        assert!(<&'static Character>::try_from("").is_err());
    }

    #[test]
    fn test_no_duplicates() {
        let mut all_cards = CHARACTERS
            .iter()
            .flat_map(|character| {
                character
                    .common_card_pool
                    .iter()
                    .chain(character.starting_deck.iter())
                    .filter(|card| ![Card::Strike, Card::Defend].contains(card))
                    .chain(character.uncommon_card_pool.iter())
                    .chain(character.rare_card_pool.iter())
            })
            .chain(UNCOMMON_COLORLESS_CARDS.iter())
            .chain(RARE_COLORLESS_CARDS.iter())
            .chain(SPECIAL_COLORLESS_CARDS.iter())
            .chain(STATUS_CARDS.iter())
            .chain(CURSE_CARDS.iter())
            .copied()
            .collect::<Vec<_>>();
        all_cards.sort();
        let initial_cards = all_cards.clone();
        all_cards.dedup();
        assert_eq!(all_cards, initial_cards);
        assert_eq!(all_cards.len(), 354);
    }
}
