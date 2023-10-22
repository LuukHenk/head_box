

use bevy::{
    prelude::Component,
    time::Timer,
};

#[derive(Component)]
pub struct CoolDownTime(pub Timer);

#[derive(Component)]
pub struct WeaponMarker;