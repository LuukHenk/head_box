use bevy::prelude::{
    Commands,
    Color,
};
use super::wall_bundle::WallBundle;
use super::HIDDEN_WALL_COLOR;
use super::generic_constants::OUTER_Y_COORDINATES;
use super::generic_components::GameScreenMarker;
pub struct Boxy;

impl Boxy {
    pub fn spawn(mut commands: Commands) {
        // Top
        commands.spawn((WallBundle::new(0., OUTER_Y_COORDINATES, 2000., 80., HIDDEN_WALL_COLOR), GameScreenMarker));
        commands.spawn((WallBundle::new(-400., 340., 600., 40., Color::BLACK), GameScreenMarker));
        commands.spawn((WallBundle::new(400., 340., 600., 40., Color::BLACK), GameScreenMarker));
        // Bottom
        commands.spawn((WallBundle::new(0., -OUTER_Y_COORDINATES, 2000., 80., HIDDEN_WALL_COLOR), GameScreenMarker));
        commands.spawn((WallBundle::new(-400., -340., 600., 40., Color::BLACK), GameScreenMarker));
        commands.spawn((WallBundle::new(400., -340., 600., 40., Color::BLACK), GameScreenMarker));
        // Sides
        commands.spawn((WallBundle::new(-620., 0., 40., 2000.,Color::BLACK), GameScreenMarker));
        commands.spawn((WallBundle::new(620., 0., 40., 2000.,Color::BLACK), GameScreenMarker));
    }
}