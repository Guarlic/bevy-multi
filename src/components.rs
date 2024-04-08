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
pub struct Attack;

#[derive(Component)]
pub struct P1Attack;

#[derive(Component)]
pub struct P2Attack;

#[derive(Component)]
pub struct AttackSound;

#[derive(Component)]
pub(crate) enum Direction {
    Left,
    Right
}
