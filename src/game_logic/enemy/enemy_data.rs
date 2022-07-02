use phf::phf_map;

use super::enemy_skills::*;

/// Static map containing all enemy data.
///
/// ## Examples
/// ```
/// if let Some(&data) = ENEMY_DATA_DEPOT.get("tiamat") {
///     // Do something with data
/// }
/// ```
pub static ENEMY_DATA_DEPOT: phf::Map<&'static str, &'static EnemyData> = phf_map! {
    "tiamat" => &TIAMAT,
};

/// Represents base enemy  data.
pub struct EnemyData {
    pub name: &'static str,
    pub base_hp: f64,
    pub base_atk: f64,
    pub skill_normal: &'static EnemySkillData,
    pub skill_special: &'static EnemySkillData,
    pub special_cooldown: u32,
}

static TIAMAT: EnemyData = EnemyData {
    name: "Tiamat",
    base_hp: 1000.0,
    base_atk: 10.0,
    skill_normal: &TIAMAT_SKILL_NORMAL,
    skill_special: &TIAMAT_SKILL_SPECIAL,
    special_cooldown: 3,
};
