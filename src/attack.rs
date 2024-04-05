use crate::components::*;
use crate::components::Direction;
use bevy::prelude::*;

pub fn update(
    time: Res<Time>,
    mut attack_query: Query<(&mut Transform, &Direction, Entity), With<Attack>>,
    mut commands: Commands
) {
    for (mut attack_transform, attack_direction, attack_entity) in attack_query.iter_mut() {
        if attack_transform.translation.x < -450. || attack_transform.translation.x > 450. {
            commands.entity(attack_entity).despawn();

            return;
        }

        const ATTACK_SPEED: f32 = 400.;
        let distance = ATTACK_SPEED * time.delta_seconds();

        match *attack_direction {
            Direction::Left => attack_transform.translation.x -= distance,
            Direction::Right => attack_transform.translation.x += distance
        }
    }
}
