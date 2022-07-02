use crate::game_logic::battle::battle_action::BattleAction;

/// Represents a single enemy skill.
pub struct EnemySkillData {
    pub name: &'static str,
    pub actions: &'static [BattleAction],
}

pub static TIAMAT_SKILL_NORMAL: EnemySkillData = EnemySkillData {
    name: "",
    actions: &[BattleAction::DealDamageRandom { multiplier: 1.0 }],
};

pub static TIAMAT_SKILL_SPECIAL: EnemySkillData = EnemySkillData {
    name: "Tornado Blitz",
    actions: &[BattleAction::DealDamageRandom { multiplier: 1.5 }; 3],
};
