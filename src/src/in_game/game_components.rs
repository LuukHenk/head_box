use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct EnemyMarker;

#[derive(Component)]
pub struct GameScreenMarker;

#[derive(Component)]
pub struct CollisionMarker;

#[derive(Component, Debug)]
pub struct Health(pub f32);

#[derive(Component, Debug)]
pub struct Movement {
    pub direction_x: f32,
    pub direction_y: f32,
    pub velocity: f32,
}

