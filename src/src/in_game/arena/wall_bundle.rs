

use bevy::prelude::{
    Bundle,
    SpriteBundle,
    Sprite,
    default,
    Vec3,
    Transform,
    Color,
};
use bevy_rapier2d::prelude::RigidBody;
use super::WallMarker;
use super::generic_constants::Z_VALUE;
#[derive(Bundle)]
pub struct WallBundle {
    wall_marker: WallMarker,
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
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
            rigid_body: RigidBody::Fixed,
        }
    }
}