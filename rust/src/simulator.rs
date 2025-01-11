use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Ok};

use crate::data::{Act, Ascension, Character, NeowBlessing, NeowBonus, NeowPenalty, Relic};
use crate::map::{MapBuilder, NodeGrid};
use crate::message::{Choice, PlayerView, Prompt, StsMessage};
use crate::rng::{EncounterGenerator, NeowGenerator, Seed};

pub struct StsSimulator {
    // Information typically set on the command line
    seed: Seed,
    character: &'static Character,
    ascension: Ascension,

    // Communication channels
    input_rx: Receiver<usize>,
    output_tx: Sender<StsMessage>,

    // Random number generators for various game elements
    neow_generator: NeowGenerator,
    encounter_generator: EncounterGenerator,

    // Current map layout
    map: NodeGrid,
    player_row_col: Option<(usize, usize)>,

    // Current player state
    player_hp: u32,
    player_hp_max: u32,
    player_gold: u32,
    player_relics: Vec<Relic>,
}

impl StsSimulator {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        ascension: Ascension,
        input_rx: Receiver<usize>,
        output_tx: Sender<StsMessage>,
    ) -> Self {
        let map = MapBuilder::from(&seed, ascension, Act::get(1)).build();
        let neow_generator = NeowGenerator::new(&seed);
        let encounter_generator = EncounterGenerator::new(&seed);
        Self {
            seed,
            character,
            ascension,
            input_rx,
            output_tx,
            neow_generator,
            encounter_generator,
            map,
            player_row_col: None,
            player_hp: character.start_hp,
            player_hp_max: character.start_hp,
            player_gold: 99,
            player_relics: vec![character.starting_relic],
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!(
            "[Simulator] Starting simulator of size {} with messages of size {}",
            std::mem::size_of::<StsSimulator>(),
            std::mem::size_of::<StsMessage>(),
        );
        self.send_map()?;
        self.send_relics()?;
        self.send_player_view()?;
        let mut prompt = Prompt::NeowBlessing;
        let mut choices = self
            .neow_generator
            .blessing_choices()
            .map(Choice::NeowBlessing)
            .to_vec();
        self.send_prompt_and_choices(prompt, &choices)?;
        loop {
            let choice_index = self.input_rx.recv()?;
            if let Some(choice) = choices.get(choice_index) {
                (prompt, choices) = self.handle_response(choice)?;
                self.send_player_view()?;
                self.send_prompt_and_choices(prompt, &choices)?;
            } else {
                return Err(anyhow!(
                    "[Simulator] Invalid choice index {} from client; expected 0..{}",
                    choice_index,
                    choices.len()
                ));
            }
        }
    }

    fn send_map(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(self.map.to_string()))?;
        Ok(())
    }

    fn send_relics(&self) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Relics(self.player_relics.clone()))?;
        Ok(())
    }

    fn send_player_view(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::View(PlayerView {
            hp: self.player_hp,
            hp_max: self.player_hp_max,
            gold: self.player_gold,
        }))?;
        Ok(())
    }

    fn send_prompt_and_choices(
        &self,
        prompt: Prompt,
        choices: &[Choice],
    ) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Choose(prompt, choices.to_vec()))?;
        Ok(())
    }

    fn handle_response(&mut self, choice: &Choice) -> Result<(Prompt, Vec<Choice>), anyhow::Error> {
        match choice {
            Choice::NeowBlessing(blessing) => {
                self.handle_neow_blessing(blessing)?;
                Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
            }
            Choice::CatchFire => Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire])),
        }
    }

    fn handle_neow_blessing(&mut self, blessing: &NeowBlessing) -> Result<(), anyhow::Error> {
        match blessing {
            NeowBlessing::ChooseOneOfThreeCards => todo!(),
            NeowBlessing::ChooseUncommonColorlessCard => todo!(),
            NeowBlessing::GainOneHundredGold => {
                self.player_gold += 100;
            }
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                self.player_hp_max += self.player_hp_max / 10;
                self.player_hp = self.player_hp_max;
            }
            NeowBlessing::NeowsLament => self.obtain_relic(Relic::NeowsLament)?,
            NeowBlessing::ObtainRandomCommonRelic => todo!(),
            NeowBlessing::ObtainRandomRareCard => todo!(),
            NeowBlessing::ObtainThreeRandomPotions => todo!(),
            NeowBlessing::RemoveCard => todo!(),
            NeowBlessing::ReplaceStarterRelic => todo!(),
            NeowBlessing::TransformCard => todo!(),
            NeowBlessing::UpgradeCard => todo!(),
            NeowBlessing::Composite(bonus, penalty) => {
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => {
                        self.player_gold += 250;
                    }
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        self.player_hp_max += self.player_hp_max / 5;
                        self.player_hp = self.player_hp_max;
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => {
                        self.player_hp_max -= self.player_hp_max / 10;
                        self.player_hp = self.player_hp_max;
                    }
                    NeowPenalty::LoseAllGold => {
                        self.player_gold = 0;
                    }
                    NeowPenalty::ObtainCurse => todo!(),
                    NeowPenalty::TakeDamage => {
                        self.player_hp -= (self.player_hp / 10) * 3;
                    }
                }
            }
        }
        Ok(())
    }

    fn obtain_relic(&mut self, relic: Relic) -> Result<(), anyhow::Error> {
        self.player_relics.push(relic);
        self.send_relics()?;
        match relic {
            Relic::Akabeko => todo!(),
            Relic::Anchor => todo!(),
            Relic::AncientTeaSet => todo!(),
            Relic::ArtOfWar => todo!(),
            Relic::Astrolabe => todo!(),
            Relic::BagOfMarbles => todo!(),
            Relic::BagOfPreparation => todo!(),
            Relic::BirdFacedUrn => todo!(),
            Relic::BlackBlood => todo!(),
            Relic::BlackStar => todo!(),
            Relic::BloodVial => todo!(),
            Relic::BloodyIdol => todo!(),
            Relic::BlueCandle => todo!(),
            Relic::BottledFlame => todo!(),
            Relic::BottledLightning => todo!(),
            Relic::BottledTornado => todo!(),
            Relic::Brimstone => todo!(),
            Relic::BronzeScales => todo!(),
            Relic::BurningBlood => todo!(),
            Relic::BustedCrown => todo!(),
            Relic::Calipers => todo!(),
            Relic::CallingBell => todo!(),
            Relic::CaptainsWheel => todo!(),
            Relic::Cauldron => todo!(),
            Relic::CentennialPuzzle => todo!(),
            Relic::CeramicFish => todo!(),
            Relic::ChampionBelt => todo!(),
            Relic::CharonsAshes => todo!(),
            Relic::ChemicalX => todo!(),
            Relic::Circlet => todo!(),
            Relic::CloakClasp => todo!(),
            Relic::ClockworkSouvenir => todo!(),
            Relic::CoffeeDripper => todo!(),
            Relic::CrackedCore => todo!(),
            Relic::CultistHeadpiece => todo!(),
            Relic::CursedKey => todo!(),
            Relic::Damaru => todo!(),
            Relic::DarkstonePeriapt => todo!(),
            Relic::DataDisk => todo!(),
            Relic::DeadBranch => todo!(),
            Relic::DollysMirror => todo!(),
            Relic::DreamCatcher => todo!(),
            Relic::DuVuDoll => todo!(),
            Relic::Duality => todo!(),
            Relic::Ectoplasm => todo!(),
            Relic::EmotionChip => todo!(),
            Relic::EmptyCage => todo!(),
            Relic::Enchiridion => todo!(),
            Relic::EternalFeather => todo!(),
            Relic::FaceOfCleric => todo!(),
            Relic::FossilizedHelix => todo!(),
            Relic::FrozenCore => todo!(),
            Relic::FrozenEgg => todo!(),
            Relic::FrozenEye => todo!(),
            Relic::FusionHammer => todo!(),
            Relic::GamblingChip => todo!(),
            Relic::Ginger => todo!(),
            Relic::Girya => todo!(),
            Relic::GoldPlatedCables => todo!(),
            Relic::GoldenEye => todo!(),
            Relic::GoldenIdol => todo!(),
            Relic::GremlinHorn => todo!(),
            Relic::GremlinVisage => todo!(),
            Relic::HandDrill => todo!(),
            Relic::HappyFlower => todo!(),
            Relic::HolyWater => todo!(),
            Relic::HornCleat => todo!(),
            Relic::HoveringKite => todo!(),
            Relic::IceCream => todo!(),
            Relic::IncenseBurner => todo!(),
            Relic::InkBottle => todo!(),
            Relic::Inserter => todo!(),
            Relic::JuzuBracelet => todo!(),
            Relic::Kunai => todo!(),
            Relic::Lantern => todo!(),
            Relic::LeesWaffle => todo!(),
            Relic::LetterOpener => todo!(),
            Relic::LizardTail => todo!(),
            Relic::MagicFlower => todo!(),
            Relic::Mango => todo!(),
            Relic::MarkOfPain => todo!(),
            Relic::MarkOfTheBloom => todo!(),
            Relic::Matryoshka => todo!(),
            Relic::MawBank => todo!(),
            Relic::MealTicket => todo!(),
            Relic::MeatOnTheBone => todo!(),
            Relic::MedicalKit => todo!(),
            Relic::Melange => todo!(),
            Relic::MembershipCard => todo!(),
            Relic::MercuryHourglass => todo!(),
            Relic::MoltenEgg => todo!(),
            Relic::MummifiedHand => todo!(),
            Relic::MutagenicStrength => todo!(),
            Relic::NeowsLament => todo!(),
            Relic::NlothsGift => todo!(),
            Relic::NlothsHungryFace => todo!(),
            Relic::Necronomicon => todo!(),
            Relic::NilrysCodex => todo!(),
            Relic::NinjaScroll => todo!(),
            Relic::NuclearBattery => todo!(),
            Relic::Nunchaku => todo!(),
            Relic::OddMushroom => todo!(),
            Relic::OddlySmoothStone => todo!(),
            Relic::OldCoin => todo!(),
            Relic::Omamori => todo!(),
            Relic::OrangePellets => todo!(),
            Relic::Orichalcum => todo!(),
            Relic::OrnamentalFan => todo!(),
            Relic::Orrery => todo!(),
            Relic::PandorasBox => todo!(),
            Relic::Pantograph => todo!(),
            Relic::PaperKrane => todo!(),
            Relic::PaperPhrog => todo!(),
            Relic::PeacePipe => todo!(),
            Relic::Pear => todo!(),
            Relic::PenNib => todo!(),
            Relic::PhilosophersStone => todo!(),
            Relic::Pocketwatch => todo!(),
            Relic::PotionBelt => todo!(),
            Relic::PrayerWheel => todo!(),
            Relic::PreservedInsect => todo!(),
            Relic::PrismaticShard => todo!(),
            Relic::PureWater => todo!(),
            Relic::QuestionCard => todo!(),
            Relic::RedMask => todo!(),
            Relic::RedSkull => todo!(),
            Relic::RegalPillow => todo!(),
            Relic::RingOfTheSerpent => todo!(),
            Relic::RingOfTheSnake => todo!(),
            Relic::RunicCapacitor => todo!(),
            Relic::RunicCube => todo!(),
            Relic::RunicDome => todo!(),
            Relic::RunicPyramid => todo!(),
            Relic::SacredBark => todo!(),
            Relic::SelfFormingClay => todo!(),
            Relic::Shovel => todo!(),
            Relic::Shuriken => todo!(),
            Relic::SingingBowl => todo!(),
            Relic::SlaversCollar => todo!(),
            Relic::SlingOfCourage => todo!(),
            Relic::SmilingMask => todo!(),
            Relic::SneckoEye => todo!(),
            Relic::SneckoSkull => todo!(),
            Relic::Sozu => todo!(),
            Relic::SpiritPoop => todo!(),
            Relic::SsserpentHead => todo!(),
            Relic::StoneCalendar => todo!(),
            Relic::StrangeSpoon => todo!(),
            Relic::Strawberry => todo!(),
            Relic::StrikeDummy => todo!(),
            Relic::Sundial => todo!(),
            Relic::SymbioticVirus => todo!(),
            Relic::TeardropLocket => todo!(),
            Relic::TheAbacus => todo!(),
            Relic::TheBoot => todo!(),
            Relic::TheCourier => todo!(),
            Relic::TheSpecimen => todo!(),
            Relic::ThreadAndNeedle => todo!(),
            Relic::Tingsha => todo!(),
            Relic::TinyChest => todo!(),
            Relic::TinyHouse => todo!(),
            Relic::Toolbox => todo!(),
            Relic::Torii => todo!(),
            Relic::ToughBandages => todo!(),
            Relic::ToxicEgg => todo!(),
            Relic::ToyOrnithopter => todo!(),
            Relic::TungstenRod => todo!(),
            Relic::Turnip => todo!(),
            Relic::TwistedFunnel => todo!(),
            Relic::UnceasingTop => todo!(),
            Relic::Vajra => todo!(),
            Relic::VelvetChoker => todo!(),
            Relic::VioletLotus => todo!(),
            Relic::WarPaint => todo!(),
            Relic::WarpedTongs => todo!(),
            Relic::Whetstone => todo!(),
            Relic::WhiteBeastStatue => todo!(),
            Relic::WingBoots => todo!(),
            Relic::WristBlade => todo!(),
        }
    }
}
