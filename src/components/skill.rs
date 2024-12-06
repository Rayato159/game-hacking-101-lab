use std::sync::Arc;

use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum SkillIdentity {
    Healing,
    Zoltraak,
}

pub trait Skill: Send + Sync + 'static {
    fn get_identity(&self) -> SkillIdentity;
    fn get_value(&self) -> i32;
    fn mana_cost(&self) -> i32;
}

#[derive(Component, Clone)]
pub struct SkillComponent {
    pub skill: Arc<dyn Skill>,
}

#[derive(Component, Clone, PartialEq)]
pub struct Healing {
    pub mana: i32,
    pub health: i32,
}

impl Skill for Healing {
    fn get_identity(&self) -> SkillIdentity {
        SkillIdentity::Healing
    }

    fn get_value(&self) -> i32 {
        self.health
    }

    fn mana_cost(&self) -> i32 {
        self.mana
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct Zoltraak {
    pub mana: i32,
    pub damage: i32,
}

impl Skill for Zoltraak {
    fn get_identity(&self) -> SkillIdentity {
        SkillIdentity::Zoltraak
    }

    fn get_value(&self) -> i32 {
        self.damage
    }

    fn mana_cost(&self) -> i32 {
        self.mana
    }
}
