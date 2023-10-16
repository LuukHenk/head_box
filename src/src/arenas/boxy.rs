use bevy::prelude::*;
use crate::in_game::game_components::GameScreenMarker;
use super::game_components::*;
use super::game_constants::*;
use super::WallBundle;
use super::HIDDEN_WALL_COLOR;
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