

use bevy::prelude::{
    Bundle,
    SpriteBundle,
    AssetServer,
    Res,
    Transform,
    Sprite,
    default,
    Color,
    Vec3,
};
use crate::in_game::data_classes::player_constants::PLAYER_SIZE;

use super::generic_components::Health;
use super::data_classes::movement_components::{Movement};
use super::generic_constants::Z_VALUE;
use super::player_constants::INITIAL_PLAYER_HEALTH;
use super::player_components::{
    PlayerMarker,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    health: Health,
    sprite_bundle: SpriteBundle,
    movement: Movement,
}


impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> PlayerBundle {
        // let ship_handle = asset_server.load("textures/image10.png");
        PlayerBundle {
            player_marker: PlayerMarker,
            sprite_bundle: SpriteBundle {
                // texture: ship_handle,
                transform: Transform {
                    translation: Vec3::new(0., 0., Z_VALUE),
                    scale: Vec3::new(PLAYER_SIZE, PLAYER_SIZE, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::PURPLE, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                default_velocity: 6.,
                current_velocity: 0.,
            },
            health: Health(INITIAL_PLAYER_HEALTH),
        }
    }
}
