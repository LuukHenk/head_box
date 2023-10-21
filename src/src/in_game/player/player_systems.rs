


use bevy::prelude::{Commands, Res, AssetServer, Input, KeyCode, Query, With, Transform, Entity, Vec2};
use bevy_rapier2d::prelude::{Velocity};

use super::generic_components::GameScreenMarker;
use super::bullet_bundle::BulletBundle;
use super::player_bundle::PlayerBundle;
use super::player_components::PlayerMarker;
use super::player_constants::PLAYER_SIZE;
pub struct PlayerSystems;


impl PlayerSystems {

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(PlayerBundle::new(asset_server));
    }

    pub fn set_velocity(
        keyboard_input: Res<Input<KeyCode>>,
        mut velocity_query: Query<&mut Velocity, With<PlayerMarker>>
    ) {
        let velocity_speed = 200.;
        for mut velocity in velocity_query.iter_mut() {
            velocity.angvel = 0.;
            velocity.linvel = Vec2::new(0., 0.);
            if keyboard_input.pressed(KeyCode::Right) {
                velocity.linvel[0] += velocity_speed;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                velocity.linvel[0] -= velocity_speed;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                velocity.linvel[1] += velocity_speed;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                velocity.linvel[1] -= velocity_speed;
            }
        }
    }

    pub fn shoot(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&mut Transform, Entity), With<PlayerMarker>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        if keyboard_input.pressed(KeyCode::Space) {
            for (transform, entity) in player_query.iter_mut() {
                let bullet_bundle = BulletBundle::new(transform, PLAYER_SIZE, entity, &asset_server);
                commands.spawn(bullet_bundle);
            }
        }
    }
}