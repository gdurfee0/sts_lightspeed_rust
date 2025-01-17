use crate::data::{Card, Character, Potion, Relic};
use crate::{
    Block, Buff, Debuff, DeckIndex, Energy, Gold, Health, Hp, HpMax, PotionIndex, StackCount,
    StsRandom,
};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Mostly a dumb container.
#[derive(Debug)]
pub struct PlayerState {
    // State that persists outside of combat
    hp: Hp,
    hp_max: HpMax,
    gold: Gold,
    relics: Vec<Relic>,
    deck: Vec<Card>,
    potions: Vec<Option<Potion>>,
}

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.  TODO: lock down field visibility
#[derive(Debug)]
pub struct CombatState {
    shuffle_rng: StsRandom,
    pub energy: Energy,
    pub block: Block,
    pub buffs: Vec<(Buff, StackCount)>,
    pub debuffs: Vec<(Debuff, StackCount)>,
    pub hand: Vec<Card>,
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub exhaust_pile: Vec<Card>,
}

impl PlayerState {
    pub fn new(character: &'static Character) -> Self {
        let relics = vec![character.starting_relic];
        let deck = character.starting_deck.to_vec();
        let potions = [None; 3].to_vec();
        Self {
            hp: character.starting_hp,
            hp_max: character.starting_hp,
            gold: 99,
            relics,
            deck,
            potions,
        }
    }

    pub fn health(&self) -> Health {
        (self.hp, self.hp_max)
    }

    pub fn hp(&self) -> Hp {
        self.hp
    }

    pub fn hp_max(&self) -> HpMax {
        self.hp_max
    }

    pub fn gold(&self) -> Gold {
        self.gold
    }

    pub fn relics(&self) -> &[Relic] {
        &self.relics
    }

    pub fn obtain_relic(&mut self, relic: Relic) {
        self.relics.push(relic);
    }

    pub fn deck(&self) -> &[Card] {
        &self.deck
    }

    pub fn obtain_card(&mut self, card: Card) {
        self.deck.push(card);
    }

    pub fn remove_card(&mut self, deck_index: DeckIndex) -> Card {
        self.deck.remove(deck_index)
    }

    pub fn discard_potion(&mut self, index: PotionIndex) {
        self.potions[index] = None;
    }

    pub fn potions(&self) -> &[Option<Potion>] {
        &self.potions
    }

    pub fn has_potion_slot_available(&self) -> bool {
        self.potions.iter().any(|potion| potion.is_none())
    }

    pub fn gain_potion_slot(&mut self) {
        self.potions.push(None);
    }

    pub fn obtain_potion(&mut self, potion: Potion) {
        if let Some(slot) = self.potions.iter_mut().find(|potion| potion.is_none()) {
            *slot = Some(potion);
        }
    }

    pub fn increase_hp(&mut self, amount: Hp) {
        self.hp = self.hp.saturating_add(amount).min(self.hp_max);
    }

    pub fn decrease_hp(&mut self, amount: Hp) {
        self.hp = self.hp.saturating_sub(amount);
    }

    pub fn increase_hp_max(&mut self, amount: HpMax) {
        self.hp_max = self.hp_max.saturating_add(amount);
        self.hp = self.hp.saturating_add(amount);
    }

    pub fn decrease_hp_max(&mut self, amount: HpMax) {
        self.hp_max = self.hp_max.saturating_sub(amount);
        self.hp = self.hp.min(self.hp_max);
    }

    pub fn increase_gold(&mut self, amount: Gold) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub fn decrease_gold(&mut self, amount: Gold) {
        self.gold = self.gold.saturating_sub(amount);
    }
}

impl CombatState {
    pub fn new(deck: &[Card], mut shuffle_rng: StsRandom) -> Self {
        let hand = Vec::new();
        let mut draw_pile = deck.to_vec();
        shuffle_rng.java_compat_shuffle(&mut draw_pile);
        let discard_pile = Vec::new();
        let exhaust_pile = Vec::new();
        let debuffs = Vec::new();
        Self {
            shuffle_rng,
            energy: 3,
            block: 0,
            buffs: Vec::new(),
            debuffs,
            hand,
            draw_pile,
            discard_pile,
            exhaust_pile,
        }
    }

    pub fn shuffle(&mut self) {
        self.shuffle_rng.java_compat_shuffle(&mut self.draw_pile);
    }

    pub fn has_debuff(&self, debuff: Debuff) -> bool {
        self.debuffs.iter().any(|(d, _)| *d == debuff)
    }
}

#[cfg(test)]
mod test {
    use crate::IRONCLAD;

    use super::*;

    #[test]
    fn test_increase_hp() {
        let mut state = PlayerState::new(IRONCLAD);
        state.hp = 65;
        state.increase_hp(10);
        assert_eq!(state.hp(), 75);
        state.increase_hp(10);
        assert_eq!(state.hp(), 80);
    }

    #[test]
    fn test_decrease_hp() {
        let mut state = PlayerState::new(IRONCLAD);
        state.hp = 15;
        state.decrease_hp(10);
        assert_eq!(state.hp(), 5);
        state.decrease_hp(10);
        assert_eq!(state.hp(), 0);
    }

    #[test]
    fn test_increase_hp_max() {
        let mut state = PlayerState::new(IRONCLAD);
        state.increase_hp_max(10);
        assert_eq!(state.hp_max(), 90);
        assert_eq!(state.hp(), 90);
    }

    #[test]
    fn test_decrease_hp_max() {
        let mut state = PlayerState::new(IRONCLAD);
        state.decrease_hp_max(10);
        assert_eq!(state.hp_max(), 70);
        assert_eq!(state.hp(), 70);
    }

    #[test]
    fn test_increase_gold() {
        let mut state = PlayerState::new(IRONCLAD);
        state.increase_gold(10);
        assert_eq!(state.gold(), 109);
        state.increase_gold(10);
        assert_eq!(state.gold(), 119);
    }

    #[test]
    fn test_decrease_gold() {
        let mut state = PlayerState::new(IRONCLAD);
        state.decrease_gold(10);
        assert_eq!(state.gold(), 89);
        state.decrease_gold(100);
        assert_eq!(state.gold(), 0);
    }

    #[test]
    fn test_obtain_relic() {
        let mut state = PlayerState::new(IRONCLAD);
        state.obtain_relic(Relic::CaptainsWheel);
        assert_eq!(state.relics(), &[Relic::BurningBlood, Relic::CaptainsWheel]);
        state.obtain_relic(Relic::Calipers);
        assert_eq!(
            state.relics(),
            &[Relic::BurningBlood, Relic::CaptainsWheel, Relic::Calipers]
        );
    }

    #[test]
    fn test_obtain_potion() {
        let mut state = PlayerState::new(IRONCLAD);
        assert!(state.has_potion_slot_available());
        state.obtain_potion(Potion::BlockPotion);
        assert!(state.has_potion_slot_available());
        assert_eq!(state.potions(), &[Some(Potion::BlockPotion), None, None]);
        state.obtain_potion(Potion::BlessingOfTheForge);
        assert!(state.has_potion_slot_available());
        assert_eq!(
            state.potions(),
            &[
                Some(Potion::BlockPotion),
                Some(Potion::BlessingOfTheForge),
                None
            ]
        );
        state.obtain_potion(Potion::CultistPotion);
        assert!(!state.has_potion_slot_available());
        assert_eq!(
            state.potions(),
            &[
                Some(Potion::BlockPotion),
                Some(Potion::BlessingOfTheForge),
                Some(Potion::CultistPotion)
            ]
        );

        let mut state = PlayerState::new(IRONCLAD);
        state.potions = vec![None, Some(Potion::Ambrosia), None];
        assert!(state.has_potion_slot_available());
        state.obtain_potion(Potion::BlockPotion);
        assert!(state.has_potion_slot_available());
        assert_eq!(
            state.potions(),
            &[Some(Potion::BlockPotion), Some(Potion::Ambrosia), None]
        );
        state.obtain_potion(Potion::BlessingOfTheForge);
        assert!(!state.has_potion_slot_available());
        assert_eq!(
            state.potions(),
            &[
                Some(Potion::BlockPotion),
                Some(Potion::Ambrosia),
                Some(Potion::BlessingOfTheForge),
            ]
        );
        assert!(!state.has_potion_slot_available());
        state.obtain_potion(Potion::CultistPotion);
        assert_eq!(
            state.potions(),
            &[
                Some(Potion::BlockPotion),
                Some(Potion::Ambrosia),
                Some(Potion::BlessingOfTheForge),
            ]
        );
    }
}
