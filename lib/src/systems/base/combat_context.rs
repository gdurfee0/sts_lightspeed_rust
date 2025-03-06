use crate::components::{EffectQueue, Interaction, PlayerCombatState, PlayerPersistentState};
use crate::data::Encounter;
use crate::systems::rng::{Seed, StsRandom};
use crate::types::EnemyIndex;

use super::enemy_party::EnemyParty;

#[derive(Debug)]
pub struct CombatContext<'a, I: Interaction> {
    pub comms: &'a I,
    pub pcs: PlayerCombatState<'a>,
    pub enemy_party: EnemyParty,
    pub maybe_enemy_index: Option<EnemyIndex>,
    pub effect_queue: EffectQueue,
    pub misc_rng: &'a mut StsRandom,
    pub enemy_rng: StsRandom,
    pub shuffle_rng: StsRandom,
    pub card_randomizer_rng: StsRandom,
}

impl<'a, I: Interaction> CombatContext<'a, I> {
    pub fn new(
        comms: &'a I,
        seed_for_floor: Seed,
        encounter: Encounter,
        pps: &'a mut PlayerPersistentState,
        misc_rng: &'a mut StsRandom,
    ) -> Self {
        let pcs = PlayerCombatState::new(pps);
        let mut enemy_rng = StsRandom::from(seed_for_floor);
        let enemy_party = EnemyParty::generate(seed_for_floor, encounter, &mut enemy_rng, misc_rng);
        let maybe_enemy_index = None;
        let effect_queue = EffectQueue::new();
        let shuffle_rng = StsRandom::from(seed_for_floor);
        let card_randomizer_rng = StsRandom::from(seed_for_floor);
        Self {
            comms,
            pcs,
            enemy_party,
            maybe_enemy_index,
            effect_queue,
            misc_rng,
            enemy_rng,
            shuffle_rng,
            card_randomizer_rng,
        }
    }

    /// Returns true iff either the player is dead or all enemies are dead.
    pub fn combat_should_end(&self) -> bool {
        self.pcs.pps.hp == 0 || self.enemy_party.0.iter().all(|enemy| enemy.is_none())
    }
}
