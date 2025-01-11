use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error, Ok};

use super::message::{Choice, PlayerView, Prompt, StsMessage};

use crate::data::{
    Act, Ascension, Card, Character, NeowBlessing, NeowBonus, NeowPenalty, Relic,
    UNCOMMON_COLORLESS_CARDS,
};
use crate::map::{MapBuilder, Node, NodeGrid, Room};
use crate::rng::{EncounterGenerator, NeowGenerator, Seed, StsRandom};

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
    card_sts_random: StsRandom,

    // Current map layout
    map: NodeGrid,

    // Current player state
    player_hp: u32,
    player_hp_max: u32,
    player_gold: u32,
    player_relics: Vec<Relic>,
    player_deck: Vec<Card>,
    player_row_col: Option<(usize, usize)>,
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
        let neow_generator = NeowGenerator::new(&seed, character);
        let encounter_generator = EncounterGenerator::new(&seed);
        let card_sts_random = StsRandom::from(&seed);
        Self {
            seed,
            character,
            ascension,
            input_rx,
            output_tx,
            neow_generator,
            encounter_generator,
            card_sts_random,
            map,
            player_hp: character.starting_hp,
            player_hp_max: character.starting_hp,
            player_gold: 99,
            player_relics: vec![character.starting_relic],
            player_deck: character.starting_deck.to_vec(),
            player_row_col: None,
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
        self.send_deck()?;
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
            } else {
                eprintln!(
                    "[Simulator] Invalid choice index {} from client; expected 0..{}",
                    choice_index,
                    choices.len()
                );
            }
            self.send_prompt_and_choices(prompt, &choices)?;
        }
    }

    fn send_map(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(
            self.map
                .to_string_with_highlighted_row_col(self.player_row_col)
                + "\n\n a  b  c  d  e  f  g",
        ))?;
        Ok(())
    }

    fn send_relics(&self) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Relics(self.player_relics.clone()))?;
        Ok(())
    }

    fn send_deck(&self) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Deck(self.player_deck.clone()))?;
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

    fn handle_response(&mut self, choice: &Choice) -> Result<(Prompt, Vec<Choice>), Error> {
        match choice {
            Choice::MapEntryColumn(col) => {
                self.player_row_col = Some((0, *col));
                self.send_map()?;
                let room = self
                    .map
                    .get(0, *col)
                    .expect("We offered an invalid column")
                    .room;
                self.enter_room(room)
            }
            Choice::NeowBlessing(blessing) => self.handle_neow_blessing(blessing),
            Choice::ObtainCard(card) => {
                self.player_deck.push(*card);
                self.send_deck()?;
                // Pick up where we left off.
                self.continue_game()
            }
            Choice::CatchFire => Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire])),
        }
    }

    fn continue_game(&mut self) -> Result<(Prompt, Vec<Choice>), Error> {
        if let Some((row, col)) = self.player_row_col {
            Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
        } else {
            // Player needs to enter the map.
            self.send_map()?;
            Ok((
                Prompt::EnterMap,
                self.map
                    .nonempty_cols_for_row(0)
                    .into_iter()
                    .map(Choice::MapEntryColumn)
                    .collect(),
            ))
        }
    }

    fn enter_room(&mut self, room: Room) -> Result<(Prompt, Vec<Choice>), Error> {
        println!("[Simulator] Player entered room {:?}", room);
        Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
    }

    fn handle_neow_blessing(
        &mut self,
        blessing: &NeowBlessing,
    ) -> Result<(Prompt, Vec<Choice>), Error> {
        match blessing {
            NeowBlessing::ChooseOneOfThreeCards => {
                return Ok((
                    Prompt::ObtainCard,
                    self.neow_generator
                        .three_card_choices()
                        .into_iter()
                        .map(Choice::ObtainCard)
                        .collect(),
                ));
            }
            NeowBlessing::ChooseUncommonColorlessCard => {
                return Ok((
                    Prompt::ObtainCard,
                    self.card_sts_random
                        .sample_without_replacement(UNCOMMON_COLORLESS_CARDS, 3)
                        .into_iter()
                        .map(Choice::ObtainCard)
                        .collect(),
                ));
            }
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
        Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
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
