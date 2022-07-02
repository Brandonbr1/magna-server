use phf::phf_map;

use super::weapon_skills::*;

/// Static map containing all weapon data.
///
/// ## Examples
/// ```
/// if let Some(&data) = WEAPON_DATA_DEPOT.get("mirror_blade") {
///     // Do something with data
/// }
/// ```
pub static WEAPON_DATA_DEPOT: phf::Map<&'static str, &'static WeaponData> = phf_map! {
    "mirror_blade" => &MIRROR_BLADE
};

/// Represents base weapon data.
pub struct WeaponData {
    pub name: &'static str,
    pub base_hp: f64,
    pub base_atk: f64,
    pub skills: &'static [&'static WeaponSkillData],
}

static MIRROR_BLADE: WeaponData = WeaponData {
    name: "Mirror Blade",
    base_hp: 30.0,
    base_atk: 20.0,
    skills: &[&WEAPON_ATK_1, &WEAPON_HP_1],
};
