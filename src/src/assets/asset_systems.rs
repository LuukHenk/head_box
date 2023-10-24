use bevy::prelude::*;
use crate::assets::asset_components::{BulletTexture, PlayerTexture, ZombieTexture};


pub struct AssetSystems;

impl AssetSystems {

    pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(PlayerTexture(asset_server.load("textures/player.png")));
        commands.spawn(ZombieTexture(asset_server.load("textures/zombie.png")));
        commands.spawn(BulletTexture(asset_server.load("textures/bullet.png")));
    }
}