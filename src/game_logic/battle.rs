pub mod battle_action;
pub mod battle_state;
pub mod stats;

/// Represents an entity (character or enemy) in battle.
pub trait BattleEntity {
    /// Gets the ATK of the entity.
    fn atk(&self) -> f64;
    /// Gets the HP of the entity.
    fn hp(&self) -> f64;
    /// Sets the HP of the entity.
    fn set_hp(&mut self, hp: f64);

    /// Calculates the damage that would be done by the entity given a specified multiplier.
    fn damage(&self, multiplier: f64) -> f64 {
        self.atk() * multiplier /* * buffs */
    }

    /// Calculates the healing that would be provided by the entity given a specified multiplier.
    fn heal(&self, multiplier: f64) -> f64 {
        self.hp() * multiplier
    }

    /// Deals a specified amount of damage to the entity.
    fn receive_damage(&mut self, damage: f64) {
        self.set_hp(if damage > self.hp() {
            0.0
        } else {
            self.hp() - damage
        })
    }

    /// Gives a specified amount of healing to the entity.
    fn receive_heal(&mut self, heal: f64) {
        let hp = self.hp() + heal;
        self.set_hp(if hp > self.hp() { self.hp() } else { hp })
    }
}
