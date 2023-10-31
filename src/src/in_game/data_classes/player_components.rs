
use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct ShootingCoolDownTimer(pub Timer);