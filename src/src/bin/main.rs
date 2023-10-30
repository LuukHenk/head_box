
use bevy::prelude::*;
use head_box::HeadBoxPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HeadBox".into(),
                resolution: (960., 540.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(HeadBoxPlugin)
        .run();
}