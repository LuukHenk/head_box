
use bevy::prelude::*;
use head_box::HeadBoxPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HeadBoxPlugin)
        .run();
}