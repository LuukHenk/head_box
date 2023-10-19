

use bevy::prelude::{
    Bundle,
    SpriteBundle,
    Sprite,
    default,
    Vec3,
    Transform,
    Color,
};
use super::WallMarker;
use super::generic_constants::Z_VALUE;
use super::CollisionMarker;

#[derive(Bundle)]
pub struct WallBundle {
    wall_marker: WallMarker,
    sprite_bundle: SpriteBundle,
    collision_marker: CollisionMarker,
}

impl WallBundle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> WallBundle {
        WallBundle {
            wall_marker: WallMarker,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    scale: Vec3::new(width, height, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            },
            collision_marker: CollisionMarker,
        }
    }
}