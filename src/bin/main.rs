use bevy::prelude::*;
use head_box::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HeadBox".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_plugins(MainMenuPlugin)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
