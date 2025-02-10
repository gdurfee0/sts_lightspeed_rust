use crate::types::Dexterity;

pub trait DefenderStatus {
    fn dexterity(&self) -> Dexterity;
    fn is_frail(&self) -> bool;
    fn is_vulnerable(&self) -> bool;
}
