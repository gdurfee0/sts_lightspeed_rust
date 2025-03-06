use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::components::{Choice, Interaction, Notification, Prompt, StsMessage};

/// Handles all interactions with the player via the from_client and to_client channels, sending
/// messages to the player to prompt for decisions and returning the choices made by the player.
#[derive(Debug)]
pub struct PlayerInteraction {
    from_client: Receiver<usize>,
    to_client: Sender<StsMessage>,
}

impl PlayerInteraction {
    pub fn new(from_client: Receiver<usize>, to_client: Sender<StsMessage>) -> Self {
        Self {
            from_client,
            to_client,
        }
    }
}

impl Interaction for PlayerInteraction {
    /// Sends the supplied notification to the user.
    fn send_notification(&self, notification: Notification) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::Notification(notification))?;
        Ok(())
    }

    /// Sends the game over message to the user.
    fn send_game_over(&self, victorious: bool) -> Result<(), Error> {
        self.to_client.send(StsMessage::GameOver(victorious))?;
        Ok(())
    }

    /// Internal helper function to prompt the user to choose one of the supplied choices.
    /// Annoyingly repeats the prompt until the user makes a valid choice.
    fn prompt_for_choice<'a>(
        &self,
        prompt: Prompt,
        choices: &'a [Choice],
    ) -> Result<&'a Choice, Error> {
        loop {
            self.to_client
                .send(StsMessage::Choices(prompt.clone(), choices.to_vec()))?;
            let choice_index = self.from_client.recv()?;
            if let Some(choice) = choices.get(choice_index) {
                return Ok(choice);
            }
        }
    }
}
