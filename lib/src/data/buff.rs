// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Buffs)

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Buff {
    Block,        // Prevents damage up to the amount of block, consuming block on hit.
    Strength,     // Increases attack damage by X (per hit).
    ThousandCuts, // Whenever you play a card, deal 1 damage to ALL enemies.
}
