
use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct PlayerShootEvent(pub Entity);

#[derive(Event)]
pub struct BulletSpawnEvent(pub Entity);