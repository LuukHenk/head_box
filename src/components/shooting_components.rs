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
pub struct GunName(pub String);