use anyhow::Error;

use crate::data::Event;
use crate::systems::player::Player;
use crate::systems::rng::PotionGenerator;
use crate::{Choice, Prompt};

pub struct EventSimulator<'a> {
    event: Event,
    potion_generator: &'a mut PotionGenerator,
    player: &'a mut Player,
}

impl<'a> EventSimulator<'a> {
    pub fn new(
        event: Event,
        potion_generator: &'a mut PotionGenerator,
        player: &'a mut Player,
    ) -> Self {
        Self {
            event,
            potion_generator,
            player,
        }
    }

    pub fn run(self) -> Result<(), Error> {
        match self.event {
            Event::AncientWriting => todo!("{:?}", self.event),
            Event::ANoteForYourself => todo!("{:?}", self.event),
            Event::Augmenter => todo!("{:?}", self.event),
            Event::BigFish => todo!("{:?}", self.event),
            Event::BonfireSpirits => todo!("{:?}", self.event),
            Event::CouncilOfGhosts => todo!("{:?}", self.event),
            Event::CursedTome => todo!("{:?}", self.event),
            Event::DeadAdventurer => todo!("{:?}", self.event),
            Event::DesignerInSpire => todo!("{:?}", self.event),
            Event::Duplicator => todo!("{:?}", self.event),
            Event::FaceTrader => todo!("{:?}", self.event),
            Event::Falling => todo!("{:?}", self.event),
            Event::ForgottenAltar => todo!("{:?}", self.event),
            Event::GoldenIdol => todo!("{:?}", self.event),
            Event::GoldenShrine => todo!("{:?}", self.event),
            Event::HypnotizingColoredMushrooms => todo!("{:?}", self.event),
            Event::KnowingSkull => todo!("{:?}", self.event),
            Event::Lab => todo!("{:?}", self.event),
            Event::LivingWall => todo!("{:?}", self.event),
            Event::MaskedBandits => todo!("{:?}", self.event),
            Event::MatchAndKeep => todo!("{:?}", self.event),
            Event::MindBloom => todo!("{:?}", self.event),
            Event::MysteriousSphere => todo!("{:?}", self.event),
            Event::Neow => todo!("{:?}", self.event),
            Event::Nloth => todo!("{:?}", self.event),
            Event::NoteForYourself => todo!("{:?}", self.event),
            Event::OldBeggar => todo!("{:?}", self.event),
            Event::OminousForge => todo!("{:?}", self.event),
            Event::PleadingVagrant => todo!("{:?}", self.event),
            Event::Purifier => todo!("{:?}", self.event),
            Event::ScrapOoze => todo!("{:?}", self.event),
            Event::SecretPortal => todo!("{:?}", self.event),
            Event::SensoryStone => todo!("{:?}", self.event),
            Event::ShiningLight => todo!("{:?}", self.event),
            Event::TheCleric => self.the_cleric(),
            Event::TheColosseum => todo!("{:?}", self.event),
            Event::TheDivineFountain => todo!("{:?}", self.event),
            Event::TheJoust => todo!("{:?}", self.event),
            Event::TheLibrary => todo!("{:?}", self.event),
            Event::TheMausoleum => todo!("{:?}", self.event),
            Event::TheMoaiHead => todo!("{:?}", self.event),
            Event::TheNest => todo!("{:?}", self.event),
            Event::TheSsssserpent => todo!("{:?}", self.event),
            Event::TheWomanInBlue => self.the_woman_in_blue(),
            Event::TombOfLordRedMask => todo!("{:?}", self.event),
            Event::Transmogrifier => todo!("{:?}", self.event),
            Event::UpgradeShrine => todo!("{:?}", self.event),
            Event::Vampires => todo!("{:?}", self.event),
            Event::WeMeetAgain => todo!("{:?}", self.event),
            Event::WheelOfChange => todo!("{:?}", self.event),
            Event::WindingHalls => todo!("{:?}", self.event),
            Event::WingStatue => todo!("{:?}", self.event),
            Event::WorldOfGoop => todo!("{:?}", self.event),
        }
    }

    fn the_cleric(self) -> Result<(), Error> {
        let mut choices = vec![Choice::EventChoice(35, "35 Gold: Heal 20 HP".into())];
        if self.player.state.gold >= 50 {
            choices.push(Choice::EventChoice(
                50,
                "50 Gold: Remove a card from your deck.".into(),
            ));
        }
        choices.push(Choice::Skip);
        let choice = self
            .player
            .comms
            .prompt_for_choice(Prompt::ChooseForEvent, &choices)?;
        println!("{:?}", choice);
        match choice {
            Choice::EventChoice(35, _) => {
                self.player.decrease_gold(35)?;
                self.player.increase_hp(20)
            }
            Choice::EventChoice(50, _) => {
                self.player.decrease_gold(50)?;
                self.player.choose_card_to_remove()
            }
            Choice::Skip => Ok(()),
            _ => unreachable!(),
        }
    }

    fn the_woman_in_blue(self) -> Result<(), Error> {
        let mut choices = vec![Choice::EventChoice(20, "Buy 1 Potion for 20 Gold".into())];
        if self.player.state.gold >= 30 {
            choices.push(Choice::EventChoice(30, "Buy 2 Potions for 30 Gold".into()));
        }
        if self.player.state.gold >= 40 {
            choices.push(Choice::EventChoice(40, "Buy 3 Potions for 40 Gold".into()));
        }
        choices.push(Choice::Skip);
        let choice = self
            .player
            .comms
            .prompt_for_choice(Prompt::ChooseForEvent, &choices)?;
        println!("{:?}", choice);
        let (gold, count) = match choice {
            Choice::EventChoice(20, _) => (20, 1),
            Choice::EventChoice(30, _) => (30, 2),
            Choice::EventChoice(40, _) => (40, 3),
            Choice::Skip => return Ok(()),
            _ => unreachable!(),
        };
        self.player.decrease_gold(gold)?;
        self.player
            .choose_potions_to_obtain(&self.potion_generator.gen_potions(count), count)
    }
}
