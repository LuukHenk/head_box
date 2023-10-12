
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        // .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, (
        //
        // ))
        .run();
}