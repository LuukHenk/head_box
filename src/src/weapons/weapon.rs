use std::time::Duration;

use bevy::{
    prelude::*,
    time::Timer,
};
use super::game_components::*;



#[derive(Component)]
struct BulletMarker;

#[derive(Component)]
struct Damage(f32);

#[derive(Component)]
struct BulletsPerShot(u32);

#[derive(Component)]
struct MaxLifeTime(Timer);

#[derive(Component)]
struct CoolDownTime(Timer);

#[derive(Bundle)]
struct Bullet {
    damage: Damage,
    movement: Movement,
    bullet_marker: BulletMarker,
    max_lifetime: MaxLifeTime,
}

impl Bullet {
    fn new(damage: f32, movement: Movement, max_lifetime: f32) -> Bullet {

        Bullet {
            max_lifetime: MaxLifeTime(Timer::new(Duration::from_secs(max_lifetime as u64), TimerMode::Once)),
            damage: Damage(damage),
            movement,
            bullet_marker: BulletMarker,
        }
    }
}

#[derive(Bundle)]
struct Weapon {
    cooldown_time: CoolDownTime,
    bullet: Bullet,
    bullets_per_shot: BulletsPerShot,
}


impl Weapon {
    fn new(cooldown_time: f32, bullet: Bullet, bullets_per_shot: u32) -> Weapon {

        Weapon {
            cooldown_time: CoolDownTime(Timer::new(Duration::from_secs(cooldown_time as u64), TimerMode::Once)),
            bullet,
            bullets_per_shot: BulletsPerShot(bullets_per_shot),
        }
    }
    fn shoot(&self, mut commands: Commands, bullet_type: Bullet) {
        if self.cooldown_time.0.finished() {
            for _i in 0..self.bullets_per_shot.0 {
                // TODO: Shoot bullet
            }
        }
    }

}
