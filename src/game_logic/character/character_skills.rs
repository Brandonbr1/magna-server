use crate::game_logic::battle::battle_action::BattleAction;

/// Represents a single character skill.
pub struct CharacterSkillData {
    pub name: &'static str,
    pub description: &'static str,
    pub actions: &'static [BattleAction],
    pub cooldown: u32,
}

pub static ALEXIEL_SKILL_1: CharacterSkillData = CharacterSkillData {
    name: "Mighty Sword",
    description: "400% damage to a foe.",
    actions: &[BattleAction::DealDamageSingle { multiplier: 4.0 }],
    cooldown: 3,
};

pub static ALEXIEL_SKILL_2: CharacterSkillData = CharacterSkillData {
    name: "Flurry Rush",
    description: "6-hit, 150% damage to a foe.",
    actions: &[BattleAction::DealDamageSingle { multiplier: 1.5 }; 6],
    cooldown: 5,
};

pub static ALEXIEL_SKILL_3: CharacterSkillData = CharacterSkillData {
    name: "New Life",
    description: "Restores 30% of all allies' HP.",
    actions: &[BattleAction::HealAll { multiplier: 0.3 }],
    cooldown: 6,
};
