use bevy::asset::Handle;
use bevy::prelude::{Bundle, SpriteBundle, Sprite, default, Vec3, Transform, Color, Vec2, Image};
use bevy_rapier2d::prelude::{ActiveEvents, Ccd, Collider, GravityScale, RigidBody, Sleeping, Velocity};
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::rigid_body_constants::DEFAULT_COLLISION_GROUPS;
use crate::in_game::rigid_body::rigid_body_bundle::RigidBodyBundle;
use super::WallMarker;
use super::generic_constants::Z_VALUE;
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
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            gravity: GravityScale(0.0),
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
            active_events: ActiveEvents::COLLISION_EVENTS,
        };
        WallBundle {
            game_screen_marker: GameScreenMarker,
            rigid_body_bundle: wall_rigid_body,
            wall_marker: WallMarker,
        }
    }
}