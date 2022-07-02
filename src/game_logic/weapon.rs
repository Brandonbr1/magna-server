use std::collections::HashMap;

use self::weapon_data::{WeaponData, WEAPON_DATA_DEPOT};

use super::battle::stats::Stat;

mod weapon_data;
mod weapon_skills;

/// Represents a weapon in the player's inventory.
pub struct Weapon {
    pub hp: f64,
    pub atk: f64,

    pub data: &'static WeaponData,
}

impl Weapon {
    /// Creates a ```Weapon``` given an identifier string.
    ///
    /// ## Returns
    /// Returns [Some(Weapon)] if the identifier maps to am existing weapon, otherwise returns [None].
    fn new(id: &str) -> Option<Self> {
        let &data = WEAPON_DATA_DEPOT.get(id)?;

        Some(Self {
            hp: data.base_hp,
            atk: data.base_atk,
            data,
        })
    }
}

/// Represents the grid of weapons in a party setup.
pub struct WeaponGrid<'a> {
    pub weapons: Vec<&'a Weapon>,

    pub total_hp: f64,
    pub total_atk: f64,
    pub all_modifiers: HashMap<Stat, f64>,
}

impl<'a> From<Vec<&'a Weapon>> for WeaponGrid<'a> {
    fn from(weapons: Vec<&'a Weapon>) -> Self {
        let mut grid = Self {
            total_hp: WeaponGrid::get_total_hp(&weapons),
            total_atk: WeaponGrid::get_total_atk(&weapons),
            all_modifiers: HashMap::new(),
            weapons: Vec::new(),
        };
        grid.update_all_modifiers(&weapons);
        grid.weapons = weapons;

        grid
    }
}

impl<'a> WeaponGrid<'a> {
    /// Creates an empty ```WeaponGrid```.
    fn new() -> Self {
        Self {
            weapons: Vec::new(),
            total_hp: 0.0,
            total_atk: 0.0,
            all_modifiers: HashMap::new(),
        }
    }

    /// Adds a weapon to the grid.
    fn add(&mut self, weapon: &'a Weapon) {
        self.weapons.push(weapon);
        self.total_atk += weapon.atk;
        self.total_hp += weapon.hp;
        self.add_modifiers(weapon);
    }

    /// Removes a weapon from the grid.
    fn remove(&mut self, index: usize) {
        let weapon = self.weapons.remove(index);
        self.total_atk -= weapon.atk;
        self.total_hp -= weapon.hp;
        self.remove_modifiers(weapon);
    }

    /// Replaces a weapon in the grid.
    fn replace(&mut self, index: usize, new_weapon: &'a Weapon) {
        let old_weapon = std::mem::replace(&mut self.weapons[index], new_weapon);

        self.total_atk -= old_weapon.atk;
        self.total_hp -= old_weapon.hp;
        self.remove_modifiers(old_weapon);

        self.total_atk += new_weapon.atk;
        self.total_hp += new_weapon.hp;
        self.add_modifiers(new_weapon);
    }

    /// Calculates the sum of HP from the grid's weapons.
    fn get_total_hp(weapons: &Vec<&Weapon>) -> f64 {
        weapons.iter().fold(0.0, |acc, &weapon| acc + weapon.hp)
    }

    /// Calculates the sum of ATK from the grid's weapons.
    fn get_total_atk(weapons: &Vec<&Weapon>) -> f64 {
        weapons.iter().fold(0.0, |acc, &weapon| acc + weapon.atk)
    }

    /// Updates a [HashMap] of all the ```StatModifiers``` from the grid's weapons' skills.
    fn update_all_modifiers(&mut self, weapons: &Vec<&Weapon>) {
        self.all_modifiers = HashMap::new();
        for &weapon in weapons {
            self.add_modifiers(weapon);
        }
    }

    /// Adds a weapon's ```StatModifiers```s to ```all_modifiers```.
    fn add_modifiers(&mut self, weapon: &Weapon) {
        for &skill in weapon.data.skills {
            for stat_modifier in skill.modifiers {
                let modifier = self.all_modifiers.entry(stat_modifier.stat).or_default();
                *modifier += stat_modifier.modifier;
            }
        }
    }

    /// Removes a weapon's ```StatModifiers```s to ```all_modifiers```.
    fn remove_modifiers(&mut self, weapon: &Weapon) {
        for &skill in weapon.data.skills {
            for stat_modifier in skill.modifiers {
                let modifier = self.all_modifiers.entry(stat_modifier.stat).or_default();
                *modifier -= stat_modifier.modifier;
            }
        }
    }
}
