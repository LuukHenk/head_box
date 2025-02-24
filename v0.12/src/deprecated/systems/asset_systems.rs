
use bevy::prelude::*;

use crate::components::asset_components::{InGameMusicHandle, BulletTextureHandle, PistolSoundHandle, CharacterTextureHandles, ShotgunSoundHandle, UziSoundHandle, ZombieTenseSoundHandle, MenuMusicHandle, PlayerTextureMarker, ZombieTextureMarker, PistolTextureMarker, ShotgunTextureMarker, UziTextureMarker};

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
        commands.spawn((player_textures, PlayerTextureMarker));
    }

    pub fn setup_zombie_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let zombie_front_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/zombie/zombie_front_1.png"),
            asset_server.load("textures/zombie/zombie_front_0.png"),
            asset_server.load("textures/zombie/zombie_front_2.png"),
        ];
        let zombie_right_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/zombie/zombie_right_0.png"),
            asset_server.load("textures/zombie/zombie_right_1.png"),
            asset_server.load("textures/zombie/zombie_right_2.png"),
        ];
        let zombie_back_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/zombie/zombie_back_0.png"),
            asset_server.load("textures/zombie/zombie_back_1.png"),
            asset_server.load("textures/zombie/zombie_back_2.png"),
        ];
        let zombie_textures = CharacterTextureHandles {
            front: zombie_front_textures,
            right: zombie_right_textures,
            back: zombie_back_textures,
        };
        commands.spawn((zombie_textures, ZombieTextureMarker));
    }

    pub fn setup_pistol_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let pistol_front_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/pistol/pistol_front.png"),
        ];
        let pistol_right_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/pistol/pistol_right.png"),
        ];
        let pistol_back_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/pistol/pistol_back.png"),
        ];
        let pistol_textures = CharacterTextureHandles {
            front: pistol_front_textures,
            right: pistol_right_textures,
            back: pistol_back_textures,
        };
        commands.spawn((pistol_textures, PistolTextureMarker));
        commands.spawn(PistolSoundHandle(asset_server.load("sounds/pistol.ogg")));
    }
    pub fn setup_uzi_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let uzi_front_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/uzi/uzi_front.png"),
        ];
        let uzi_right_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/uzi/uzi_right.png"),
        ];
        let uzi_back_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/uzi/uzi_back.png"),
        ];
        let uzi_textures = CharacterTextureHandles {
            front: uzi_front_textures,
            right: uzi_right_textures,
            back: uzi_back_textures,
        };
        commands.spawn((uzi_textures, UziTextureMarker));
        commands.spawn(UziSoundHandle(asset_server.load("sounds/uzi.ogg")));
    }
    pub fn setup_shotgun_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let shotgun_front_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/shotgun/shotgun_front.png"),
        ];
        let shotgun_right_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/shotgun/shotgun_right.png"),
        ];
        let shotgun_back_textures: Vec<Handle<Image>> = vec![
            asset_server.load("textures/weapons/shotgun/shotgun_back.png"),
        ];
        let shotgun_textures = CharacterTextureHandles {
            front: shotgun_front_textures,
            right: shotgun_right_textures,
            back: shotgun_back_textures,
        };
        commands.spawn((shotgun_textures, ShotgunTextureMarker));
        commands.spawn(ShotgunSoundHandle(asset_server.load("sounds/shotgun.ogg")));
    }

    pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(BulletTextureHandle(asset_server.load("textures/bullet.png")));

        commands.spawn(ZombieTenseSoundHandle(asset_server.load("sounds/zombie_tense.ogg")));
        commands.spawn(InGameMusicHandle(asset_server.load("sounds/in_game_music.ogg")));
        commands.spawn(MenuMusicHandle(asset_server.load("sounds/menu_music.ogg")));

    }
}
