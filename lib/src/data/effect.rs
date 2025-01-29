use crate::types::{AttackCount, AttackDamage, Block, DrawCount, Energy, Hp, Strength};

use super::card::{Card, CardType};
use super::condition::{EnemyCondition, PlayerCondition};
use super::intent::Intent;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnemyEffect {
    AddToDiscardPile(&'static [Card]),
    Apply(PlayerCondition),
    ApplyToSelf(EnemyCondition),
    DealDamage(AttackDamage),
    GainBlock(Block),
    GainStrength(Strength),
    /*
    Buff(Buff, StackCount),
    BuffAll(Buff, StackCount),
    Debuff(Debuff, StackCount),
    GiveBlockToLeader(Block),
    Heal(Hp),
    HealAll(Hp),
    Reincarnate(),
    Revive(),
    ShuffleIntoDrawPile(&'static [Card]),
    StealCard(),
    */
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlayerEffect {
    AddRandomCardThatCostsZeroThisTurnToHand(CardType),
    AddToDiscardPile(&'static [Card]),
    Apply(EnemyCondition),
    ApplyToAll(EnemyCondition),
    ApplyToSelf(PlayerCondition),
    ApplyToSelfAtEndOfTurn(PlayerCondition),
    CloneSelfIntoDiscardPile(),
    CloneAttackOrPowerCardIntoHand(usize),
    DealDamage(AttackDamage),
    DealDamageCustom(),
    DealDamageToAll(AttackDamage),
    DealDamageToRandomEnemy(AttackDamage),
    DealDamageWithStrengthMultiplier(AttackDamage, Strength),
    Draw(DrawCount),
    ExhaustCardInHand(),
    ExhaustCustom(),
    ExhaustRandomCardInHand(),
    GainBlock(Block),
    GainBlockCustom(),
    GainEnergy(Energy),
    GainStrength(Strength),
    IfEnemyVulnerable(Vec<PlayerEffect>),
    LoseHp(Hp),
    PlayThenExhaustTopCardOfDrawPile(),
    PutCardFromDiscardPileOnTopOfDrawPile(),
    PutCardFromHandOnTopOfDrawPile(),
    UpgradeAllCardsInHandThisCombat(),
    UpgradeOneCardInHandThisCombat(),
    /*
    AddToHand(&'static [Card]),
    Buff(Buff, StackCount),
    BuffCustom(),
    Channel(Orb, OrbCount),
    ChannelCustom(),
    ChannelRandom(OrbCount),
    DealDamage(AttackDamage),
    DealDamageToAll(AttackDamage),
    DealDamageToAllCustom(),
    Debuff(Debuff, StackCount),
    DebuffAll(Debuff, StackCount),
    DebuffCustom(),
    DebuffSelf(Debuff, StackCount),
    Discard(DiscardCount),
    DiscardCustom(),
    DiscardAtRandom(),
    Draw(DrawCount),
    DrawCustom(),
    EndTurn(),
    EnterStance(Stance),
    EvokeCustom(),
    ExhaustCard(),
    Exhume(),
    ExitStance(),
    GainBlock(Block),
    GainBlockCustom(),
    GainDexterity(Dexterity),
    GainEnergy(Energy),
    GainEnergyCustom(),
    GainFocus(Focus),
    GainOrbSlots(OrbSlots),
    GainStrength(Strength),
    HandCustom(),
    Heal(Hp),
    HealCustom(),
    LoseHp(Hp),
    LoseOrbSlots(OrbSlots),
    ObtainRandomPotion(),
    SapStrength(Strength),
    Scry(ScryCount),
    ShuffleIntoDrawPile(&'static [Card]),
    ShuffleIntoDrawPileCustom(),
    StanceCustom(),
    TakeDamage(AttackDamage),
    UpgradeOneCardInCombat(),
    UpgradeAllCardsInCombat(),
    */
}

impl From<&[EnemyEffect]> for Intent {
    fn from(effect_chain: &[EnemyEffect]) -> Intent {
        let has_debuff = effect_chain.iter().any(|effect| {
            matches!(
                effect,
                EnemyEffect::Apply(_) | EnemyEffect::AddToDiscardPile(_)
            )
        });
        let has_buff = effect_chain.iter().any(|effect| {
            matches!(
                effect,
                EnemyEffect::ApplyToSelf(_) | EnemyEffect::GainStrength(_)
            )
        });
        let has_defense = false;
        /* effect_chain.iter().any(|effect| {
            matches!(
                effect,
                EnemyEffect::GainBlock(_) | EnemyEffect::GiveBlockToLeader(_)
            )
        }); */
        let attack_damage: Option<AttackDamage> = effect_chain.iter().find_map(|effect| {
            if let EnemyEffect::DealDamage(attack_damage) = effect {
                Some(*attack_damage)
            } else {
                None
            }
        });
        let attack_count: AttackCount = effect_chain
            .iter()
            .filter_map(|effect| {
                if let EnemyEffect::DealDamage(_) = effect {
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
