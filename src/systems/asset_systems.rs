use bevy::prelude::*;

use crate::components::asset_components::{BulletTexture, PistolSound, PlayerTextures, ZombieTexture};
pub struct AssetSystems;

impl AssetSystems {
    pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player_textures = PlayerTextures {
            back: asset_server.load("textures/player/player_back.png"),
            front: asset_server.load("textures/player/player_front.png"),
            side: asset_server.load("textures/player/player_side.png"),
            side_flipped: asset_server.load("textures/player/player_side_flipped.png"),
        };
        commands.spawn(player_textures);
        commands.spawn(ZombieTexture(
            asset_server.load("textures/zombie/zombie_front.png"),
        ));
        commands.spawn(BulletTexture(asset_server.load("textures/bullet.png")));
        commands.spawn(PistolSound(asset_server.load("sounds/pistol.ogg")));
    }
}
