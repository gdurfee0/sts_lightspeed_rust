use crate::types::{Block, Strength};

pub trait AttackerStatus {
    fn block(&self) -> Block;
    fn draw_pile_size(&self) -> usize;
    fn hand_size(&self) -> usize;
    fn is_weak(&self) -> bool;
    fn number_of_strike_cards_owned(&self) -> usize;
    fn strength(&self) -> Strength;
}
