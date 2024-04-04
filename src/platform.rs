use std::time::Duration;
use crate::components::*;
use crate::components::Direction;
use bevy::prelude::*;
use rand::Rng;

pub fn setup(mut commands: Commands) {
    let platform = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200., 50.)),
            ..default()
        },
        transform: Transform::from_xyz(0., -100., 0.),
        ..default()
    };

    commands.spawn((platform, Platform));

    commands.spawn((
        TimerStruct(Timer::from_seconds(10., TimerMode::Repeating)),
        PlatformTimer
    ));
}

pub fn update_spawn(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<PlatformTimer>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands
) {
    // TODO 발판 지속적으로 생성
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for player_transform in player_query.iter() {
                let player_x = player_transform.translation.x;

                let mut random = rand::thread_rng();

                let platform_y = random.gen_range(-250..=250) as f32;
                let platform_direction_temp = random.gen_range(1..=2);
                let platform_direction = match platform_direction_temp {
                    1 => Direction::Left,
                    _ => Direction::Right,
                };

                let platform = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(70., 30.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(player_x, platform_y, 0.),
                    ..default()
                };

                commands.spawn((platform, platform_direction, Platform));

                let timer_duration = random.gen_range(3..5);

                timer.0.set_duration(Duration::from_secs(timer_duration));
                timer.0.reset();
            }
        }
    }
}

pub fn update_move() {
    // TODO 발판이 좌우로 왕복 운동
}
