use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct TimerStruct(pub(crate) Timer);

#[derive(Component)]
pub struct PlatformTimer;

#[derive(Component)]
pub(crate) enum Direction {
    Left,
    Right
}
