use anyhow::Error;

use crate::components::{Choice, Interaction, PlayerPersistentState, Prompt};
use crate::data::Event;
use crate::systems::rng::PotionGenerator;

pub struct EventSimulator<'a, I: Interaction> {
    comms: &'a I,
    gold_system: &'a mut GoldSystem,
    health_system: &'a mut HealthSystem,
    potion_generator: &'a mut PotionGenerator,
}

impl<'a, I: Interaction> EventSimulator<'a, I> {
    pub fn new(
        comms: &'a I,
        gold_system: &'a mut GoldSystem,
        health_system: &'a mut HealthSystem,
        potion_generator: &'a mut PotionGenerator,
    ) -> Self {
        Self {
            comms,
            gold_system,
            health_system,
            potion_generator,
        }
    }

    pub fn simulate_event(
        self,
        event: Event,
        player_persistent_state: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        match event {
            Event::AncientWriting => todo!("{:?}", event),
            Event::ANoteForYourself => todo!("{:?}", event),
            Event::Augmenter => todo!("{:?}", event),
            Event::BigFish => todo!("{:?}", event),
            Event::BonfireSpirits => todo!("{:?}", event),
            Event::CouncilOfGhosts => todo!("{:?}", event),
            Event::CursedTome => todo!("{:?}", event),
            Event::DeadAdventurer => todo!("{:?}", event),
            Event::DesignerInSpire => todo!("{:?}", event),
            Event::Duplicator => todo!("{:?}", event),
            Event::FaceTrader => todo!("{:?}", event),
            Event::Falling => todo!("{:?}", event),
            Event::ForgottenAltar => todo!("{:?}", event),
            Event::GoldenIdol => todo!("{:?}", event),
            Event::GoldenShrine => todo!("{:?}", event),
            Event::HypnotizingColoredMushrooms => todo!("{:?}", event),
            Event::KnowingSkull => todo!("{:?}", event),
            Event::Lab => todo!("{:?}", event),
            Event::LivingWall => todo!("{:?}", event),
            Event::MaskedBandits => todo!("{:?}", event),
            Event::MatchAndKeep => todo!("{:?}", event),
            Event::MindBloom => todo!("{:?}", event),
            Event::MysteriousSphere => todo!("{:?}", event),
            Event::Neow => todo!("{:?}", event),
            Event::Nloth => todo!("{:?}", event),
            Event::NoteForYourself => todo!("{:?}", event),
            Event::OldBeggar => todo!("{:?}", event),
            Event::OminousForge => todo!("{:?}", event),
            Event::PleadingVagrant => todo!("{:?}", event),
            Event::Purifier => todo!("{:?}", event),
            Event::ScrapOoze => todo!("{:?}", event),
            Event::SecretPortal => todo!("{:?}", event),
            Event::SensoryStone => todo!("{:?}", event),
            Event::ShiningLight => todo!("{:?}", event),
            Event::TheCleric => self.the_cleric(player_persistent_state),
            Event::TheColosseum => todo!("{:?}", event),
            Event::TheDivineFountain => todo!("{:?}", event),
            Event::TheJoust => todo!("{:?}", event),
            Event::TheLibrary => todo!("{:?}", event),
            Event::TheMausoleum => todo!("{:?}", event),
            Event::TheMoaiHead => todo!("{:?}", event),
            Event::TheNest => todo!("{:?}", event),
            Event::TheSsssserpent => todo!("{:?}", event),
            Event::TheWomanInBlue => Self::the_woman_in_blue(),
            Event::TombOfLordRedMask => todo!("{:?}", event),
            Event::Transmogrifier => todo!("{:?}", event),
            Event::UpgradeShrine => todo!("{:?}", event),
            Event::Vampires => todo!("{:?}", event),
            Event::WeMeetAgain => todo!("{:?}", event),
            Event::WheelOfChange => todo!("{:?}", event),
            Event::WindingHalls => todo!("{:?}", event),
            Event::WingStatue => todo!("{:?}", event),
            Event::WorldOfGoop => todo!("{:?}", event),
        }
    }

    fn the_cleric(self, player_persistent_state: &mut PlayerPersistentState) -> Result<(), Error> {
        let mut choices = vec![Choice::EventChoice(35, "35 Gold: Heal 20 HP".into())];
        if player_persistent_state.gold >= 50 {
            choices.push(Choice::EventChoice(
                50,
                "50 Gold: Remove a card from your deck.".into(),
            ));
        }
        choices.push(Choice::Skip);
        let choice = self
            .comms
            .prompt_for_choice(Prompt::ChooseForEvent, &choices)?;
        println!("{:?}", choice);
        match choice {
            Choice::EventChoice(35, _) => {
                player_persistent_state.decrease_gold(35)?;
                player_persistent_state.increase_hp(20)
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
