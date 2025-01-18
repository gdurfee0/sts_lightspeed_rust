use crate::data::debuff::Debuff;
use crate::data::enemy::EnemyType;
use crate::rng::StsRandom;
use crate::{AttackDamage, Block, Hp, StackCount};

use super::action::{enemy_params, Action, NextActionFn};
use super::status::EnemyStatus;

/// The `EnemyState` is the basic unit representing enemy combatants in the game.
#[derive(Debug)]
pub struct EnemyState {
    enemy_type: EnemyType,
    hp: u32,
    hp_max: u32,
    block: Block,
    debuffs: Vec<(Debuff, StackCount)>,
    next_action_fn: NextActionFn,
    next_action: &'static Action,
    run_length: u8,
}

impl EnemyState {
    pub fn new(enemy_type: EnemyType, hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Self {
        let (health_range, next_action_fn) = enemy_params(enemy_type);
        let hp = hp_rng.gen_range(health_range);
        let hp_max = hp;
        let next_action = next_action_fn(ai_rng, None, 0);
        Self {
            enemy_type,
            hp,
            hp_max,
            block: 0,
            debuffs: Vec::new(),
            next_action_fn,
            next_action,
            run_length: 1,
        }
    }

    pub fn hp(&self) -> Hp {
        self.hp
    }

    pub fn status(&self) -> EnemyStatus {
        EnemyStatus {
            enemy_type: self.enemy_type,
            hp: self.hp,
            hp_max: self.hp_max,
            block: self.block,
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

    pub fn start_turn(&mut self) -> bool {
        // TODO: Should this go at the end of the enemy's turn?
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

    fn take_damage(&mut self, amount: AttackDamage) -> (Block, AttackDamage) {
        let block = self.block;
        let remaining_damage = amount.saturating_sub(block);
        self.block = self.block.saturating_sub(amount);
        self.hp = self.hp.saturating_sub(remaining_damage);
        (block, remaining_damage)
    }
}
