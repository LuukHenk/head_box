
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::in_game::data_classes::bullet_components::Damage;

use crate::in_game::data_classes::generic_components::{GameScreenMarker, Health};
use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_COLLISION_GROUPS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::in_game::rigid_body::rigid_body_bundle::RigidBodyBundle;
use super::generic_constants::Z_VALUE;
use super::enemy_components::EnemyMarker;

#[derive(Bundle)]
pub struct ZombieBundle {
    enemy_marker: EnemyMarker,
    game_screen_marker: GameScreenMarker,
    rigid_body_bundle: RigidBodyBundle,
    health: Health,
    damage: Damage,
}
impl ZombieBundle {
    pub fn new(x: f32, y: f32, asset_server: Res<AssetServer>) -> ZombieBundle {

        let zombie_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            velocity: DEFAULT_VELOCITY,
            walking_velocity: WalkingVelocity(130.),
            gravity: DEFAULT_GRAVITY,
            collider: Collider::cuboid(10., 10.),
            continuous_collision_detection: Ccd::enabled(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("textures/player.png"),
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

        ZombieBundle {
            game_screen_marker: GameScreenMarker,
            enemy_marker: EnemyMarker,
            health: Health(100.),
            rigid_body_bundle: zombie_rigid_body,
            damage: Damage(1.),
        }
    }
}
