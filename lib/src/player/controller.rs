use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::data::{Card, CardDetails, Character, Debuff, EnemyType, NeowBlessing, Potion, Relic};
use crate::enemy::EnemyStatus;
use crate::message::{PotionAction, StsMessage};
use crate::rng::StsRandom;
use crate::types::{
    AttackDamage, Block, ColumnIndex, EnemyIndex, Gold, HandIndex, Hp, HpMax, StackCount,
};

use super::comms::{Comms, MainScreenAction};
use super::state::{CombatState, PlayerState};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct PlayerController {
    state: PlayerState,
    comms: Comms,
}

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself. Also hands combat-related communication
/// with the user.
#[derive(Debug)]
pub struct CombatController<'a> {
    combat_state: CombatState,
    card_just_played: Option<HandIndex>,
    state: &'a mut PlayerState,
    comms: &'a mut Comms,
}

/// Returned by various methods to indicate the player's choice of action in combat.
#[derive(Clone, Debug)]
pub enum CombatAction {
    EndTurn,
    PlayCard(Card, &'static CardDetails),
    PlayCardAgainstEnemy(Card, &'static CardDetails, EnemyIndex),
    Potion(PotionAction),
}

/// Some convenience methods for Player interaction.
impl PlayerController {
    pub fn new(
        character: &'static Character,
        from_client: Receiver<usize>,
        to_client: Sender<StsMessage>,
    ) -> Self {
        let state = PlayerState::new(character);
        let comms = Comms::new(from_client, to_client);
        Self { state, comms }
    }

    pub fn hp(&self) -> Hp {
        self.state.hp()
    }

    pub fn hp_max(&self) -> HpMax {
        self.state.hp_max()
    }

    pub fn gold(&self) -> Gold {
        self.state.gold()
    }

    pub fn increase_hp(&mut self, amount: Hp) -> Result<(), Error> {
        self.state.increase_hp(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn decrease_hp(&mut self, amount: Hp) -> Result<(), Error> {
        self.state.decrease_hp(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn increase_hp_max(&mut self, amount: HpMax) -> Result<(), Error> {
        self.state.increase_hp_max(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn decrease_hp_max(&mut self, amount: HpMax) -> Result<(), Error> {
        self.state.decrease_hp_max(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn increase_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.state.increase_gold(amount);
        self.comms.send_gold_changed(self.state.gold())
    }

    pub fn decrease_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.state.decrease_gold(amount);
        self.comms.send_gold_changed(self.state.gold())
    }

    pub fn send_full_player_state(&self) -> Result<(), Error> {
        self.comms.send_health_changed(self.state.health())?;
        self.comms.send_gold_changed(self.state.gold())?;
        self.comms.send_deck(self.state.deck())?;
        self.comms.send_potions(self.state.potions())?;
        self.comms.send_relics(self.state.relics())
    }

    pub fn send_game_over(&self) -> Result<(), Error> {
        self.comms.send_game_over(self.state.hp() > 0)
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), anyhow::Error> {
        self.comms.send_map_string(map_string)
    }

    pub fn obtain_card(&mut self, card: Card) -> Result<(), Error> {
        self.state.obtain_card(card);
        self.comms.send_card_obtained(card)
    }

    pub fn choose_card_to_obtain(&mut self, cards: &[Card]) -> Result<(), Error> {
        if let Some(card) = self.comms.choose_card_to_obtain(cards, true)? {
            self.obtain_card(card)
        } else {
            Ok(())
        }
    }

    pub fn obtain_relic(&mut self, relic: Relic) -> Result<(), Error> {
        self.state.obtain_relic(relic);
        self.comms.send_relic_obtained(relic)
    }

    pub fn choose_neow_blessing(&self, blessings: &[NeowBlessing]) -> Result<NeowBlessing, Error> {
        self.comms.choose_neow_blessing(blessings)
    }

    pub fn climb_floor(&mut self, climb_options: &[ColumnIndex]) -> Result<ColumnIndex, Error> {
        loop {
            let mut potion_options = Vec::new();
            for (index, maybe_potion) in self.state.potions().iter().enumerate() {
                if let Some(potion) = maybe_potion {
                    potion_options.push(PotionAction::Discard(index, *potion));
                    if potion.can_drink_anywhere() {
                        potion_options.push(PotionAction::Drink(index, *potion));
                    }
                }
            }
            match self
                .comms
                .choose_main_screen_action(climb_options, &potion_options)?
            {
                MainScreenAction::ClimbFloor(index) => return Ok(index),
                MainScreenAction::Potion(PotionAction::Discard(index, _)) => {
                    self.state.discard_potion(index);
                    self.comms.send_potions(self.state.potions())?;
                }
                MainScreenAction::Potion(PotionAction::Drink(index, potion)) => {
                    self.state.discard_potion(index);
                    self.comms.send_potions(self.state.potions())?;
                    self.consume_potion(potion)?;
                }
            }
        }
    }

    fn consume_potion(&mut self, potion: Potion) -> Result<(), Error> {
        match potion {
            Potion::BloodPotion => self.increase_hp(self.hp_max() / 5),
            Potion::EntropicBrew => todo!(),
            Potion::FruitJuice => self.increase_hp_max(5),
            _ => unreachable!(),
        }
    }

    pub fn choose_potions_to_obtain(
        &mut self,
        potions: &[Potion],
        mut choice_count: usize,
    ) -> Result<(), Error> {
        let mut potion_vec = potions.to_vec();
        while self.state.has_potion_slot_available() && !potion_vec.is_empty() && choice_count > 0 {
            if let Some(potion) = self
                .comms
                .choose_potion_to_obtain(&potion_vec, choice_count == 1)?
            {
                self.state.obtain_potion(potion);
                self.comms.send_potions(self.state.potions())?;
                let potion_index = potion_vec
                    .iter()
                    .position(|p| *p == potion)
                    .expect("Potion not found");
                potion_vec.remove(potion_index);
                choice_count -= 1;
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn choose_card_to_remove(&mut self) -> Result<(), Error> {
        let deck_index = self.comms.choose_card_to_remove(self.state.deck())?;
        let card = self.state.remove_card(deck_index);
        self.comms.send_card_removed(card)?;
        self.comms.send_deck(self.state.deck())
    }

    pub fn start_combat(&mut self, shuffle_rng: StsRandom) -> CombatController {
        CombatController::new(shuffle_rng, &mut self.state, &mut self.comms)
    }
}

impl<'a> CombatController<'a> {
    pub fn new(shuffle_rng: StsRandom, state: &'a mut PlayerState, comms: &'a mut Comms) -> Self {
        let combat_state = CombatState::new(state.deck(), shuffle_rng);
        Self {
            combat_state,
            card_just_played: None,
            state,
            comms,
        }
    }

    pub fn hp(&self) -> Hp {
        self.state.hp()
    }

    pub fn start_combat(&self) -> Result<(), Error> {
        self.comms.send_starting_combat()
    }

    pub fn start_turn(&mut self) -> Result<(), Error> {
        // Reset energy
        // TODO: energy conservation
        self.combat_state.energy = 3;

        // Draw cards
        self.draw_cards()?;

        // Set block to 0
        if self.combat_state.block > 0 {
            self.combat_state.block = 0;
            self.comms.send_block(self.combat_state.block)?;
        }

        // TODO: Apply other start-of-turn effects
        Ok(())
    }

    pub fn end_turn(&mut self) -> Result<(), Error> {
        self.discard_hand()?;

        // Tick down debuffs
        for (_, stacks) in self.combat_state.debuffs.iter_mut() {
            *stacks = stacks.saturating_sub(1);
        }
        self.combat_state.debuffs.retain(|(_, stacks)| *stacks > 0);
        self.comms.send_debuffs(&self.combat_state.debuffs)?;

        // TODO: Apply other end-of-turn effects
        Ok(())
    }

    pub fn end_combat(self) -> Result<(), Error> {
        self.comms.send_ending_combat()
    }

    pub fn take_damage(&mut self, amount: AttackDamage) -> Result<(), Error> {
        if amount <= self.combat_state.block {
            self.combat_state.block -= amount;
            self.comms.send_damage_blocked(amount)?;
            self.comms.send_block_lost(amount)?;
            self.comms.send_block(self.combat_state.block)
        } else if self.combat_state.block > 0 {
            let remaining_damage = amount - self.combat_state.block;
            self.comms.send_damage_blocked(amount)?;
            self.comms.send_block_lost(self.combat_state.block)?;
            self.combat_state.block = 0;
            self.comms.send_block(0)?;
            self.comms.send_damage_taken(remaining_damage)?;
            self.state.decrease_hp(remaining_damage);
            self.comms.send_health_changed(self.state.health())
        } else {
            self.state.decrease_hp(amount);
            self.comms.send_damage_taken(amount)?;
            self.comms.send_health_changed(self.state.health())
        }
    }

    pub fn draw_cards(&mut self) -> Result<(), Error> {
        // Draw new cards
        let draw_count = 5;
        for i in 0..draw_count {
            if let Some(card) = self.combat_state.draw_pile.pop() {
                self.combat_state.hand.push(card);
                self.comms.send_card_drawn(i, card)?;
            } else {
                // Shuffle discard pile into draw pile
                self.comms.send_shuffling_discard_to_draw()?;
                println!(
                    "About to shuffle discard pile into draw pile, shuffle_rng: {}",
                    self.combat_state.shuffle_rng.get_counter()
                );
                self.combat_state.shuffle();
                println!(
                    "About to shuffle discard pile into draw pile, shuffle_rng: {}",
                    self.combat_state.shuffle_rng.get_counter()
                );
                self.combat_state
                    .draw_pile
                    .append(&mut self.combat_state.discard_pile);
                if let Some(card) = self.combat_state.draw_pile.pop() {
                    self.combat_state.hand.push(card);
                    self.comms.send_card_drawn(i, card)?;
                }
            }
        }
        Ok(())
    }

    pub fn add_to_discard_pile(&mut self, cards: &[Card]) -> Result<(), Error> {
        self.combat_state.discard_pile.extend_from_slice(cards);
        self.comms.send_add_to_discard_pile(cards)
    }

    fn discard_hand(&mut self) -> Result<(), Error> {
        // Emulating the game's behavior
        while let Some(card) = self.combat_state.hand.pop() {
            self.combat_state.discard_pile.push(card);
        }
        self.comms.send_hand_discarded()
    }

    pub fn enemy_died(&self, index: EnemyIndex, enemy_type: EnemyType) -> Result<(), Error> {
        self.comms.send_enemy_died(index, enemy_type)
    }

    pub fn choose_next_action(
        &mut self,
        enemies: &[Option<EnemyStatus>],
    ) -> Result<CombatAction, Error> {
        // TODO: drink a potion, discard a potion
        // TODO: check for unwinnable situations
        // TODO: Intent
        self.comms.send_enemy_statuses(enemies)?;
        self.comms.send_energy(self.combat_state.energy)?;
        let playable_cards = self
            .combat_state
            .hand
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(hand_index, card)| {
                if card.cost() > self.combat_state.energy {
                    None
                } else {
                    Some((hand_index, card))
                }
            })
            .collect::<Vec<_>>();

        match self.comms.choose_card_to_play(&playable_cards)? {
            Some(hand_index) => {
                self.card_just_played = Some(hand_index);
                let card = self.combat_state.hand[hand_index];
                let card_details = CardDetails::for_card(card);
                if card_details.requires_target {
                    let enemy_index = self.comms.choose_enemy_to_target(enemies)?;
                    Ok(CombatAction::PlayCardAgainstEnemy(
                        card,
                        card_details,
                        enemy_index,
                    ))
                } else {
                    Ok(CombatAction::PlayCard(card, card_details))
                }
            }
            None => Ok(CombatAction::EndTurn),
        }
    }

    pub fn dispose_card_just_played(&mut self) -> Result<(), Error> {
        if let Some(hand_index) = self.card_just_played {
            let card = self.combat_state.hand.remove(hand_index);
            self.combat_state.energy = self.combat_state.energy.saturating_sub(card.cost());
            if card.exhausts() {
                self.combat_state.exhaust_pile.push(card);
                self.comms.send_card_exhausted(hand_index, card)?;
            } else {
                self.combat_state.discard_pile.push(card);
                self.comms.send_card_discarded(hand_index, card)?;
            }
        }
        Ok(())
    }

    pub fn apply_debuff(&mut self, debuff: Debuff, stacks: StackCount) -> Result<(), Error> {
        if let Some((_, c)) = self
            .combat_state
            .debuffs
            .iter_mut()
            .find(|(d, _)| *d == debuff)
        {
            *c += stacks;
        } else {
            self.combat_state.debuffs.push((debuff, stacks));
        }
        self.comms.send_debuffs(&self.combat_state.debuffs)
    }

    pub fn gain_block(&mut self, amount: Block) -> Result<(), Error> {
        self.comms.send_block_gained(amount)?;
        self.combat_state.block = self.combat_state.block.saturating_add(amount);
        self.comms.send_block(self.combat_state.block)
    }

    pub fn update_enemy_status(&self, index: EnemyIndex, status: EnemyStatus) -> Result<(), Error> {
        self.comms.send_enemy_status(index, status)
    }

    pub fn has_debuff(&self, debuff: Debuff) -> bool {
        self.combat_state.has_debuff(debuff)
    }

    pub fn is_dead(&self) -> bool {
        self.hp() == 0
    }

    pub fn is_frail(&self) -> bool {
        self.has_debuff(Debuff::Frail)
    }

    pub fn is_vulnerable(&self) -> bool {
        self.has_debuff(Debuff::Vulnerable)
    }

    pub fn is_weak(&self) -> bool {
        self.has_debuff(Debuff::Weak)
    }

    pub fn send_enemy_died(&self, index: EnemyIndex, enemy_type: EnemyType) -> Result<(), Error> {
        self.comms.send_enemy_died(index, enemy_type)
    }

    pub fn send_enemy_status(&self, index: EnemyIndex, status: EnemyStatus) -> Result<(), Error> {
        self.comms.send_enemy_status(index, status)
    }
}
