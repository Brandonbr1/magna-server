use rand::{prelude::SliceRandom, thread_rng};

use crate::game_logic::{character::CharacterBattle, enemy::Enemy};

use super::{battle_action::BattleAction, BattleEntity};

/// Represents the state of an ongoing battle.
pub struct BattleState {
    pub enemies: Vec<Enemy>,
    pub characters: Vec<CharacterBattle>,
}

impl BattleState {
    /// Performs an action, mutating the state.
    pub fn execute(
        &mut self,
        action: BattleAction,
        source: &dyn BattleEntity,
        targets: &mut [&mut dyn BattleEntity],
    ) {
        use BattleAction::*;

        match action {
            DealDamageSingle { multiplier } => {
                // Only process action if target[0] exists.
                if let [target] = targets {
                    target.receive_damage(source.damage(multiplier));
                }
            }

            DealDamageRandom { multiplier } => {
                if let Some(target) = targets.choose_mut(&mut thread_rng()) {
                    target.receive_damage(source.damage(multiplier));
                }
            }

            HealAll { multiplier } => {
                for target in targets {
                    target.receive_heal(source.heal(multiplier));
                }
            }
        }
    }
}
