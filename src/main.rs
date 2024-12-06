use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use game_hacking_101_lab::systems::player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "wgpu=warn,bevy_ecs=info".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(640., 360.)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, player::spawn_player)
        .add_systems(Update, player::healing)
        .add_systems(Update, player::magic_attack)
        .run();
}
