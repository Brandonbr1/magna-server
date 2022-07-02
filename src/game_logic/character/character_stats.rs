use crate::game_logic::weapon::WeaponGrid;

/// Gets the total HP of a character.
/// Will take into account weapons in the future too.
pub fn get_total_hp(base_hp: f64, grid: &WeaponGrid) -> f64 {
    base_hp + grid.total_hp
}

/// Gets the total ATK of a character.
/// Will take into account weapons in the future too.
pub fn get_total_atk(base_atk: f64, grid: &WeaponGrid) -> f64 {
    base_atk + grid.total_atk
}
