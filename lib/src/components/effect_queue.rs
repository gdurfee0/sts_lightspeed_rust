use std::collections::VecDeque;

use crate::data::{EnemyEffect, PlayerEffect};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    Card(&'static PlayerEffect),
    EnemyPlaybook(&'static EnemyEffect),
    EnemyState(EnemyEffect),
    PlayerState(PlayerEffect),
}

#[derive(Debug)]
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

impl Drop for EffectQueue {
    fn drop(&mut self) {
        if !self.queue.is_empty() {
            eprintln!(
                "EffectQueue dropped with {} effects remaining",
                self.queue.len()
            );
        }
    }
}
