
use bevy::prelude::{
    Bundle,
    SpriteBundle,
    Transform,
    Vec3,
    default,
    Sprite,
    Color,
};
use crate::in_game::data_classes::generic_components::Health;
use super::generic_constants::Z_VALUE;
use super::data_classes::movement_components::{
    Movement,
};
use super::enemy_components::EnemyMarker;

#[derive(Bundle)]
pub struct ZombieBundle {
    sprite_bundle: SpriteBundle,
    movement: Movement,
    enemy_marker: EnemyMarker,
    health: Health
}
impl ZombieBundle {
    pub fn new(x: f32, y: f32, velocity: f32) -> ZombieBundle {
        ZombieBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::LIME_GREEN, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                default_velocity: velocity,
                current_velocity: 0.
            },
            enemy_marker: EnemyMarker,
            health: Health(10.),
        }
    }
}
