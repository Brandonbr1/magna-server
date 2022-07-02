use crate::game_logic::battle::stats::{Stat, StatModifier};

/// Represents a single weapon skill.
pub struct WeaponSkillData {
    pub name: &'static str,
    pub description: &'static str,
    pub modifiers: &'static [StatModifier],
}

pub static WEAPON_ATK_1: WeaponSkillData = WeaponSkillData {
    name: "Normal Might I",
    description: "10% boost to all allies' ATK.",
    modifiers: &[StatModifier {
        stat: Stat::ATTACK_PERCENT,
        modifier: 0.1,
    }],
};

pub static WEAPON_HP_1: WeaponSkillData = WeaponSkillData {
    name: "Normal Health I",
    description: "10% boost to all allies' HP.",
    modifiers: &[StatModifier {
        stat: Stat::HP_PERCENT,
        modifier: 0.1,
    }],
};
