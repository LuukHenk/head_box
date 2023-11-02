use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct ShootingCoolDownTimer(pub Timer);

#[derive(Component)]
pub struct ActiveGun;

#[derive(Component)]
pub struct DamagePerHit(pub f32);

#[derive(Component)]
pub struct GunMarker;


#[derive(Component)]
pub struct BulletsRotationOffsetPerShot(pub Vec<f32>);

#[derive(Component, PartialEq)]
pub enum GunType {
    Pistol,
    Uzi,
    Shotgun,
}