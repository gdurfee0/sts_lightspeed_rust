// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Buffs)

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Buff {
    Accuracy,        // Shivs deal X additional damage.
    AfterImage,      // Whenever you play a card, gain X Block.
    Amplify,         // Your next X Power cards are played twice.
    Angry,           // Upon receiving attack damage, gain X Strength.
    Artifact,        // Negates X debuffs.
    BackAttack,      // Deal 50% more damage.
    Barricade,       // Block is not removed at the start of turn.
    BattleHymn,      // At the start of each turn add X Smites into your hand.
    BeatOfDeath,     // Whenever you play a card, take X damage.
    Berserk,         // At the start of your turn, gain X Energy.
    Blasphemer,      // At the start of your turn, die.
    BlockNextTurn,   // Gain X Block next turn.
    Blur,            // Block is not removed at the start of your next X turns.
    Brutality,       // At the start of your turn, lose X HP and draw X cards.
    Buffer,          // Prevent the next X times you would lose HP.
    Burst,           // Your next X Skills are played twice.
    ClawsPlayed,     // Internal: tracks the number of Claws played.
    Collect,         // Put a Miracle+ into your hand at the start of your next X turns.
    Combust,         // At the end of your turn, lose 1 HP and deal X dammage to ALL enemies.
    Corruption,      // Skills cost 0. Whenever you play a Skill, Exhaust it.
    CreativeAi,      // At the start of your turn, add X random Power cards into your hand.
    Curiosity,       // Whenever you play a Power card, gain X Strength.
    CurlUp,          // Upon receiving attack damage, gain X Block, once per combat.
    DarkEmbrace,     // Whenever a card is Exhausted, draw X cards.
    DemonForm,       // At the start of your turn, gain X Strength.
    Deva,            // At the start of your turn gain Energy N times and increase this gain by X.
    Devotion,        // At the start of your turn, gain X Mantra.
    Dexterity,       // Increases block gain from cards by X.
    DoubleDamage,    // Attacks deal double damage for X turns.
    DoubleTap,       // Your next X Attacks are played twice.
    DrawCard,        // Draw X additional cards next turn.
    EchoForm,        // The first X cards you play each turn are played twice.
    Electro,         // Lightning hits ALL enemies.
    Energized,       // Gain X additional Energy next turn.
    Enrage,          // Whenever you play a skill, gain X Strength.
    Envenom,         // Whenever you deal unblocked attack damage, apply X Poison.
    Equilibrium,     // Retain your hand for X turns.
    Establishment,   // Whenever a card is Retained, lower its cost by X.
    Evolve,          // Whenever you draw a Status, draw X cards.
    Explosive,       // Explodes in N turns, dealing X damage.
    Fading,          // Dies in X turns.
    FeelNoPain,      // Whenever a card is Exhausted, gain X Block.
    FireBreathing,   // Whenever you draw a Status or Curse card, deal X damage to ALL enemies.
    FlameBarrier,    // When attacked, deals X damage back. Wears off at the start of next turn.
    Flying,          // Takes 50% less damage on attacks. Removed when attacked X times.
    Focus,           // Increase the effectiveness of Orbs by X.
    Foresight,       // At the start of your turn, Scry X.
    FreeAttackPower, // The next X Attacks you play cost 0.
    Heatsink,        // Whever you play a Power card, draw X cards.
    Hello,           // At the start of your turn, add X random Common cards into your hand.
    InfiniteBlades,  // At the start of your turn, add X Shivs into your hand.
    Intangible,      // Reduce ALL damage taken and HP losses to 1 this turn. Lasts X turns.
    Invincible,      // Can lose only X more HP this turn.
    Juggernaut,      // Whenever you gain Block, deal X damage to a random enemy.
    LifeLink,        // If other LifeLink bearers are still alive, revives in 2 turns.
    LikeWater,       // At the end of your turn, if you are in Calm, gain X Block.
    Loop,            // At the start of your turn, trigger passive ability of your next Orb X times.
    MachineLearning, // At the start of your turn, draw X additional cards.
    Magnetism,       // At the start of your turn, add X random colorless cards into your hand.
    Malleable,       // Upon receiving Attack damage, gains X Block, increasing each time.
    Mantra,          // Whenever you gain 10 Mantra, enter Divinity.
    MasterReality,   // Whenever a card is created during combat, Upgrade it.
    Mayhem,          // At the start of your turn, play the top X cards of your draw pile.
    MentalFortress,  // Whenever you switch Stances, gain X Block.
    Metallicize,     // At the end of your/its turn, gain X Block.
    Minion,          // Abandons combat without their leader.
    ModeShift,       // After receiving X damage, changes to a defensive mode.
    Nightmare,       // Add X of a chosen card into your hand next turn.
    Nirvana,         // Whenever you Scry, gain X Block.
    NoxiousFumes,    // At the start of your turn, apply X Poison to ALL enemies.
    Omega,           // At the end of your turn, deal X damage to ALL enemies.
    PainfulStabs,    // When you receive attack damage from this enemy, X Wounds into discard pile.
    Panache,         // If you play N more cards this turn, deal X damage to all enemies.
    PenNib,          // Your next Attack deals double damage.
    Phantasmal,      // Deal Double Damage for the next X turns.
    PlatedArmor,     // At the end of your turn, gain X Block.
    Rage,            // Whenever you play an Attack, gain X Block.
    Reactive,        // Upon receiving attack damage, changes its Intent.
    Rebound,         // The next X cards you play this turn are placed on top of your draw pile.
    Regenerate,      // At the start of your turn, heal X HP.
    Regeneration,    // At the end of your turn, heal X HP and reduce Regeneration by 1.
    Repair,          // At the end of combat, heal X HP.
    Ritual,          // At the end of your/its turn, gain X Stength.
    Rupture,         // Whenever you lose HP from a card, gain X Strength.
    Rushdown,        // Whenever you enter Wrath, draw X cards.
    Sadistic,        // Whenever you apply a Debuff to an enemy, deal X damage.
    SharpHide,       // Whenever you play an attack, take X damage.
    Shifting,        // Upon losing HP, loses that much Strength until the end of the turn.
    SimmeringRage,   // At the start of your next turn, enter Wrath.
    Split,           // When its HP is at or below 50%, split into 2 smaller Slimes.
    SporeCloud,      // On death, applies X Vulnerable.
    Stasis,          // On death, a stolen card is returned to your hand.
    StaticDischarge, // Whenever you receive unblocked Attack damage, Channel X Lightning.
    Storm,           // Whenever you play a Power card, channel X Lightning.
    Strength,        // Increases attack damage by X (per hit).
    StrengthUp,      // At the end of its turn, gains X Strength.
    Study,           // At the end of your turn, shuffle X Insights into your draw pile.
    Surrounded,      // Receive 50% more damage if attacked from behind.
    TheBomb,         // At the end of N turns, deal X damage to all enemies.
    Theivery,        // Steals X Gold whenever it attacks.
    Thorns,          // When attacked, deal X damage back.
    ThousandCuts,    // Whenever you play a card, deal 1 damage to ALL enemies.
    TimeWarp,        // Whenever you play N cards, ends your turn and gains X Strength.
    ToolsOfTheTrade, // At the start of your turn, draw X cards and discard X cards.\
    Unawakened,      // This enemy hasn't awakened yet...
    Vigor,           // Your next Attack deals X additional damage.
    WaveOfTheHand,   // Whenever you gain Block this turn, apply X Weak to ALL enemies.
    WellLaidPlans,   // At the end of your turn, Retain up to X cards.
}
