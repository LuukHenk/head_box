use bevy::prelude::*;
use super::ScreenState;
use super::despawn_screen;
use super::game_components::*;
use super::player_bundle::PlayerBundle;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(ScreenState::Game), (
                    PlayerBundle::spawn
                )
            )
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<GameScreenMarker>)
            .add_systems(FixedUpdate, (
                    PlayerBundle::set_direction,
                    move_objects,
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}

fn move_objects(mut query: Query<(&mut Transform, &Movement), With<Movement>>) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation.x += movement.direction_x * movement.velocity;
        transform.translation.y += movement.direction_y * movement.velocity;
    }
}