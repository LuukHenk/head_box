use bevy::prelude::Component;

#[derive(Component)]
pub struct WalkingVelocity(pub f32);

#[derive(Component)]
pub struct RotationDegrees(pub f32);