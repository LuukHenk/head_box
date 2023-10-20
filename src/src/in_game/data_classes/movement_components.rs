use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Movement {
    pub direction_x: f32,
    pub direction_y: f32,
    pub current_velocity: f32,
    pub default_velocity: f32,
}