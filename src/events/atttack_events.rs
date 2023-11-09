use bevy::prelude::*;
use crate::components::weapon_components::WeaponType;

#[derive(Event)]
pub struct AttackRequestEvent;

#[derive(Event)]
pub struct BulletSpawnEvent(pub f32);

#[derive(Event)]
pub struct WeaponSelectionEvent(pub WeaponType);