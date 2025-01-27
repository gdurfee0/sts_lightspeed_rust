// Source: Slay the Spire wiki:
// - https://slay-the-spire.fandom.com/wiki/Category:Monster
// - https://slay-the-spire.fandom.com/wiki/Category:Elite
// - https://slay-the-spire.fandom.com/wiki/Category:Boss_Monster

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::types::AttackDamage;

use super::card::Card;
use super::condition::{EnemyCondition, PlayerCondition};
use super::effect::EnemyEffect;
use super::intent::Intent;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnemyAction {
    AcidSlimeMCorrosiveSpit,
    AcidSlimeMLick,
    AcidSlimeMTackle,
    AcidSlimeSLick,
    AcidSlimeSTackle,
    CultistDarkStrike,
    CultistIncantation,
    FungiBeastBite,
    FungiBeastGrow,
    GreenLouseBite(AttackDamage),
    GreenLouseSpitWeb,
    JawWormBellow,
    JawWormChomp,
    JawWormThrash,
    SpikeSlimeMFlameTackle,
    SpikeSlimeMLick,
    SpikeSlimeSTackle,
}

impl EnemyAction {
    pub fn effect_chain(&self) -> &[EnemyEffect] {
        EnemyActionDetails::for_action(*self)
            .effect_chain
            .as_slice()
    }

    pub fn intent(&self) -> Intent {
        EnemyActionDetails::for_action(*self).intent
    }
}

#[derive(Clone, Debug)]
pub struct EnemyActionDetails {
    pub effect_chain: Vec<EnemyEffect>,
    pub intent: Intent,
}

pub struct EnemyActionDetailsBuilder {
    effect_chain: Vec<EnemyEffect>,
}

impl EnemyActionDetails {
    pub fn for_action(action: EnemyAction) -> &'static Self {
        ALL_ENEMY_ACTIONS.get(&action).unwrap_or_else(|| {
            panic!("No action details found for {:?}", action);
        })
    }
}

impl EnemyActionDetailsBuilder {
    fn new() -> Self {
        Self {
            effect_chain: vec![],
        }
    }

    fn push(mut self, effect: EnemyEffect) -> Self {
        self.effect_chain.push(effect);
        self
    }

    fn build(self) -> EnemyActionDetails {
        EnemyActionDetails {
            intent: Intent::from(self.effect_chain.as_slice()),
            effect_chain: self.effect_chain,
        }
    }
}

macro_rules! define_action {
    ($variant:ident => [$($effect:ident($($param:expr),*)),*]) => {
        (
            EnemyAction::$variant,
            EnemyActionDetailsBuilder::new()$(.push(EnemyEffect::$effect($($param),*)))*.build()
        )
    };
}

macro_rules! define_actions {
    ($($variant:ident => $e:tt,)*) => {
        Lazy::new(
            || vec![$(define_action!($variant => $e)),*].into_iter().collect::<HashMap<_, _>>()
        )
    }
}

static ALL_ENEMY_ACTIONS: Lazy<HashMap<EnemyAction, EnemyActionDetails>> = define_actions!(
    AcidSlimeMCorrosiveSpit => [DealDamage(7), AddToDiscardPile(&[Card::Slimed(false)])],
    AcidSlimeMLick => [Apply(PlayerCondition::Weak(1))],
    AcidSlimeMTackle => [DealDamage(10)],
    AcidSlimeSLick => [Apply(PlayerCondition::Weak(1))],
    AcidSlimeSTackle => [DealDamage(3)],
    CultistDarkStrike => [DealDamage(6)],
    CultistIncantation => [ApplyToSelf(EnemyCondition::Ritual(3, true))],
    FungiBeastBite => [DealDamage(6)],
    FungiBeastGrow => [GainStrength(3)],
    JawWormBellow => [GainStrength(3), GainBlock(6)],
    JawWormChomp => [DealDamage(11)],
    JawWormThrash => [DealDamage(7), GainBlock(5)],
    SpikeSlimeMFlameTackle => [DealDamage(8), AddToDiscardPile(&[Card::Slimed(false)])],
    SpikeSlimeMLick => [Apply(PlayerCondition::Frail(1))],
    SpikeSlimeSTackle => [DealDamage(5)],
);
