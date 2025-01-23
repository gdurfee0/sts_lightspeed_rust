use anyhow::Error;

use crate::data::{Character, Event};
use crate::systems::rng::StsRandom;
use crate::{Choice, Prompt};

use super::player::Player;

pub struct EventSimulator<'a> {
    character: &'static Character,
    event: Event,
    potion_rng: &'a mut StsRandom,
    player: &'a mut Player,
}

impl<'a> EventSimulator<'a> {
    pub fn new(
        character: &'static Character,
        event: Event,
        potion_rng: &'a mut StsRandom,
        player: &'a mut Player,
    ) -> Self {
        Self {
            character,
            event,
            potion_rng,
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
            Event::TheCleric => todo!("{:?}", self.event),
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

    fn the_woman_in_blue(self) -> Result<(), Error> {
        let mut choices = vec![Choice::EventChoice(20, "Buy 1 Potion for 20 Gold".into())];
        if self.player.state.gold >= 30 {
            choices.push(Choice::EventChoice(30, "Buy 2 Potions for 30 Gold".into()));
        }
        if self.player.state.gold >= 40 {
            choices.push(Choice::EventChoice(40, "Buy 3 Potions for 40 Gold".into()));
        }
        let choice = self
            .player
            .comms
            .prompt_for_choice(Prompt::ChooseForEvent, &choices)?;
        println!("{:?}", choice);
        match choice {
            Choice::EventChoice(20, _) => {
                self.player.decrease_gold(20)?;
                self.player.choose_potions_to_obtain(
                    &[*self.potion_rng.choose(self.character.potion_pool)],
                    1,
                )
            }
            Choice::EventChoice(30, _) => {
                self.player.decrease_gold(30)?;
                self.player.choose_potions_to_obtain(
                    &[
                        *self.potion_rng.choose(self.character.potion_pool),
                        *self.potion_rng.choose(self.character.potion_pool),
                    ],
                    2,
                )
            }
            Choice::EventChoice(40, _) => {
                self.player.decrease_gold(40)?;
                self.player.choose_potions_to_obtain(
                    &[
                        *self.potion_rng.choose(self.character.potion_pool),
                        *self.potion_rng.choose(self.character.potion_pool),
                        *self.potion_rng.choose(self.character.potion_pool),
                    ],
                    3,
                )
            }
            Choice::Skip => Ok(()),
            _ => unreachable!(),
        }
    }
}
