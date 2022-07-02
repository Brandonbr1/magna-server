/// Enumerates all the stat properties of a character.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Stat {
    ATTACK,
    ATTACK_PERCENT,
    HP,
    HP_PERCENT,
}

/// Represents an atomic stat modifier for a character.
pub struct StatModifier {
    pub stat: Stat,
    pub modifier: f64,
}
