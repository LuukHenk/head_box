


use bevy::prelude::{
    Commands,
    Res,
    AssetServer,
    Input,
    KeyCode,
    Query,
    With,
    Transform,
    Entity,
};

use super::generic_components::GameScreenMarker;
use super::data_classes::movement_components::Movement;
use super::bullet_bundle::BulletBundle;
use super::player_bundle::PlayerBundle;
use super::player_components::PlayerMarker;
use super::player_constants::PLAYER_SIZE;

pub struct PlayerSystems;


impl PlayerSystems {

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((PlayerBundle::new(asset_server), GameScreenMarker));
    }

    pub fn set_direction(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<&mut Movement, With<PlayerMarker>>
    ) {

        for mut movement in player_query.iter_mut() {
            movement.direction_x = 0.;
            movement.direction_y = 0.;
            if keyboard_input.pressed(KeyCode::Right) {
                movement.direction_x += 1.;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                movement.direction_x -= 1.;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                movement.direction_y += 1.;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                movement.direction_y -= 1.;
            }
        }
    }

    pub fn shoot(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&mut Transform, Entity), With<PlayerMarker>>,
        mut commands: Commands,
    ) {
        if keyboard_input.pressed(KeyCode::Space) {
            for (transform, entity) in player_query.iter_mut() {
                let bullet_bundle = BulletBundle::new(transform, PLAYER_SIZE, entity);
                commands.spawn((bullet_bundle, GameScreenMarker));
            }
        }
    }
}