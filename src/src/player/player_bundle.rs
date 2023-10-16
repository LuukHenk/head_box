

use bevy::prelude::*;
use super::game_components::*;
use super::game_constants::*;
const INITIAL_PLAYER_HEALTH: f32 = 300.;

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    health: Health,
    sprite_bundle: SpriteBundle,
    movement: Movement,
    collision_marker: CollisionMarker,
}

impl PlayerBundle {
    fn new() -> PlayerBundle {
        PlayerBundle {
            player_marker: PlayerMarker,
            collision_marker: CollisionMarker,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., Z_VALUE),
                    scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::PURPLE, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                velocity: 7.,
            },
            health: Health(INITIAL_PLAYER_HEALTH),
        }
    }

    pub fn spawn(mut commands: Commands) {
        commands.spawn((PlayerBundle::new(), GameScreenMarker));
    }

    pub fn set_direction(keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<&mut Movement, With<PlayerMarker>>) {
        for mut movement in player_query.iter_mut() {
            movement.direction_x = 0.;
            movement.direction_y = 0.;
            if keyboard_input.pressed(KeyCode::Right) {
                movement.direction_x += 1.;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                movement.direction_x -= 1.;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                movement.direction_y += 1.;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                movement.direction_y -= 1.;
            }
        }
    }
}

