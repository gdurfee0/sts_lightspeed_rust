// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Intent)

use crate::data::EnemyEffect;
use crate::types::{AttackCount, AttackDamage};

/// An `Intent` provides the user-visible view of the enemy's next action.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Intent {
    Aggressive(AttackDamage, AttackCount),
    AggressiveBuff(AttackDamage, AttackCount),
    AggressiveDebuff(AttackDamage, AttackCount),
    AggressiveDefensive(AttackDamage, AttackCount),
    Cowardly,
    Defensive,
    DefensiveBuff,
    DefensiveDebuff,
    Sleeping,
    StrategicBuff,
    StrategicDebuff,
    Stunned,
    Unknown,
}

impl From<&[EnemyEffect]> for Intent {
    fn from(effect_chain: &[EnemyEffect]) -> Intent {
        let has_debuff = effect_chain
            .iter()
            .any(|effect| matches!(effect, EnemyEffect::Apply(_)));
        let has_buff = effect_chain
            .iter()
            .any(|effect| matches!(effect, EnemyEffect::ApplyToSelf(_)));
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
