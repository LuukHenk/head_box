
use bevy::prelude::*;

use crate::components::asset_components::{InGameMusicHandle, BulletTextureHandle, PistolSoundHandle, PlayerTextureHandles, ShotgunSoundHandle, UziSoundHandle, ZombieTenseSoundHandle, ZombieTextureHandle, MenuMusicHandle};

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
        commands.spawn(UziSoundHandle(asset_server.load("sounds/uzi.ogg")));
        commands.spawn(ShotgunSoundHandle(asset_server.load("sounds/shotgun.ogg")));

        commands.spawn(ZombieTenseSoundHandle(asset_server.load("sounds/zombie_tense.ogg")));
        commands.spawn(InGameMusicHandle(asset_server.load("sounds/in_game_music.ogg")));
        commands.spawn(MenuMusicHandle(asset_server.load("sounds/menu_music.ogg")));

    }
}
