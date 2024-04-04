use bevy::prelude::*;
use bevy::window::WindowTheme;

mod system;
mod components;
mod player1;
mod player2;
mod platform;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Multiplayer Test".into(),
                resolution: (1000., 700.).into(),
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup, (
                system::setup,
                player1::setup,
                player2::setup,
                platform::setup,
            )
        )
        .add_systems(
            Update, (
                system::update_game_over,
                system::update_gravity,
                player1::update,
                player2::update,
                platform::update_spawn,
                platform::update_move,
            )
        )
        .run();
}
