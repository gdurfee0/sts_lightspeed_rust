// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Debuffs)

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Debuff {
    Bias,            // At the start of your turn, lose X Focus.
    BlockReturn,     // Whenever you attack this enemy, gain X Block.
    Choked,          // Whenever you play a card this turn, the targeted enemy loses X HP.
    Constricted,     // At the end of your turn, take X damage.
    Confused,        // The costs of your cards are randomized on draw, from 0 to 3.
    CorpseExplosion, // On death, the enemy deals X times its max HP worth of damage to all enemies.
    DexterityDown,   // At the end of your turn, lose X Dexterity.
    DrawReduction,   // Draw 1 fewer card next X turns.
    Entangled,       // You may not play any Attacks this turn.
    Fasting,         // Gain X less Energy at the start of each turn.
    Frail,           // Block gained from cards is reduced by 25%.
    Hex,             // Whenever you play a non-Attack card, shuffle X Dazed into your draw pile.
    LockOn,          // Lightning and Dark orbs deal 50% more damage to this enemy.
    Mark,            // Whenever you play Pressure Points, all enemies with Mark lose X HP.
    NoBlock,         // You may not gain Block from cards for the next X turns.
    NoDraw,          // You may not draw any more cards this turn.
    Poison,          // At the beginning of its turn, the target loses X HP and 1 stack of Poison.
    Shackled,        // At the end of its turn, regains X Strength.
    Slow,            // The enemy receives (X*10)% more damage from attacks this turn.
    StrengthDown,    // At the end of your turn, lose X Strength.
    Vulnerable,      // Target takes 50% more damage from attacks.
    Weak,            // Target deals 25% less attack damage.
    WraithForm,      // Lose X Dexterity at the start of your turn.
}
