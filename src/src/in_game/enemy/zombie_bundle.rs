
use bevy::prelude::{
    Bundle,
    SpriteBundle,
    Transform,
    Vec3,
    default,
    Sprite,
    Color,
};
use super::generic_constants::Z_VALUE;
use super::data_classes::movement_components::{
    Movement,
    CollisionMarker
};
use super::enemy_components::EnemyMarker;

#[derive(Bundle)]
pub struct ZombieBundle {
    sprite_bundle: SpriteBundle,
    movement: Movement,
    enemy_marker: EnemyMarker,
    collision_marker: CollisionMarker,
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
                velocity,
            },
            enemy_marker: EnemyMarker,
            collision_marker: CollisionMarker,
        }
    }
}
