use bevy::prelude::*;
use head_box::{DiagnosticsPlugin, GamePlugin, GameState, MainMenuPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HeadBox".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((MainMenuPlugin, GamePlugin))
        // .add_plugins(DiagnosticsPlugin)
        .run();
}
