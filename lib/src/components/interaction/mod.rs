use anyhow::Error;

mod choice;
mod message;
mod notification;
mod prompt;

pub use choice::{Choice, PotionAction};
pub use message::StsMessage;
pub use notification::Notification;
pub use prompt::Prompt;

pub trait Interaction {
    fn prompt_for_choice<'a>(
        &self,
        prompt: Prompt,
        choices: &'a [Choice],
    ) -> Result<&'a Choice, Error>;
    fn send_game_over(&self, victorious: bool) -> Result<(), Error>;
    fn send_notification(&self, notification: Notification) -> Result<(), Error>;
}
