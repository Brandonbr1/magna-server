use phf::phf_map;

use super::character_skills::*;

/// Static map containing all character data.
///
/// ## Examples
/// ```
/// if let Some(&data) = CHARACTER_DATA_DEPOT.get("alexiel") {
///     // Do something with data
/// }
/// ```
pub static CHARACTER_DATA_DEPOT: phf::Map<&'static str, &'static CharacterData> = phf_map! {
    "alexiel" => &ALEXIEL,
};

/// Represents base character data.
pub struct CharacterData {
    pub name: &'static str,
    pub base_hp: f64,
    pub base_atk: f64,
    pub skills: &'static [&'static CharacterSkillData],
}

// impl CharacterData {
//     /// Gets a reference to the character's nth skill, or None if it does not exist.
//     pub fn get_skill(&self, n: usize) -> Option<&'static CharacterSkill> {
//         if let Some(&skill) = self.skills.get(n) {
//             Some(skill)
//         } else {
//             None
//         }
//     }
// }

static ALEXIEL: CharacterData = CharacterData {
    name: "Alexiel",
    base_hp: 100.0,
    base_atk: 20.0,
    skills: &[&ALEXIEL_SKILL_1, &ALEXIEL_SKILL_2, &ALEXIEL_SKILL_3],
};
