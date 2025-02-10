use super::choice::Choice;
use super::notification::Notification;
use super::prompt::Prompt;

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number `Notification`s to the client, concluding with a `Choices`
/// message, at which point control the Simulator waits for a response on the input channel.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum StsMessage {
    Notification(Notification),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
    GameOver(bool),
}
