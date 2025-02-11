use crate::components::{AttackerStatus, DefenderStatus};
use crate::data::Damage;
use crate::types::{Block, Hp};

pub struct DamageCalculator;

/// Damage calculations after attacker and defender conditions are applied (i.e. strength, weak,
/// and vulnerable).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CalculatedDamage {
    Blockable(Hp),
    BlockableNonAttack(Hp),
    HpLoss(Hp),
}

/// The initial damage calculation after only strength is applied. Useful as an intermediate result.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum InitialCalculatedDamage {
    Blockable(Hp),
    BlockableNonAttack(Hp),
    HpLoss(Hp),
}

impl DamageCalculator {
    /// Calculates the damage inflicted, taking into account strength and weakness of the attacker
    /// and vulnerability of the defender.
    pub fn calculate_damage_inflicted<A: AttackerStatus, D: DefenderStatus>(
        attacker: &A,
        maybe_defender: Option<&D>,
        damage: &Damage,
    ) -> CalculatedDamage {
        let initial_damage = Self::calculate_initial_damage(attacker, damage);
        Self::calculate_final_damage(attacker, maybe_defender, initial_damage)
    }

    /// Calculates the block gained by a defender, taking into account dexterity and frailty.
    pub fn calculate_block_gained<D: DefenderStatus>(defender: &D, amount: Block) -> Block {
        let initial_amount = amount.saturating_add_signed(defender.dexterity());
        if defender.is_frail() {
            (initial_amount as f32 * 0.75).floor() as Block
        } else {
            initial_amount
        }
    }

    /// Partial calculation of damage inflicted taking only strength into account.
    fn calculate_initial_damage<A: AttackerStatus>(
        attacker: &A,
        damage: &Damage,
    ) -> InitialCalculatedDamage {
        match damage {
            Damage::Blockable(amount) => InitialCalculatedDamage::Blockable(
                amount.saturating_add_signed(attacker.strength()),
            ),
            Damage::BlockableCountingStrikeCards(base_amount, per_strike_bonus) => {
                InitialCalculatedDamage::Blockable(
                    (base_amount
                        + attacker.number_of_strike_cards_owned() as Hp * per_strike_bonus)
                        .saturating_add_signed(attacker.strength()),
                )
            }
            Damage::BlockableEqualToDrawPileSize => InitialCalculatedDamage::Blockable(
                (attacker.draw_pile_size() as Hp).saturating_add_signed(attacker.strength()),
            ),
            Damage::BlockableEqualToPlayerBlock => InitialCalculatedDamage::Blockable(
                (attacker.block() as Hp).saturating_add_signed(attacker.strength()),
            ),
            Damage::BlockableWithStrengthMultiplier(base_amount, strength_multiplier) => {
                InitialCalculatedDamage::Blockable(
                    base_amount.saturating_add_signed(attacker.strength() * strength_multiplier),
                )
            }
            Damage::BlockableNonAttack(amount) => {
                InitialCalculatedDamage::BlockableNonAttack(*amount)
            }
            Damage::HpLoss(amount) => InitialCalculatedDamage::HpLoss(*amount),

            Damage::HpLossEqualToHandSize => {
                InitialCalculatedDamage::HpLoss(attacker.hand_size() as Hp)
            }
        }
    }

    /// Final calculation of damage inflicted taking into account weakness and vulnerability of the
    /// attacker and defender.
    fn calculate_final_damage<A: AttackerStatus, D: DefenderStatus>(
        attacker: &A,
        maybe_defender: Option<&D>,
        damage: InitialCalculatedDamage,
    ) -> CalculatedDamage {
        match damage {
            InitialCalculatedDamage::Blockable(amount) => {
                let attacker_modified_amount = if attacker.is_weak() {
                    (amount as f32 * 0.75).floor() as Hp
                } else {
                    amount
                };
                if maybe_defender.map_or(false, |d| d.is_vulnerable()) {
                    CalculatedDamage::Blockable(
                        (attacker_modified_amount as f32 * 1.5).floor() as Hp
                    )
                } else {
                    CalculatedDamage::Blockable(attacker_modified_amount)
                }
            }
            InitialCalculatedDamage::BlockableNonAttack(amount) => {
                CalculatedDamage::BlockableNonAttack(amount)
            }
            InitialCalculatedDamage::HpLoss(amount) => CalculatedDamage::HpLoss(amount),
        }
    }
}
