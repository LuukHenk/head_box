

use bevy::prelude::*;
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::generic_constants::Z_VALUE;

use crate::in_game::data_layers::wall_bundle::WallBundle;
pub struct ArenaSystems;

impl ArenaSystems {



    pub fn spawn_boxy_arena(mut commands: Commands, asset_server: Res<AssetServer>) {

        let center: f32 = 0.;

        // let wide_wall_texture : Handle<Image>= asset_server.load("textures/wide_wall.png");
        let wide_wall_x: f32 = 360.;
        let wide_wall_y: f32 = 340.;
        let wide_entry_y: f32 = 380.;
        let wide_wall_width: f32 = 280.;
        let wide_wall_height: f32 = 20.;

        // let long_wall_texture : Handle<Image>= asset_server.load("textures/long_wall.png");
        let long_wall_x: f32 = 620.;
        let long_wall_width: f32 = wide_wall_height;
        let long_wall_height: f32 = 360.;

        // Top
        // commands.spawn(WallBundle::new(
        //     center,
        //     wide_entry_y,
        //     wide_wall_width,
        //     wide_wall_height,
        //     wide_wall_texture.clone()
        // ));
        // commands.spawn(WallBundle::new(
        //     wide_wall_x,
        //     wide_wall_y,
        //     wide_wall_width, wide_wall_height,
        //     wide_wall_texture.clone()
        // ));
        // commands.spawn(WallBundle::new(
        //     -wide_wall_x,
        //     wide_wall_y,
        //     wide_wall_width, wide_wall_height,
        //     wide_wall_texture.clone()
        // ));
        //
        // // Bottom
        // commands.spawn(WallBundle::new(
        //     center,
        //     -wide_entry_y,
        //     wide_wall_width, wide_wall_height,
        //     wide_wall_texture.clone()
        // ));
        // commands.spawn(WallBundle::new(
        //     wide_wall_x,
        //     -wide_wall_y,
        //     wide_wall_width, wide_wall_height,
        //     wide_wall_texture.clone()
        // ));
        // commands.spawn(WallBundle::new(
        //     -wide_wall_x,
        //     -wide_wall_y,
        //     wide_wall_width, wide_wall_height,
        //     wide_wall_texture.clone()
        // ));
        // // Side right
        // commands.spawn(WallBundle::new(
        //     long_wall_x,
        //     center,
        //     long_wall_width,
        //     long_wall_height,
        //     long_wall_texture.clone()
        // ));
        // // Side left
        // commands.spawn(WallBundle::new(
        //     -long_wall_x,
        //     center,
        //     long_wall_width,
        //     long_wall_height,
        //     long_wall_texture.clone()
        // ));
        commands.spawn((SpriteBundle{
            texture: asset_server.load("textures/arena/arena.png"),
            transform: Transform{
                ..default()
            },
            ..default()
        }, GameScreenMarker));
    }


}