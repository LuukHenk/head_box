
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::main_menu_plugin::MainMenuPlugin;
use crate::plugins::game_plugin::GamePlugin;
use crate::plugins::load_screen_plugin::LoadScreenPlugin;

use crate::systems::asset_systems::AssetSystems;
use crate::systems::camera_systems::CameraSystems;

use crate::states::screen_state::ScreenState;


pub struct HeadBoxPlugin;
impl Plugin for HeadBoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
            .add_state::<ScreenState>()
            .add_systems(Startup, (
                CameraSystems::setup_camera,
                AssetSystems::setup_assets,
                AssetSystems::setup_player_assets,
                AssetSystems::setup_zombie_assets,
                AssetSystems::setup_pistol_assets,
            ))
            .add_plugins((
                GamePlugin,
                MainMenuPlugin,
                LoadScreenPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
                RapierDebugRenderPlugin::default(),
            ));
    }
}
