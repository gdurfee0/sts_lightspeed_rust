use std::io::stdin;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::components::{Choice, EnemyStatus, Notification, PlayerStatus, Prompt, StsMessage};
use crate::data::{CardDetails, Character, PlayerEffect, Resource, TargetEffect};
use crate::systems::DamageCalculator;
use crate::types::EnemyIndex;

pub struct CombatClient<'a> {
    player_status: PlayerStatus,
    enemy_party: Vec<Option<EnemyStatus>>,
    card_chosen: Option<&'static CardDetails>,
    from_server: &'a Receiver<StsMessage>,
    to_server: &'a Sender<usize>,
}

impl<'a> CombatClient<'a> {
    pub fn new(
        character: &'static Character,
        from_server: &'a Receiver<StsMessage>,
        to_server: &'a Sender<usize>,
    ) -> Self {
        Self {
            player_status: PlayerStatus::new(character),
            enemy_party: vec![None, None, None, None, None],
            card_chosen: None,
            from_server,
            to_server,
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        loop {
            match self.from_server.recv()? {
                StsMessage::Choices(prompt, choices) => {
                    let choice = self.collect_user_choice(prompt, choices)?;
                    self.to_server.send(choice)?;
                }
                StsMessage::GameOver(result) => {
                    println!(
                        "[Main] Game Over; the player was {}victorious",
                        if result { "" } else { "not " }
                    );
                    break;
                }
                StsMessage::Notification(notification) => {
                    self.process_notification(notification);
                }
            }
        }
        Ok(())
    }

    fn process_notification(&mut self, notification: Notification) {
        println!("{:?}", notification);
        match notification {
            Notification::Block(block) => self.player_status.block = block,
            Notification::Conditions(player_conditions) => {
                self.player_status.conditions = player_conditions;
            }
            Notification::Deck(cards) => self.player_status.deck = cards,
            Notification::Dexterity(dexterity) => self.player_status.dexterity = dexterity,
            Notification::DiscardPile(cards) => self.player_status.discard_pile = cards,
            Notification::EnemyParty(enemy_party) => self.enemy_party = enemy_party,
            Notification::Energy(energy) => self.player_status.energy = energy,
            Notification::Gold(gold) => self.player_status.gold = gold,
            Notification::Health(health) => {
                self.player_status.hp = health.0;
                self.player_status.hp_max = health.1;
            }
            Notification::Potions(potions) => self.player_status.potions = potions,
            Notification::Relics(relics) => self.player_status.relics = relics,
            Notification::Status(player_status) => self.player_status = player_status,
            Notification::Strength(strength) => self.player_status.strength = strength,
            _ => {}
        }
    }

    fn collect_user_choice(
        &mut self,
        prompt: Prompt,
        choices: Vec<Choice>,
    ) -> Result<usize, Error> {
        loop {
            if choices.is_empty() {
                return Err(anyhow!("Cannot choose from an empty list of choices"));
            }
            // Display prompts and choices, converting to 1-indexing for user convenience
            println!("\n{}:", prompt);
            for (i, choice) in choices.iter().enumerate() {
                println!(
                    "{}: {} {}",
                    i + 1,
                    choice,
                    match choice {
                        Choice::PlayCardFromHand(_, card, _) => {
                            self.modified_card_details(CardDetails::for_card(*card), None)
                        }
                        Choice::TargetEnemy(index, _) => self.modified_card_details(
                            self.card_chosen.expect("Card just chosen"),
                            Some(*index)
                        ),
                        _ => "".to_string(),
                    }
                );
            }
            let mut user_input = String::new();
            match stdin().read_line(&mut user_input) {
                Ok(0) => {
                    return Err(anyhow!("User closed the input stream"));
                }
                Ok(_) => match user_input.trim().parse::<usize>() {
                    Ok(i) if i <= choices.len() && i > 0 => {
                        if let Choice::PlayCardFromHand(_, card, _) = &choices[i - 1] {
                            self.card_chosen = Some(CardDetails::for_card(*card));
                        }
                        return Ok(i - 1);
                    }
                    _ => {
                        println!(
                            "Invalid input: must be an integer in the range {}..={}",
                            1,
                            choices.len()
                        );
                    }
                },
                Err(e) => return Err(anyhow!("Error reading input: {}", e)),
            }
        }
    }

    fn modified_card_details(
        &self,
        details: &'static CardDetails,
        maybe_enemy_index: Option<EnemyIndex>,
    ) -> String {
        let mut first = true;
        let mut result = "[".to_string();
        for effect in details.on_play.iter() {
            if !first {
                result.push_str(", ");
            }
            match effect {
                PlayerEffect::Gain(Resource::Block(amount)) => result.push_str(&format!(
                    "Gain {} block",
                    DamageCalculator::calculate_block_gained(&self.player_status, *amount).amount
                )),
                PlayerEffect::ToAllEnemies(effect) => result.push_str(
                    &self.modified_target_effect(effect, maybe_enemy_index, " to ALL enemies"),
                ),
                PlayerEffect::ToRandomEnemy(effect) => result.push_str(
                    &self.modified_target_effect(effect, maybe_enemy_index, " to a random enemy"),
                ),
                PlayerEffect::ToSingleTarget(effect) => result.push_str(
                    &self.modified_target_effect(effect, maybe_enemy_index, " to a single enemy"),
                ),
                _ => result.push_str(&format!("{:?}", effect)),
            }
            first = false;
        }
        result.push(']');
        if let Some(enemy_index) = maybe_enemy_index {
            let enemy = self.enemy_party[enemy_index]
                .as_ref()
                .expect("Enemy exists");
            result.push_str(&format!(" (HP: {}/{})", enemy.hp, enemy.hp_max));
        }
        result
    }

    fn modified_target_effect(
        &self,
        effect: &TargetEffect,
        maybe_enemy_index: Option<EnemyIndex>,
        target_suffix: &str,
    ) -> String {
        match effect {
            TargetEffect::Deal(damage) => {
                let damage = DamageCalculator::calculate_damage_inflicted(
                    &self.player_status,
                    maybe_enemy_index.and_then(|i| self.enemy_party[i].as_ref()),
                    damage,
                );
                format!("Deal {:?} damage{}", damage, target_suffix)
            }
            _ => format!("{:?}{}", effect, target_suffix),
        }
    }
}
