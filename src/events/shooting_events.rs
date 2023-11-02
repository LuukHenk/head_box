use bevy::prelude::*;

#[derive(Event)]
pub struct ShootRequestEvent;

#[derive(Event)]
pub struct BulletSpawnEvent;