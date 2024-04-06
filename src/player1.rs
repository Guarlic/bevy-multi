use crate::components::*;
use crate::components::Direction;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(30., 30.)),
            ..default()
        },
        texture: asset_server.load("player1.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    };

    commands.spawn((player, Player, Player1));
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player1>>,
    mut commands: Commands
) {
    for mut player_transform in player_query.iter_mut() {
        let movements = [
            (KeyCode::Up, Vec3::new(0., 1., 0.)),
            (KeyCode::Down, Vec3::new(0., -1., 0.)),
            (KeyCode::Left, Vec3::new(-1., 0., 0.)),
            (KeyCode::Right, Vec3::new(1., 0., 0.)),
        ];

        for (key, direction) in movements.iter() {
            if keys.just_pressed(KeyCode::K) {
                let attack = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(50., 10.)),
                        color: Color::rgb(0.9, 0.6, 0.6).into(),
                        ..default()
                    },
                    transform: *player_transform,
                    ..default()
                };

                commands.spawn((attack, Attack, P1Attack, Direction::Left));
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
