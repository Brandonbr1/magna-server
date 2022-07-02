/// Represents an atomic action that a player character or enemy can execute during battle.
#[derive(Clone, Copy)]
pub enum BattleAction {
    /// Deals damage to a single target based on a specified multiplier.
    DealDamageSingle { multiplier: f64 },

    /// Deals damage to a random target based on a specified multiplier.
    DealDamageRandom { multiplier: f64 },

    /// Heals all allies based on a specified multiplier.
    HealAll { multiplier: f64 },
}

impl BattleAction {
    // pub fn execute(&self, state: BattleState) -> BattleState {}
}
