use self::{enemy_data::ENEMY_DATA_DEPOT, enemy_skills::EnemySkillData};

use super::battle::BattleEntity;

mod enemy_data;
mod enemy_skills;

/// Represents an enemy's state during a battle.
pub struct Enemy {
    pub max_hp: f64,
    pub curr_hp: f64,
    pub atk: f64,

    // Copied from EnemyData
    pub name: &'static str,
    pub base_hp: f64,
    pub base_atk: f64,
    pub skill_normal: &'static EnemySkillData,
    pub skill_special: &'static EnemySkillData,
    pub special_cooldown: u32,
}

impl Enemy {
    /// Creates an ```Enemy``` given an identifier string.
    ///
    /// ## Returns
    /// Returns [Some(Enemy)] if the identifier maps to a playable character, otherwise returns [None].
    fn new(id: &str) -> Option<Self> {
        let &data = ENEMY_DATA_DEPOT.get(id)?;

        Some(Self {
            max_hp: data.base_hp,
            curr_hp: data.base_hp,
            atk: data.base_atk,
            name: data.name,
            base_hp: data.base_hp,
            base_atk: data.base_atk,
            skill_normal: data.skill_normal,
            skill_special: data.skill_special,
            special_cooldown: data.special_cooldown,
        })
    }
}

impl BattleEntity for Enemy {
    fn atk(&self) -> f64 {
        self.atk
    }

    fn hp(&self) -> f64 {
        self.curr_hp
    }

    fn set_hp(&mut self, hp: f64) {
        self.curr_hp = hp;
    }
}
