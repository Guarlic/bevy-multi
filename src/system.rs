use crate::components::*;
use std::thread::sleep;
use std::time::Duration;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::sprite::collide_aabb::collide;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update_game_over(
    mut exit_events: ResMut<Events<AppExit>>,
    player1_query: Query<(&Transform, &Sprite), With<Player1>>,
    player2_query: Query<(&Transform, &Sprite), With<Player2>>,
    p1attack_query: Query<(&Transform, &Sprite), With<P1Attack>>,
    p2attack_query: Query<(&Transform, &Sprite), With<P2Attack>>
) {
    for (player1_transform, player1_sprite) in player1_query.iter() {
        for (p2attack_transform, p2attack_sprite) in p2attack_query.iter() {
            let collision_p2win = collide(
                player1_transform.translation,
                player1_sprite.custom_size.unwrap(),
                p2attack_transform.translation,
                p2attack_sprite.custom_size.unwrap(),
            );

            if let Some(_) = collision_p2win {
                println!("P2 Win!");

                sleep(Duration::from_millis(800));
                exit_events.send(AppExit);

                return;
            }
        }
    }

    for (player2_transform, player2_sprite) in player2_query.iter() {
        for (p1attack_transform, p1attack_sprite) in p1attack_query.iter() {
            let collision_p1win = collide(
                player2_transform.translation,
                player2_sprite.custom_size.unwrap(),
                p1attack_transform.translation,
                p1attack_sprite.custom_size.unwrap(),
            );

            if collision_p1win.is_some() {
                println!("P1 Win!");

                sleep(Duration::from_millis(800));
                exit_events.send(AppExit);

                return;
            }
        }
    }
}

pub fn update_gravity(mut player_query: Query<&mut Transform, With<Player>>) {
    const GRAVITY: f32 = -9.81;
    const SPEED: f32 = 0.4;

    for mut player_transform in player_query.iter_mut() {
        player_transform.translation.y += GRAVITY * SPEED;

        if player_transform.translation.y >= 350. {
            player_transform.translation.y = -320.;
        }

        if player_transform.translation.y <= -350. {
            player_transform.translation.y = 320.;
        }
    }
}
