
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::GamePlugin;
use super::MainMenuPlugin;
use super::ScreenState;

pub struct HeadBoxPlugin;
impl Plugin for HeadBoxPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_state::<ScreenState>()
            .add_systems(Startup, setup)
            .add_plugins((
                GamePlugin,
                MainMenuPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                // RapierDebugRenderPlugin::default(),
            ));
    }

}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}