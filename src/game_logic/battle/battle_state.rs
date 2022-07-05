use std::fmt::Display;

use anyhow::Context;
use rand::{prelude::SliceRandom, thread_rng};

use crate::game_logic::{character::CharacterBattle, enemy::Enemy};

use super::{battle_action::BattleAction, BattleEntity};

/// Represents the state of an ongoing battle.
pub struct BattleState {
    pub enemies: Vec<Box<Enemy>>,
    pub characters: Vec<Box<CharacterBattle>>,
}

impl BattleState {
    /// Performs an action, mutating the state.
    pub fn execute(
        &mut self,
        action: BattleAction,
        source_type: BattleEntityType,
        source_index: usize,
        target_type: BattleEntityType,
        target_index: usize,
    ) -> anyhow::Result<f64> {
        use BattleAction::*;

        let source = self
            .get_entity(source_type, source_index)
            .with_context(|| {
                format!(
                    "Failed to obtain source entity of type {} and index {}",
                    source_type, source_index
                )
            })?;

        match action {
            DealDamageSingle { multiplier } => {
                let damage = source.damage(multiplier);
                let target = self
                    .get_entity_mut(target_type, target_index)
                    .with_context(|| {
                        format!(
                            "Failed to obtain target entity of type {} and index {}",
                            target_type, target_index
                        )
                    })?;
                target.receive_damage(damage);
                Ok(damage)
            }

            DealDamageRandom { multiplier } => {
                let damage = source.damage(multiplier);
                let target = self.get_random_entity_mut(target_type).with_context(|| {
                    format!(
                        "Failed to obtain random target entity of type {}",
                        target_type
                    )
                })?;
                target.receive_damage(damage);
                Ok(damage)
            }

            HealAll { multiplier } => {
                let heal = source.heal(multiplier);
                for target in self.get_all_entities_mut(target_type) {
                    target.receive_heal(heal);
                }
                Ok(heal)
            }
        }
    }

    /// Returns a reference to a ```BattleEntity```.
    fn get_entity(
        &self,
        entity_type: BattleEntityType,
        entity_index: usize,
    ) -> Option<&dyn BattleEntity> {
        match entity_type {
            BattleEntityType::CharacterBattle => self
                .characters
                .get(entity_index)
                .map(|b| b.as_ref() as &dyn BattleEntity),

            BattleEntityType::Enemy => self
                .enemies
                .get(entity_index)
                .map(|b| b.as_ref() as &dyn BattleEntity),
        }
    }

    /// Returns a mutable reference to a ```BattleEntity```.
    fn get_entity_mut(
        &mut self,
        entity_type: BattleEntityType,
        entity_index: usize,
    ) -> Option<&mut dyn BattleEntity> {
        match entity_type {
            BattleEntityType::CharacterBattle => self
                .characters
                .get_mut(entity_index)
                .map(|b| b.as_mut() as &mut dyn BattleEntity),

            BattleEntityType::Enemy => self
                .enemies
                .get_mut(entity_index)
                .map(|b| b.as_mut() as &mut dyn BattleEntity),
        }
    }

    /// Returns a mutable reference to a random ```BattleEntity```.
    fn get_random_entity_mut(
        &mut self,
        entity_type: BattleEntityType,
    ) -> Option<&mut dyn BattleEntity> {
        match entity_type {
            BattleEntityType::CharacterBattle => self
                .characters
                .choose_mut(&mut thread_rng())
                .map(|b| b.as_mut() as &mut dyn BattleEntity),

            BattleEntityType::Enemy => self
                .enemies
                .choose_mut(&mut thread_rng())
                .map(|b| b.as_mut() as &mut dyn BattleEntity),
        }
    }

    /// Returns the references to all the ```BattleEntity```s on a specified sides.
    fn get_all_entities_mut(
        &mut self,
        entity_type: BattleEntityType,
    ) -> Vec<&mut dyn BattleEntity> {
        match entity_type {
            BattleEntityType::CharacterBattle => self
                .characters
                .iter_mut()
                .map(|b| b.as_mut() as &mut dyn BattleEntity)
                .collect(),

            BattleEntityType::Enemy => self
                .enemies
                .iter_mut()
                .map(|b| b.as_mut() as &mut dyn BattleEntity)
                .collect(),
        }
    }
}

/// Represents the type of ```BattleEntity``` (either Character or Enemy).
/// Used for determining which side is initiating and receiving the attack or skill.
#[derive(Clone, Copy)]
pub enum BattleEntityType {
    CharacterBattle,
    Enemy,
}

impl Display for BattleEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterBattle => f.write_str("Character"),
            Self::Enemy => f.write_str("Enemy"),
        }
    }
}
