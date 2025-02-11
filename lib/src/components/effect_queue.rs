use std::collections::VecDeque;

use crate::data::{EnemyEffect, PlayerEffect};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    FromCard(&'static PlayerEffect),
    FromEnemyPlaybook(&'static EnemyEffect),
    FromEnemyState(EnemyEffect),
    FromPlayerState(PlayerEffect),
}

pub struct EffectQueue {
    queue: VecDeque<Effect>,
}

impl EffectQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl Default for EffectQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectQueue {
    pub fn push_front(&mut self, effect: Effect) {
        self.queue.push_front(effect);
    }

    pub fn push_back(&mut self, effect: Effect) {
        self.queue.push_back(effect);
    }

    pub fn pop_front(&mut self) -> Option<Effect> {
        self.queue.pop_front()
    }
}
