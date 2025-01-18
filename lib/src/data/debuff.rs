// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Debuffs)

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Debuff {
    /// At the start of your turn, lose X Focus.
    Bias,
    /// Whenever you attack this enemy, gain X Block.
    BlockReturn,
    /// Whenever you play a card this turn, the targeted enemy loses X HP.
    Choked,
    /// At the end of your turn, take X damage.
    Constricted,
    /// The costs of your cards are randomized on draw, from 0 to 3.
    Confused,
    /// On death, the enemy deals X times its max HP in damage to all enemies.
    CorpseExplosion,
    /// At the end of your turn, lose X Dexterity.
    DexterityDown,
    /// Draw 1 fewer card next X turns.
    DrawReduction,
    /// You may not play any Attacks this turn.
    Entangled,
    /// Gain X less Energy at the start of each turn.
    Fasting,
    /// Block gained from cards is reduced by 25%.
    Frail,
    /// Whenever you play a non-Attack card, shuffle X Dazed into your draw pile.
    Hex,
    /// Lightning and Dark orbs deal 50% more damage to this enemy.
    LockOn,
    /// Whenever you play Pressure Points, all enemies with Mark lose X HP.
    Mark,
    /// You may not gain Block from cards for the next X turns.    
    NoBlock,
    /// You may not draw any more cards this turn.
    NoDraw,
    /// At the beginning of its turn, the target loses X HP and 1 stack of Poison.
    Poison,
    /// At the end of its turn, regains X Strength.
    Shackled,
    /// The enemy receives (X*10)% more damage from attacks this turn.
    Slow,
    /// At the end of your turn, lose X Strength.
    StrengthDown,
    /// Lose X Strength this turn.
    StrengthLossThisTurn,
    /// Target takes 50% more damage from attacks.
    Vulnerable,
    /// Target deals 25% less attack damage.
    Weak,
    /// Lose X Dexterity at the start of your turn.
    WraithForm,
}
