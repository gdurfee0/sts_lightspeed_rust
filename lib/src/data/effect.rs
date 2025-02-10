use crate::types::{AttackCount, Block, DrawCount, Energy, Gold, Hp, HpMax, Strength};

use super::card::Card;
use super::condition::{EnemyCondition, PlayerCondition};
use super::damage::Damage;
use super::intent::Intent;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSource {
    AllCardsInCombat,
    AttacksInDrawPile,
    DiscardPile,
    ExhaustPile,
    Hand,
    NonAttackCardsInHand,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardPool {
    AttacksAndPowersInHand,
    CardInPlay,
    CharacterAttackPool,
    CharacterCardPool,
    CharacterPowerPool,
    CharacterSkillPool,
    ColorlessCardPool,
    Fixed(&'static [Card]),
    UpgradedColorlessCardPool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSelection {
    All,
    PlayerChoice(usize),
    PlayerChoiceUnlimited,
    PlayerChoiceUpTo(usize),
    Random(usize),
    RandomThenPlayerChoice(usize, usize),
    RandomX,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardDestination {
    BottomOfDrawPile,
    DiscardPile,
    ExhaustPile,
    Hand,
    ShuffledIntoDrawPile,
    TopOfDrawPile,
    TwoCopiesInHand,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CostModifier {
    None,
    ZeroThisCombat,
    ZeroThisTurn,
    ZeroUntilPlayed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EnemyTarget {
    All,
    Random,
    Single,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerEffectCondition {
    IfHandContainsNoAttackCards,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TargetCondition {
    AttackWasFatal,
    IntendsToAttack,
    IsVulnerable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Resource {
    Block(Block),
    CurrentBlockIsDoubled,
    CurrentStrengthIsDoubled,
    Energy(Energy),
    Gold(Gold),
    Hp(Hp),
    HpEqualToUnblockedDamage,
    HpMax(HpMax),
    Strength(Strength),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TargetEffect {
    Conditional(TargetCondition, &'static [PlayerEffect]),
    Deal(Damage),
    DealXTimes(Damage),
    Inflict(EnemyCondition),
    SapStrength(Strength),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnemyEffect {
    Apply(EnemyCondition),
    CreateCards(CardPool, CardSelection, CardDestination, CostModifier),
    Deal(Damage),
    DealThenInflict(Damage, PlayerCondition),
    Gain(Resource),
    Inflict(PlayerCondition),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlayerEffect {
    Apply(PlayerCondition),
    Conditional(PlayerEffectCondition, &'static [PlayerEffect]),
    CreateCards(CardPool, CardSelection, CardDestination, CostModifier),
    Draw(DrawCount),
    ForEachExhausted(&'static [PlayerEffect]), // TODO: Merge this into ManipulateCards
    Gain(Resource),
    Lose(Resource),
    ManipulateCards(CardSource, CardSelection, CardDestination, CostModifier),
    PlayThenExhaustTopCardOfDrawPile,
    RampUpCardDamage(Hp),
    TakeDamage(Damage),
    ToAllEnemies(TargetEffect),
    ToRandomEnemy(TargetEffect),
    ToSingleTarget(TargetEffect),
    Upgrade(CardSource, CardSelection),
}

impl From<&[EnemyEffect]> for Intent {
    fn from(effect_chain: &[EnemyEffect]) -> Intent {
        let has_debuff = effect_chain.iter().any(|effect| {
            matches!(
                effect,
                EnemyEffect::Inflict(_) | EnemyEffect::CreateCards(_, _, _, _)
            )
        });
        let has_buff = effect_chain.iter().any(|effect| {
            matches!(
                effect,
                EnemyEffect::Apply(_) | EnemyEffect::Gain(Resource::Strength(_))
            )
        });
        let has_defense = effect_chain
            .iter()
            .any(|effect| matches!(effect, EnemyEffect::Gain(Resource::Block(_))));

        let attack_damage: Option<Hp> = effect_chain.iter().find_map(|effect| match effect {
            EnemyEffect::Deal(Damage::Blockable(amount))
            | EnemyEffect::Deal(Damage::HpLoss(amount)) => Some(*amount),
            _ => None,
        });
        let attack_count: AttackCount = effect_chain
            .iter()
            .filter_map(|effect| {
                if let EnemyEffect::Deal(_) = effect {
                    Some(1)
                } else {
                    None
                }
            })
            .sum();
        // TODO: Cowardly, Sleeping, Stunned, Unknown special cases
        if attack_count > 0 {
            let attack_damage = attack_damage.expect("attack_count > 0");
            if has_buff {
                Intent::AggressiveBuff(attack_damage, attack_count)
            } else if has_debuff {
                Intent::AggressiveDebuff(attack_damage, attack_count)
            } else if has_defense {
                Intent::AggressiveDefensive(attack_damage, attack_count)
            } else {
                Intent::Aggressive(attack_damage, attack_count)
            }
        } else if has_defense {
            if has_buff {
                Intent::DefensiveBuff
            } else if has_debuff {
                Intent::DefensiveDebuff
            } else {
                Intent::Defensive
            }
        } else if has_buff {
            Intent::StrategicBuff
        } else if has_debuff {
            Intent::StrategicDebuff
        } else {
            Intent::Unknown
        }
    }
}
