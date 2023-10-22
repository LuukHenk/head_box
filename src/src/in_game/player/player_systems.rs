


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;

use super::bullet_bundle::BulletBundle;
use super::player_bundle::PlayerBundle;
use super::player_components::PlayerMarker;
use super::player_constants::PLAYER_SIZE;
pub struct PlayerSystems;


impl PlayerSystems {

    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn(PlayerBundle::new(asset_server));
    }

    pub fn set_velocity(
        keyboard_input: Res<Input<KeyCode>>,
        mut velocity_query: Query<(&mut Velocity, &WalkingVelocity), With<PlayerMarker>>
    ) {
        for (mut velocity, walking_velocity) in velocity_query.iter_mut() {
            velocity.angvel = 0.;
            velocity.linvel = Vec2::new(0., 0.);
            if keyboard_input.pressed(KeyCode::Right) {
                velocity.linvel[0] += walking_velocity.0;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                velocity.linvel[0] -= walking_velocity.0;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                velocity.linvel[1] += walking_velocity.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                velocity.linvel[1] -= walking_velocity.0;
            }
        }
    }

    pub fn shoot(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&mut Transform, &CollisionGroups), With<PlayerMarker>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        if keyboard_input.pressed(KeyCode::Space) {
            for (transform, collision_groups) in player_query.iter_mut() {
                let bullet_bundle = BulletBundle::new(transform, PLAYER_SIZE, *collision_groups, &asset_server);
                commands.spawn(bullet_bundle);
            }
        }
    }
}