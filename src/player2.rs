use crate::components::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    let player = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(30., 30.)),
            color: Color::LIME_GREEN,
            ..default()
        },
        transform: Transform::from_xyz(-100., 0., 0.),
        ..default()
    };

    commands.spawn((player, Player, Player2));
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player2>>
) {
    for mut player_transform in player_query.iter_mut() {
        let movements = [
            (KeyCode::W, Vec3::new(0., 1., 0.)),
            (KeyCode::S, Vec3::new(0., -1., 0.)),
            (KeyCode::A, Vec3::new(-1., 0., 0.)),
            (KeyCode::D, Vec3::new(1., 0., 0.)),
        ];

        for (key, direction) in movements.iter() {
            if keys.pressed(*key) {
                const PLAYER_SPEED: f32 = 500.;
                let distance = PLAYER_SPEED * time.delta_seconds();

                player_transform.translation += *direction * distance;
            }
        }
    }
}
