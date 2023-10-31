
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::asset_systems::AssetSystems;
use super::GamePlugin;
use super::MainMenuPlugin;
use super::display_handler::{ScreenState, setup_camera};

pub struct HeadBoxPlugin;
impl Plugin for HeadBoxPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_state::<ScreenState>()
            .add_systems(Startup, (
                    setup_camera,
                    AssetSystems::setup_assets
                )
            )
            .add_plugins((
                GamePlugin,
                MainMenuPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
                // RapierDebugRenderPlugin::default(),
            ))

        ;
    }

}
