use crate::data::{Card, Intent};

/// This file introduces the basic datastructures for enemy action.
///
/// An `Action` consists of a list of `Effect`s which are enacted in order.
/// An `Action` can be resolved to an `Intent` (depending on context, such as player debuffs)
/// which can be displayed to the user.
#[derive(Debug)]
pub struct Action {
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

#[derive(Clone, Copy, Debug)]
pub enum Effect {
    AddToDiscardPile(&'static [Card]),
    DealDamage(u32, u32),
    Inflict(Debuff, u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Debuff {
    Frail,
    Weak,
}

pub struct ActionChain {
    effects: Vec<Effect>,
    intent: Intent,
}

impl Action {
    pub fn deal_damage(amount: u32, times: u32) -> Action {
        Action {
            effects: vec![Effect::DealDamage(amount, times)],
            intent: Intent::Aggressive(amount, times),
        }
    }

    pub fn inflict(debuff: Debuff, count: u32) -> Action {
        Action {
            effects: vec![Effect::Inflict(debuff, count)],
            intent: Intent::StrategicDebuff,
        }
    }

    pub fn then(self) -> ActionChain {
        ActionChain {
            effects: self.effects,
            intent: self.intent,
        }
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        // Only references to &'static Actions are passed around, so this is safe and fast.
        std::ptr::eq(self, other)
    }
}

impl Eq for Action {}

impl ActionChain {
    pub fn add_to_discard_pile(mut self, cards: &'static [Card]) -> Action {
        self.effects.push(Effect::AddToDiscardPile(cards));
        Action {
            effects: self.effects,
            intent: match self.intent {
                Intent::Aggressive(amount, times) => Intent::AggressiveDebuff(amount, times),
                Intent::AggressiveBuff(_, _) => todo!(),
                Intent::AggressiveDebuff(amount, times) => Intent::AggressiveDebuff(amount, times),
                Intent::AggressiveDefensive(_, _) => todo!(),
                Intent::Cowardly => todo!(),
                Intent::Defensive => Intent::DefensiveDebuff,
                Intent::DefensiveBuff => todo!(),
                Intent::DefensiveDebuff => Intent::DefensiveDebuff,
                Intent::Sleeping => todo!(),
                Intent::StrategicBuff => todo!(),
                Intent::StrategicDebuff => todo!(),
                Intent::Stunned => todo!(),
                Intent::Unknown => Intent::Unknown,
            },
        }
    }
}
