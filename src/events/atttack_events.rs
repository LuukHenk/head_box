use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;
use crate::components::bullet_components::Damage;
use crate::components::weapon_components::WeaponType;

#[derive(Event)]
pub struct AttackRequestEvent(pub Entity);

#[derive(Event)]
pub struct BulletSpawnEvent{
    pub damage: Damage,
    pub transform: Transform,
    pub collision_groups: CollisionGroups,
    pub texture: Handle<Image>
}

#[derive(Event)]
pub struct WeaponSelectionEvent(pub WeaponType);