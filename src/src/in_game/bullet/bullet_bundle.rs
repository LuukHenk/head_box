use std::time::Duration;
use bevy::prelude::{Bundle, Transform, Vec3, SpriteBundle, Sprite, default, Color, Timer, TimerMode, Mut};

use crate::in_game::data_classes::bullet_components::{
    BulletMarker,
    Damage,
    LifeTime,
};
use super::generic_constants::{
    Z_VALUE
};

#[derive(Bundle)]
pub struct BulletBundle {
    bullet_marker: BulletMarker,
    damage: Damage,
    life_time: LifeTime,
    sprite_bundle: SpriteBundle,
}

impl BulletBundle {
    pub fn new(shooter_transform: Mut<Transform>, shooter_size: f32) -> BulletBundle {
        let bullet_length = 100.;
        let shooter_front = (shooter_transform.rotation * Vec3::Y).truncate().normalize();

        let transform = Transform {
            translation: Vec3::new(
                shooter_transform.translation.x + (shooter_size/2. + bullet_length / 2.)* shooter_front[0],
                shooter_transform.translation.y + (shooter_size/2. + bullet_length / 2.)* shooter_front[1],
                shooter_transform.translation.z
            ),
            scale: Vec3::new(1., bullet_length, Z_VALUE),
            rotation: shooter_transform.rotation,
        };

        BulletBundle {
            sprite_bundle: SpriteBundle {
                transform,
                sprite: Sprite { color: Color::BLUE, ..default() },
                ..default()
            },
            damage: Damage(0.5),
            life_time: LifeTime(Timer::new(Duration::from_secs(1), TimerMode::Once)),
            bullet_marker: BulletMarker,
        }
    }
}