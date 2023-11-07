
use bevy::prelude::*;

use crate::components::asset_components::{InGameMusicHandle, BulletTextureHandle, PistolSoundHandle, CharacterTextureHandles, ShotgunSoundHandle, UziSoundHandle, ZombieTenseSoundHandle, ZombieTextureHandle, MenuMusicHandle};
use crate::components::player_components::PlayerMarker;

pub struct AssetSystems;

impl AssetSystems {


    pub fn setup_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player_front_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/player/player_front_0.png"),
            asset_server.load("textures/player/player_front_1.png"),
            asset_server.load("textures/player/player_front_2.png"),
        ];
        let player_right_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/player/player_right_0.png"),
            asset_server.load("textures/player/player_right_1.png"),
            asset_server.load("textures/player/player_right_2.png"),
        ];
        let player_back_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/player/player_back_0.png"),
            asset_server.load("textures/player/player_back_1.png"),
            asset_server.load("textures/player/player_back_2.png"),
        ];
        let player_textures = CharacterTextureHandles {
            front: player_front_textures,
            right: player_right_textures,
            back: player_back_textures,
        };
        commands.spawn((player_textures, PlayerMarker));
    }

    pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
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
