use bevy::prelude::*;

use crate::components::skill::SkillComponent;

#[derive(Component, Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub health: i32,
    pub mana: i32,
    pub skills: Vec<SkillComponent>,
}
