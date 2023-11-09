use bevy::prelude::{Component, Entity, Handle, Image, Timer};
use bevy_rapier2d::geometry::CollisionGroups;

#[derive(Component)]
pub struct AttackCoolDownTimer(pub Timer);

#[derive(Component)]
pub struct ActiveWeapon;

#[derive(Component)]
pub struct DamagePerHit(pub f32);

#[derive(Component)]
pub struct WeaponMarker;


#[derive(Component)]
pub struct BulletsRotationOffsetPerShot(pub Vec<f32>);

#[derive(Component, PartialEq)]
pub enum WeaponType {
    Pistol,
    Uzi,
    Shotgun,
}

#[derive(Component)]
pub struct Owner(pub Option<Entity>);

#[derive(Component)]
pub struct WeaponOwnerMarker;

#[derive(Component)]
pub struct BulletTexture(pub Handle<Image>);

#[derive(Component)]
pub struct BulletCollisionGroups(pub CollisionGroups);

#[derive(Component)]
pub struct BulletLength(pub f32);