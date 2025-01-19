use anyhow::Error;

use crate::data::{Card, CardDetails, Encounter, PlayerEffect};
use crate::enemy::{EnemyPartyGenerator, EnemyState, EnemyStatus};
use crate::player::{CombatAction, CombatController, PlayerController};
use crate::rng::{Seed, StsRandom};
use crate::types::{AttackDamage, Block, EnemyIndex};

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    ai_rng: StsRandom,
    misc_rng: &'a mut StsRandom,
    player: CombatController<'a>,
    enemy_indexes: Vec<EnemyIndex>,
    enemy_party: [Option<EnemyState>; 5],
}

impl<'a> EncounterSimulator<'a> {
    pub fn new(
        seed_for_floor: Seed,
        encounter: Encounter,
        misc_rng: &'a mut StsRandom,
        player: &'a mut PlayerController,
    ) -> Self {
        let ai_rng = StsRandom::from(seed_for_floor);
        let combat_controller = player.start_combat(StsRandom::from(seed_for_floor));
        Self {
            encounter,
            seed_for_floor,
            ai_rng,
            misc_rng,
            player: combat_controller,
            enemy_indexes: vec![],
            enemy_party: [None, None, None, None, None],
        }
    }

    pub fn run(mut self) -> Result<bool, Error> {
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        EnemyPartyGenerator::new(
            self.seed_for_floor,
            self.encounter,
            &mut self.ai_rng,
            self.misc_rng,
        )
        .generate(&mut self.enemy_party);

        loop {
            self.conduct_player_turn()?;
            if self.combat_is_over() {
                return Ok(!self.player.is_dead());
            }
            self.conduct_enemy_turn()?;
            if self.combat_is_over() {
                return Ok(!self.player.is_dead());
            }
        }
    }

    fn combat_is_over(&self) -> bool {
        self.player.is_dead() || self.enemy_party.iter().all(|enemy| enemy.is_none())
    }

    // Returns true iff the battle is over
    fn conduct_player_turn(&mut self) -> Result<(), Error> {
        self.player.start_turn()?;
        loop {
            let mut enemy_statuses = vec![];
            self.enemy_indexes.clear();
            for (index, maybe_enemy) in self.enemy_party.iter().enumerate() {
                enemy_statuses.push(maybe_enemy.as_ref().map(EnemyStatus::from));
                if maybe_enemy.is_some() {
                    self.enemy_indexes.push(index);
                }
            }
            match self.player.choose_next_action(&enemy_statuses)? {
                CombatAction::PlayCard(card, card_details) => {
                    self.play_card(card, card_details, None)?
                }
                CombatAction::PlayCardAgainstEnemy(card, card_details, enemy_index) => {
                    self.play_card(card, card_details, Some(enemy_index))?
                }
                CombatAction::Potion(_) => todo!(),
                CombatAction::EndTurn => break,
            }
            if self.combat_is_over() {
                return Ok(());
            }
            self.player.discard_card_just_played()?;
        }
        self.player.end_turn()?;
        Ok(())
    }

    fn conduct_enemy_turn(&mut self) -> Result<(), Error> {
        for enemy in self
            .enemy_party
            .iter_mut()
            .filter_map(|enemy| enemy.as_mut())
        {
            // TODO: check for death and remove
            let _ = enemy.start_turn();
            //enemy_party.retain(|enemy| enemy.health().0 > 0);
        }
        for maybe_enemy in self.enemy_party.iter_mut() {
            if let Some(enemy) = maybe_enemy.as_mut() {
                for effect in enemy.next_action(&mut self.ai_rng).effect_chain.iter() {
                    // TODO: reactions
                    println!("[EncounterSimulator] Applying effect: {:?}", effect);
                    if enemy.is_dead() {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player.is_dead() {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    // TODO: reactions
    fn play_card(
        &mut self,
        card: Card,
        card_details: &'static CardDetails,
        enemy_index: Option<EnemyIndex>,
    ) -> Result<(), Error> {
        for effect in card_details.effect_chain.iter() {
            println!("[EncounterSimulator] Applying effect: {:?}", effect);
            match effect {
                PlayerEffect::AddToDiscardPile(_) => todo!(),
                PlayerEffect::AddToHand(_) => todo!(),
                PlayerEffect::Buff(_, _) => todo!(),
                PlayerEffect::BuffCustom() => todo!(),
                PlayerEffect::Channel(_, _) => todo!(),
                PlayerEffect::ChannelCustom() => todo!(),
                PlayerEffect::ChannelRandom(_) => todo!(),
                PlayerEffect::DealDamage(amount) => {
                    let outgoing_damage = self.outgoing_damage(*amount, enemy_index);
                    enemy_index
                        .and_then(|index| self.enemy_party[index].as_mut())
                        .as_mut()
                        .unwrap_or_else(|| {
                            panic!(
                                "No enemy at index {:?}, card played: {:?}",
                                enemy_index, card
                            )
                        })
                        .take_damage(outgoing_damage);
                }
                PlayerEffect::DealDamageCustom() => todo!(),
                PlayerEffect::DealDamageToAll(amount) => {
                    for index in self.enemy_indexes.iter() {
                        let outgoing_damage = self.outgoing_damage(*amount, Some(*index));
                        self.enemy_party[*index]
                            .as_mut()
                            .expect("Enemy exists")
                            .take_damage(outgoing_damage);
                    }
                }
                PlayerEffect::DealDamageToAllCustom() => todo!(),
                PlayerEffect::Debuff(debuff, stacks) => {
                    enemy_index
                        .and_then(|index| self.enemy_party[index].as_mut())
                        .as_mut()
                        .unwrap_or_else(|| {
                            panic!(
                                "No enemy at index {:?}, card played: {:?}",
                                enemy_index, card
                            )
                        })
                        .apply_debuff(*debuff, *stacks);
                }
                PlayerEffect::DebuffAll(_, _) => todo!(),
                PlayerEffect::DebuffCustom() => todo!(),
                PlayerEffect::DebuffSelf(_, _) => todo!(),
                PlayerEffect::Discard(_) => todo!(),
                PlayerEffect::DiscardCustom() => todo!(),
                PlayerEffect::DiscardAtRandom() => todo!(),
                PlayerEffect::Draw(_) => todo!(),
                PlayerEffect::DrawCustom() => todo!(),
                PlayerEffect::EndTurn() => todo!(),
                PlayerEffect::EnterStance(_) => todo!(),
                PlayerEffect::EvokeCustom() => todo!(),
                PlayerEffect::ExhaustCard() => todo!(),
                PlayerEffect::Exhume() => todo!(),
                PlayerEffect::ExitStance() => todo!(),
                PlayerEffect::GainBlock(_) => todo!(),
                PlayerEffect::GainBlockCustom() => todo!(),
                PlayerEffect::GainDexterity(_) => todo!(),
                PlayerEffect::GainEnergy(_) => todo!(),
                PlayerEffect::GainEnergyCustom() => todo!(),
                PlayerEffect::GainFocus(_) => todo!(),
                PlayerEffect::GainOrbSlots(_) => todo!(),
                PlayerEffect::GainStrength(_) => todo!(),
                PlayerEffect::HandCustom() => todo!(),
                PlayerEffect::Heal(_) => todo!(),
                PlayerEffect::LoseHp(_) => todo!(),
                PlayerEffect::LoseOrbSlots(_) => todo!(),
                PlayerEffect::ObtainRandomPotion() => todo!(),
                PlayerEffect::SapStrength(_) => todo!(),
                PlayerEffect::Scry(_) => todo!(),
                PlayerEffect::ShuffleIntoDrawPile(_) => todo!(),
                PlayerEffect::ShuffleIntoDrawPileCustom() => todo!(),
                PlayerEffect::StanceCustom() => todo!(),
                PlayerEffect::TakeDamage(_) => todo!(),
                PlayerEffect::UpgradeOneCardInCombat() => todo!(),
                PlayerEffect::UpgradeAllCardsInCombat() => todo!(),
            }
            if self.combat_is_over() {
                break;
            }
        }
        Ok(())
    }

    fn outgoing_damage(
        &self,
        amount: AttackDamage,
        maybe_enemy_index: Option<EnemyIndex>,
    ) -> AttackDamage {
        let caster_modified_mount = if self.player.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if let Some(enemy_index) = maybe_enemy_index {
            let enemy = self.enemy_party[enemy_index]
                .as_ref()
                .expect("Enemy exists");
            if enemy.is_vulnerable() {
                (caster_modified_mount as f32 * 1.5).floor() as u32
            } else {
                caster_modified_mount
            }
        } else {
            caster_modified_mount
        }
    }

    fn incoming_block(&self, amount: Block) -> Block {
        if self.player.is_frail() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        }
    }
}
