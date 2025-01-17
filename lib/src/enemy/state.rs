use crate::rng::StsRandom;
use crate::{Debuff, Effect, Hp, StackCount};

use super::action::{AcidSlimeM, AcidSlimeS, Action, NextActionFn, SpikeSlimeM, SpikeSlimeS};
use super::id::EnemyType;
use super::status::EnemyStatus;

/// The `Enemy` is the basic unit representing enemy combatants in the game. Callers will be
/// primarily interested in the class-level `Enemy::new` constructor and the instance-level
/// `Enemy::next_action` method, which advances the enemy's AI state machine according to the
/// official game mechanics, returning the enemy's next action to be performed.
#[derive(Debug)]
pub struct Enemy {
    enemy_type: EnemyType,
    hp: u32,
    hp_max: u32,
    debuffs: Vec<(Debuff, StackCount)>,
    next_action_fn: NextActionFn,
    next_action: &'static Action,
    run_length: u8,
}

// rng, last_action, run_length

impl Enemy {
    pub fn new(enemy_type: EnemyType, hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Self {
        let (health_range, next_action_fn) = match enemy_type {
            EnemyType::AcidSlimeM => AcidSlimeM::params(),
            EnemyType::AcidSlimeS => AcidSlimeS::params(),
            EnemyType::SpikeSlimeM => SpikeSlimeM::params(),
            EnemyType::SpikeSlimeS => SpikeSlimeS::params(),
            _ => todo!(),
        };
        let hp = hp_rng.gen_range(health_range);
        let hp_max = hp;
        let next_action = next_action_fn(ai_rng, None, 0);
        Self {
            enemy_type,
            hp,
            hp_max,
            next_action_fn,
            next_action,
            run_length: 1,
            debuffs: Vec::new(),
        }
    }

    pub fn enemy_type(&self) -> EnemyType {
        self.enemy_type
    }

    pub fn hp(&self) -> Hp {
        self.hp
    }

    pub fn status(&self) -> EnemyStatus {
        EnemyStatus {
            enemy_type: self.enemy_type,
            intent: self.next_action.intent,
            hp: self.hp,
            hp_max: self.hp_max,
            debuffs: self.debuffs.clone(),
        }
    }

    pub fn next_action(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        let action = self.next_action;
        self.next_action = (self.next_action_fn)(ai_rng, Some(action), self.run_length);
        if self.next_action == action {
            self.run_length = self.run_length.saturating_add(1);
        } else {
            self.run_length = 1;
        }
        action
    }

    pub fn apply_effect(&mut self, effect: &Effect) {
        match effect {
            Effect::AddToDiscardPile(_) => unreachable!(),
            Effect::AttackDamage(amount) => self.hp = self.hp.saturating_sub(*amount),
            Effect::GainBlock(_) => unreachable!(),
            Effect::Inflict(debuff, stacks) => self.apply_debuff(*debuff, *stacks),
        }
    }

    pub fn account_for_buffs_and_debuffs(&self, effect: Effect) -> Effect {
        match effect {
            Effect::AttackDamage(amount) => {
                if self
                    .debuffs
                    .iter()
                    .any(|(debuff, _)| *debuff == Debuff::Vulnerable)
                {
                    Effect::AttackDamage((amount as f32 * 1.5).floor() as u32)
                } else {
                    Effect::AttackDamage(amount)
                }
            }
            Effect::Inflict(_, _) => effect,
            _ => todo!("{:?}", effect),
        }
    }

    pub fn start_turn(&mut self) -> bool {
        for (_, stacks) in self.debuffs.iter_mut() {
            *stacks = stacks.saturating_sub(1);
        }
        self.debuffs.retain(|(_, stacks)| *stacks > 0);
        true
    }

    fn apply_debuff(&mut self, debuff: Debuff, stacks: StackCount) {
        if let Some((_, c)) = self.debuffs.iter_mut().find(|(d, _)| *d == debuff) {
            *c += stacks;
        } else {
            self.debuffs.push((debuff, stacks));
        }
    }
}
