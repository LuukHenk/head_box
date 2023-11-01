use bevy::prelude::*;

use crate::components::asset_components::{BulletTextureHandle, PistolSoundHandle, PlayerTextureHandles, ZombieTenseSoundHandle, ZombieTextureHandle};
pub struct AssetSystems;

impl AssetSystems {
    pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player_textures = PlayerTextureHandles {
            back: asset_server.load("textures/player/player_back.png"),
            front: asset_server.load("textures/player/player_front.png"),
            side: asset_server.load("textures/player/player_side.png"),
            side_flipped: asset_server.load("textures/player/player_side_flipped.png"),
        };
        commands.spawn(player_textures);
        commands.spawn(ZombieTextureHandle(
            asset_server.load("textures/zombie/zombie_front.png"),
        ));
        commands.spawn(BulletTextureHandle(asset_server.load("textures/bullet.png")));
        commands.spawn(PistolSoundHandle(asset_server.load("sounds/pistol.ogg")));
        commands.spawn(ZombieTenseSoundHandle(asset_server.load("sounds/zombie_tense.ogg")));
    }
}
