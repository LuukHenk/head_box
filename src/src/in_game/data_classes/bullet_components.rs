
use bevy::prelude::{
    Component,
    Timer
};

#[derive(Component)]
pub struct BulletMarker;

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct LifeTime(pub Timer);