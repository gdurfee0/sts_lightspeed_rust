use crate::types::{
    AttackDamage, Block, Dexterity, DiscardCount, DrawCount, Energy, Focus, Hp, OrbCount, OrbSlots,
    ScryCount, StackCount, Strength,
};

use super::buff::Buff;
use super::card::Card;
use super::debuff::Debuff;
use super::orb::Orb;
use super::stance::Stance;

#[derive(Debug)]
pub enum EnemyEffect {
    AddToDiscardPile(&'static [Card]),
    Buff(Buff, StackCount),
    BuffAll(Buff, StackCount),
    DealDamage(AttackDamage),
    Debuff(Debuff, StackCount),
    GainBlock(Block),
    GiveBlockToLeader(Block),
    Heal(Hp),
    HealAll(Hp),
    Reincarnate(),
    Revive(),
    ShuffleIntoDrawPile(&'static [Card]),
    StealCard(),
}

#[derive(Debug)]
pub enum PlayerEffect {
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
}
