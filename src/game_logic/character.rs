use self::{
    character_data::{CharacterData, CHARACTER_DATA_DEPOT},
    character_skills::CharacterSkillData,
    character_stats::{get_total_atk, get_total_hp},
};

use super::{
    battle::{battle_action::BattleAction, BattleEntity},
    weapon::WeaponGrid,
};

mod character_data;
mod character_skills;
mod character_stats;

/// Represents a character's state during a battle.
pub struct CharacterBattle {
    pub max_hp: f64,
    pub curr_hp: f64,
    pub atk: f64,

    pub skills: Vec<CharacterSkill>,

    // Copied from CharacterData
    pub name: &'static str,
    pub base_hp: f64,
    pub base_atk: f64,
}

impl From<&Character> for CharacterBattle {
    fn from(character: &Character) -> Self {
        let data = character.data;
        Self {
            max_hp: data.base_hp,
            curr_hp: data.base_hp,
            atk: data.base_atk,
            skills: data
                .skills
                .iter()
                .map(|&skill| CharacterSkill::from(skill))
                .collect(),
            name: data.name,
            base_hp: data.base_hp,
            base_atk: data.base_atk,
        }
    }
}

impl CharacterBattle {
    /// Creates a ```BattleCharacter``` from an existing ```Character``` and ```WeaponGrid```.
    fn create(character: &Character, grid: &WeaponGrid) -> Self {
        let mut character = CharacterBattle::from(character);

        let hp = get_total_hp(character.base_hp, grid);
        let atk = get_total_atk(character.base_atk, grid);
        character.max_hp = hp;
        character.curr_hp = hp;
        character.atk = atk;

        character
    }
}

impl BattleEntity for CharacterBattle {
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

/// Represents a character in the player's inventory.
pub struct Character {
    pub data: &'static CharacterData,
}

impl Character {
    /// Creates a ```Character``` given an identifier string.
    ///
    /// ## Returns
    /// Returns [Some(Character)] if the identifier maps to a playable character, otherwise returns [None].
    fn new(id: &str) -> Option<Self> {
        let &data = CHARACTER_DATA_DEPOT.get(id)?;

        Some(Self { data })
    }
}

/// Represents a character's skill.
pub struct CharacterSkill {
    pub curr_cooldown: u32,

    // Copied from CharacterSkillData
    pub name: &'static str,
    pub description: &'static str,
    pub actions: &'static [BattleAction],
    pub max_cooldown: u32,
}

impl From<&CharacterSkillData> for CharacterSkill {
    fn from(data: &CharacterSkillData) -> Self {
        Self {
            curr_cooldown: data.cooldown,
            name: data.name,
            description: data.description,
            actions: data.actions,
            max_cooldown: data.cooldown,
        }
    }
}
