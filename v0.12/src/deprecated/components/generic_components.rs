use bevy::prelude::Component;

#[derive(Component)]
pub struct GameScreenMarker;

#[derive(Component, Debug)]
pub struct Health(pub f32);
