use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    components::skill::{Healing, SkillComponent, SkillIdentity, Zoltraak},
    entities::player::Player,
};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(Player {
        id: 1,
        name: "Rayato159".to_string(),
        health: 10,
        mana: 100,
        skills: vec![
            SkillComponent {
                skill: Arc::new(Healing {
                    mana: 10,
                    health: 20,
                }),
            },
            SkillComponent {
                skill: Arc::new(Zoltraak {
                    mana: 20,
                    damage: 30,
                }),
            },
        ],
    });
}

pub fn healing(keyboard_input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Player>) {
    for mut p in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyH) {
            for s in p.skills.clone().iter() {
                if s.skill.get_identity() == SkillIdentity::Healing {
                    // Check if the player has enough mana to use the skill
                    if p.mana < s.skill.mana_cost() {
                        error!("Not enough mana to use Healing");
                        return;
                    }

                    // Check if the player's health is full
                    if p.health == 100 {
                        error!("Health is full");
                        return;
                    }

                    // Activate the skill
                    // Increase the player's health
                    if p.health + s.skill.get_value() > 100 {
                        p.health = 100;
                    } else {
                        p.health += s.skill.get_value();
                    }

                    // Decrease the player's mana
                    p.mana -= s.skill.mana_cost();

                    info!("Player's mana: {}", p.mana);
                    info!("Player's health: {}", p.health);

                    return;
                }
            }
        }
    }
}

pub fn magic_attack(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    for mut p in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for s in p.skills.clone().iter() {
                if s.skill.get_identity() == SkillIdentity::Zoltraak {
                    // Check if the player has enough mana to use the skill
                    if p.mana < s.skill.mana_cost() {
                        error!("Not enough mana to use Zoltraak");
                        return;
                    }

                    // Activate the skill
                    // Decrease the player's health
                    p.mana -= s.skill.mana_cost();

                    info!("Player's mana: {}", p.mana);
                    info!("Damage dealt by {}: {}", p.name, s.skill.get_value());

                    return;
                }
            }
        }
    }
}
