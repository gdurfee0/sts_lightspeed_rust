// Source: Slay the Spire wiki:
// - https://slay-the-spire.fandom.com/wiki/Category:Monster
// - https://slay-the-spire.fandom.com/wiki/Category:Elite
// - https://slay-the-spire.fandom.com/wiki/Category:Boss_Monster

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::types::Hp;

use super::card::Card;
use super::condition::{EnemyCondition, PlayerCondition};
use super::damage::Damage;
use super::effect::{
    CardDestination, CardPool, CardSelection, CostModifier, EnemyEffect, Resource,
};
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
    GreenLouseBite(Hp),
    GreenLouseSpitWeb,
    GremlinNobBellow,
    GremlinNobRush,
    GremlinNobSkullBash,
    JawWormBellow,
    JawWormChomp,
    JawWormThrash,
    SpikeSlimeMFlameTackle,
    SpikeSlimeMLick,
    SpikeSlimeSTackle,
}

impl EnemyAction {
    pub fn effect_chain(&self) -> &'static [EnemyEffect] {
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
    ($variant:ident => [$($effect:ident $args:tt),*]) => {
        (
            EnemyAction::$variant,
            EnemyActionDetailsBuilder::new()$(.push(EnemyEffect::$effect $args))*.build()
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
    AcidSlimeMCorrosiveSpit => [
        Deal(Damage::Blockable(7)),
        CreateCards(
            CardPool::Fixed(&[Card::Slimed]),
            CardSelection::All,
            CardDestination::DiscardPile,
            CostModifier::None,
        )
    ],
    AcidSlimeMLick => [Inflict(PlayerCondition::Weak(1))],
    AcidSlimeMTackle => [Deal(Damage::Blockable(10))],
    AcidSlimeSLick => [Inflict(PlayerCondition::Weak(1))],
    AcidSlimeSTackle => [Deal(Damage::Blockable(3))],
    CultistDarkStrike => [Deal(Damage::Blockable(6))],
    CultistIncantation => [Apply(EnemyCondition::Ritual(3, true))],
    FungiBeastBite => [Deal(Damage::Blockable(6))],
    FungiBeastGrow => [Gain(Resource::Strength(3))],
    GremlinNobBellow => [Apply(EnemyCondition::Enrage(2))],
    GremlinNobRush => [Deal(Damage::Blockable(14))],
    GremlinNobSkullBash => [Deal(Damage::Blockable(6)), Inflict(PlayerCondition::Vulnerable(2))],
    JawWormBellow => [Gain(Resource::Strength(3)), Gain(Resource::Block(6))],
    JawWormChomp => [Deal(Damage::Blockable(11))],
    JawWormThrash => [Deal(Damage::Blockable(7)), Gain(Resource::Block(5))],
    SpikeSlimeMFlameTackle => [
        Deal(Damage::Blockable(8)),
        CreateCards(
            CardPool::Fixed(&[Card::Slimed]),
            CardSelection::All,
            CardDestination::DiscardPile,
            CostModifier::None,
        )
    ],
    SpikeSlimeMLick => [Inflict(PlayerCondition::Frail(1))],
    SpikeSlimeSTackle => [Deal(Damage::Blockable(5))],
);
