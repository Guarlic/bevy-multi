use crate::components::*;
use crate::components::Direction;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(30., 30.)),
            ..default()
        },
        texture: asset_server.load("player2.png"),
        transform: Transform::from_xyz(-100., 0., 0.),
        ..default()
    };

    commands.spawn((player, Player, Player2));
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player2>>,
    mut commands: Commands
) {
    for mut player_transform in player_query.iter_mut() {
        let movements = [
            (KeyCode::W, Vec3::new(0., 1., 0.)),
            (KeyCode::S, Vec3::new(0., -1., 0.)),
            (KeyCode::A, Vec3::new(-1., 0., 0.)),
            (KeyCode::D, Vec3::new(1., 0., 0.)),
        ];

        for (key, direction) in movements.iter() {
            if keys.just_pressed(KeyCode::C) {
                let attack = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(50., 10.)),
                        color: Color::rgb(0.6, 0.8, 0.8).into(),
                        ..default()
                    },
                    transform: *player_transform,
                    ..default()
                };

                commands.spawn((attack, Attack, P2Attack, Direction::Right));
            }

            if keys.pressed(*key) {
                const PLAYER_SPEED: f32 = 500.;
                let distance = PLAYER_SPEED * time.delta_seconds();

                player_transform.translation += *direction * distance;

                player_transform.translation.x = player_transform.translation.x.clamp(-450., 450.);
            }
        }
    }
}
