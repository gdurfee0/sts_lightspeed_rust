use crate::types::{AttackDamage, Block};

use super::card::Card;
use super::{EnemyCondition, PlayerCondition};

#[derive(Debug)]
pub enum EnemyEffect {
    AddToDiscardPile(&'static [Card]),
    Apply(PlayerCondition),
    ApplyToSelf(EnemyCondition),
    DealDamage(AttackDamage),
    /*
    Buff(Buff, StackCount),
    BuffAll(Buff, StackCount),
    Debuff(Debuff, StackCount),
    GainBlock(Block),
    GiveBlockToLeader(Block),
    Heal(Hp),
    HealAll(Hp),
    Reincarnate(),
    Revive(),
    ShuffleIntoDrawPile(&'static [Card]),
    StealCard(),
    */
}

#[derive(Debug)]
pub enum PlayerEffect {
    Apply(EnemyCondition),
    ApplyToAll(EnemyCondition),
    ApplyToSelf(PlayerCondition),
    DealDamage(AttackDamage),
    DealDamageToAll(AttackDamage),
    GainBlock(Block),
    UpgradeOneCardInCombat(),
    /*
    AddToDiscardPile(&'static [Card]),
    AddToHand(&'static [Card]),
    Buff(Buff, StackCount),
    BuffCustom(),
    Channel(Orb, OrbCount),
    ChannelCustom(),
    ChannelRandom(OrbCount),
    DealDamage(AttackDamage),
    DealDamageCustom(),
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