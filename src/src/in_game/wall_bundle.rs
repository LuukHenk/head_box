
use bevy::prelude::*;
use super::game_components::*;
use super::game_constants::*;

pub const HIDDEN_WALL_COLOR: Color = Color::BLUE;

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