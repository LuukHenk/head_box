
use bevy::prelude::*;
use head_box::HeadBoxDisplayPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HeadBoxDisplayPlugin)
        .run();
}