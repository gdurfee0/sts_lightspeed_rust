use crate::types::{Block, Hp};

pub struct DamageTaken {
    pub blocked: Block,
    pub hp_lost: Hp,
    pub provokes_thorns: bool,
}
