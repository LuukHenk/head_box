use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::in_game::data_classes::bullet_constants::{BULLET_LENGTH, BULLET_WIDTH};
use crate::in_game::data_classes::bullet_components::{BulletMarker, Damage, LifeTime};
use crate::in_game::data_classes::generic_components::GameScreenMarker;


#[derive(Bundle)]
pub struct BulletBundle {
    bullet_marker: BulletMarker,
    damage: Damage,
    life_time: LifeTime,
    game_screen_marker: GameScreenMarker,
    rigid_body: RigidBody,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    gravity: GravityScale,
    collider: Collider,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
}

impl BulletBundle {
    pub fn new(
        transform: Transform,
        collision_groups: CollisionGroups,
        texture: Handle<Image>,
    ) -> BulletBundle {

        let bullet_timer = Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once);

        BulletBundle {
            damage: Damage(0.5),
            life_time: LifeTime(bullet_timer),
            bullet_marker: BulletMarker,
            game_screen_marker: GameScreenMarker,
            rigid_body: RigidBody::Fixed,
            velocity: DEFAULT_VELOCITY,
            gravity: DEFAULT_GRAVITY,
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
        }
    }
}