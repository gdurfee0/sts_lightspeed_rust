use crate::{
    AttackDamage, Block, DiscardCount, DrawCount, Energy, Focus, Hp, OrbCount, StackCount,
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
    Channel(Orb, OrbCount),
    DealDamage(AttackDamage),
    DealDamageCustom(Card),
    DealDamageToAll(AttackDamage),
    DealDamageToAllCustom(Card),
    Debuff(Debuff, StackCount),
    DebuffSelf(Debuff, StackCount),
    Discard(DiscardCount),
    DiscardAtRandom(),
    Draw(DrawCount),
    EnterStance(Stance),
    ExhaustCard(),
    GainBlock(Block),
    GainBlockCustom(Card),
    GainEnergy(Energy),
    GainEnergyCustom(Card),
    GainFocus(Focus),
    HandCustom(Card),
    Heal(Hp),
    LoseHp(Hp),
    ObtainRandomPotion(),
    ShuffleIntoDrawPile(&'static [Card]),
    UpgradeOneCardInCombat(),
    UpgradeAllCardsInCombat(),
}
