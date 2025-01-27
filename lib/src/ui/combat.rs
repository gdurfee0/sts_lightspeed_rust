use std::io::stdin;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::components::{Choice, EnemyStatus, Notification, Prompt, StsMessage};
use crate::data::{CardDetails, PlayerCondition, PlayerEffect};
use crate::types::{AttackDamage, Block, Dexterity, EnemyIndex};

pub struct CombatClient<'a> {
    my_conditions: Vec<PlayerCondition>,
    my_dexterity: Dexterity,
    enemy_party: Vec<Option<EnemyStatus>>,
    card_chosen: Option<&'static CardDetails>,

    from_server: &'a Receiver<StsMessage>,
    to_server: &'a Sender<usize>,
}

impl<'a> CombatClient<'a> {
    pub fn new(from_server: &'a Receiver<StsMessage>, to_server: &'a Sender<usize>) -> Self {
        Self {
            my_conditions: vec![],
            my_dexterity: 0,
            enemy_party: vec![],
            card_chosen: None,
            from_server,
            to_server,
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        loop {
            match self.from_server.recv()? {
                StsMessage::Notification(Notification::Conditions(conditions)) => {
                    self.my_conditions = conditions;
                    println!("Buffs: {:?}", self.my_conditions);
                }
                StsMessage::Notification(Notification::Dexterity(dexterity)) => {
                    self.my_dexterity = dexterity;
                    println!("Dexterity: {}", self.my_dexterity);
                }
                StsMessage::Choices(prompt, choices) => {
                    let choice = self.collect_user_choice(prompt, choices)?;
                    self.to_server.send(choice)?;
                }
                StsMessage::Notification(Notification::EndingCombat) => {
                    println!("Combat ended");
                    break;
                }
                StsMessage::Notification(Notification::EnemyParty(party)) => {
                    for enemy_status in party.iter().flatten() {
                        println!("Enemy: {}", enemy_status);
                    }
                    self.enemy_party = party;
                }
                StsMessage::GameOver(result) => {
                    println!(
                        "[Main] Game Over; the player was {}victorious",
                        if result { "" } else { "not " }
                    );
                    break;
                }
                sts_message => println!("{:?}", sts_message),
            }
        }
        Ok(())
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

    fn is_frail(&self) -> bool {
        self.my_conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Frail(_)))
    }

    fn is_weak(&self) -> bool {
        self.my_conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Weak(_)))
    }

    fn outgoing_damage(
        &self,
        amount: AttackDamage,
        maybe_enemy_index: Option<EnemyIndex>,
    ) -> AttackDamage {
        let caster_modified_mount = if self.is_weak() {
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
        let amount = amount.saturating_add_signed(self.my_dexterity);
        if self.is_frail() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        }
    }

    fn modified_card_details(
        &self,
        details: &'static CardDetails,
        maybe_enemy_index: Option<EnemyIndex>,
    ) -> String {
        let mut first = true;
        let mut result = "[".to_string();
        for effect in details.effect_chain.iter() {
            if !first {
                result.push_str(", ");
            }
            match effect {
                PlayerEffect::GainBlock(amount) => {
                    result.push_str(&format!("Gain {} block", self.incoming_block(*amount)))
                }
                PlayerEffect::DealDamage(amount) => result.push_str(&format!(
                    "Deal {} damage",
                    self.outgoing_damage(*amount, maybe_enemy_index)
                )),
                PlayerEffect::DealDamageToAll(amount) => result.push_str(&format!(
                    "Deal {} damage to ALL enemies",
                    self.outgoing_damage(*amount, maybe_enemy_index)
                )),
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
}
