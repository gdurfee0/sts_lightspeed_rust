use std::fmt;

use crate::data::{Card, NeowBlessing, Potion, Relic};
use crate::enemy::{EnemyStatus, EnemyType};
use crate::{
    Block, ColumnIndex, Debuff, DeckIndex, Effect, EnemyIndex, Energy, Gold, HandIndex, Health, Hp,
    PotionIndex, StackCount,
};

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// one of the question type messages (`Choices` and `NestedChoices`), at which point control
/// the Simulator waits for a response on the input channel.
#[derive(Debug)]
pub enum StsMessage {
    // State updates for the main game loop, outside of an encounter or event.
    CardObtained(Card),
    CardRemoved(Card),
    Deck(Vec<Card>),
    Gold(Gold),
    Map(String),
    RelicObtained(Relic),
    Relics(Vec<Relic>),
    PotionObtained(PotionIndex, Potion),
    Potions(Vec<Option<Potion>>),

    // Encounter / combat messages
    AddToDiscardPile(Vec<Card>),
    Block(Block),
    BlockGained(Block),
    BlockLost(Block),
    CardDiscarded(HandIndex, Card),
    CardDrawn(HandIndex, Card),
    DamageBlocked(Hp),
    DamageTaken(Hp),
    Debuffs(Vec<(Debuff, StackCount)>),
    DiscardPile(Vec<Card>),
    EnemyStatus(EnemyIndex, EnemyStatus),
    EnemyDied(EnemyIndex, EnemyType),
    EnemyParty(Vec<Option<EnemyStatus>>),
    HandDiscarded,
    Health(Health),
    ShufflingDiscardToDraw,
    Energy(Energy),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
    GameOver(bool),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Prompt {
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept any and all of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    CombatAction,
    ClimbFloor,
    ClimbFloorHasPotion,
    RemoveCard,
    TargetEnemy,
}

#[derive(Clone, Debug)]
pub enum Choice {
    EndTurn,
    PotionAction(PotionAction),
    ClimbFloor(ColumnIndex),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainPotion(Potion),
    RemoveCard(DeckIndex, Card),
    PlayCardFromHand(HandIndex, Card, Vec<Effect>),
    Skip,
    TargetEnemy(EnemyIndex, EnemyType, Vec<Effect>),
}

#[derive(Clone, Copy, Debug)]
pub enum PotionAction {
    Discard(PotionIndex, Potion),
    Drink(PotionIndex, Potion),
}

#[derive(Clone, Debug)]
pub enum MainScreenOption {
    ClimbFloor(ColumnIndex),
    Potion(PotionAction),
}

// TODO: Move these to a presentation module
impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::ChooseNeow => write!(f, "Choose Neow's Blessing"),
            Prompt::ChooseNext => write!(f, "Choose the next item to obtain"),
            Prompt::ChooseOne => write!(f, "Choose an item to obtain"),
            Prompt::ClimbFloor => write!(f, "Move up into one of the following columns"),
            Prompt::ClimbFloorHasPotion => write!(
                f,
                "Move up into one of the following columns, or drink/discard a potion"
            ),
            Prompt::CombatAction => write!(f, "It is your turn to act"),
            Prompt::RemoveCard => write!(f, "Choose a card to remove"),
            Prompt::TargetEnemy => write!(f, "Choose an enemy to target"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::ClimbFloor(column_index) => {
                write!(
                    f,
                    "Climb Spire, column {}",
                    (b'a' + *column_index as u8) as char
                )
            }
            Choice::PotionAction(PotionAction::Discard(_, potion)) => {
                write!(f, "Discard potion \"{:?}\"", potion)
            }
            Choice::PotionAction(PotionAction::Drink(_, potion)) => {
                write!(f, "Drink potion \"{:?}\"", potion)
            }

            Choice::EndTurn => write!(f, "(End Turn)"),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
            Choice::ObtainCard(card) => write!(f, "{:?}", card),
            Choice::ObtainPotion(potion) => write!(f, "{:?}", potion),
            Choice::PlayCardFromHand(_, card, effects) => {
                write!(f, "Play card \"{:?}\" {:?}", card, effects)
            }
            Choice::RemoveCard(_, card) => write!(f, "{:?}", card),
            Choice::Skip => write!(f, "(Skip)"),
            Choice::TargetEnemy(_, enemy, effects) => {
                write!(f, "Target \"{:?}\" {:?}", enemy, effects)
            } // TODO: Index?
        }
    }
}
