
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::in_game::data_classes::rigid_body_constants::DEFAULT_WALING_VELOCITY;

use super::data_classes::generic_components::GameScreenMarker;
use super::data_classes::rigid_body_constants::{
    DEFAULT_COLLISION_GROUPS,
    DEFAULT_GRAVITY,
    DEFAULT_ACTIVE_EVENTS,
    DEFAULT_VELOCITY
};
use super::data_classes::wall_components::WallMarker;
use super::data_classes::generic_constants::Z_VALUE;

use super::rigid_body::rigid_body_bundle::RigidBodyBundle;

#[derive(Bundle)]
pub struct WallBundle {
    wall_marker: WallMarker,
    game_screen_marker: GameScreenMarker,
    rigid_body_bundle: RigidBodyBundle,
}

impl WallBundle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, texture: Handle<Image>) -> WallBundle {
        let wall_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Fixed,
            velocity: DEFAULT_VELOCITY,
            walking_velocity: DEFAULT_WALING_VELOCITY,
            gravity: DEFAULT_GRAVITY,
            collider: Collider::cuboid(width, height),
            continuous_collision_detection: Ccd::disabled(),
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    ..default()
                },
                ..default()
            },
            sleeping: Sleeping::disabled(),
            collision_groups: DEFAULT_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
        };
        WallBundle {
            game_screen_marker: GameScreenMarker,
            rigid_body_bundle: wall_rigid_body,
            wall_marker: WallMarker,
        }
    }
}