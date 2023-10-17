use std::time::Duration;

use bevy::{
    prelude::*,
    time::Timer,
};
use crate::weapons::bullets::Bullet;

use super::game_components::*;
use super::bullets::PistolBullet;

#[derive(Component)]
pub struct BulletsPerShot(pub u32);

#[derive(Component)]
pub struct CoolDownTime(pub Timer);

#[derive(Component)]
pub struct GunMarker;


#[derive(Bundle)]
pub struct Gun {
    cooldown_time: CoolDownTime,
    bullets_per_shot: BulletsPerShot,
    gun_marker: GunMarker,
    bullet_type: Bullet,
}


pub trait Pistol{
    fn new();

}

impl Pistol for Gun {
    fn new() -> Gun {
        Gun {
            cooldown_time: CoolDownTime(Timer::new(Duration::from_secs(1), TimerMode::Once)),
            bullets_per_shot: BulletsPerShot(1),
            gun_marker: GunMarker,
            bullet_type: PistolBullet::new(),
        }
    }

}