use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY, DEFAULT_WALING_VELOCITY};
use crate::in_game::data_classes::bullet_constants::{BULLET_LENGTH, BULLET_WIDTH, SHOOTER_DISTANCE_BUFFER};
use crate::in_game::data_classes::bullet_components::{BulletMarker, Damage, LifeTime};
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::generic_constants::{Z_VALUE};

use super::rigid_body_bundle::RigidBodyBundle;


#[derive(Bundle)]
pub struct BulletBundle {
    bullet_marker: BulletMarker,
    damage: Damage,
    life_time: LifeTime,
    rigid_body_bundle: RigidBodyBundle,
    game_screen_marker: GameScreenMarker,
}

impl BulletBundle {
    pub fn new(
        transform: Transform,
        collision_groups: CollisionGroups,
        texture: Handle<Image>,
    ) -> BulletBundle {
        let bullet_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Fixed,
            velocity: DEFAULT_VELOCITY,
            gravity: DEFAULT_GRAVITY,
            walking_velocity: DEFAULT_WALING_VELOCITY,
            collider: Collider::cuboid(BULLET_WIDTH, BULLET_LENGTH),
            continuous_collision_detection: Ccd::disabled(),
            sprite_bundle: SpriteBundle {
                texture,
                transform,
                ..default()
            },
            sleeping: Sleeping::disabled(),
            collision_groups,
            active_events: DEFAULT_ACTIVE_EVENTS,
        };

        let bullet_timer = Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once);

        BulletBundle {
            damage: Damage(0.5),
            life_time: LifeTime(bullet_timer),
            bullet_marker: BulletMarker,
            rigid_body_bundle: bullet_rigid_body,
            game_screen_marker: GameScreenMarker,
        }
    }
}