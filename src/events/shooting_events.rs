use bevy::prelude::*;
use crate::components::shooting_components::GunType;

#[derive(Event)]
pub struct ShootRequestEvent;

#[derive(Event)]
pub struct BulletSpawnEvent;

#[derive(Event)]
pub struct WeaponSelectionEvent(pub GunType);