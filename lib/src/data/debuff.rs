// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Debuffs)

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Debuff {
    Frail,      // Block gained from cards is reduced by 25%.
    Vulnerable, // Target takes 50% more damage from attacks.
    Weak,       // Target deals 25% less attack damage.
}
