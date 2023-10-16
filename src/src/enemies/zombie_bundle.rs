

use bevy::prelude::*;
use super::game_components::*;
use super::game_constants::*;
use super::set_directions_to_target;
#[derive(Bundle)]
pub struct ZombieBundle {
    sprite_bundle: SpriteBundle,
    movement: Movement,
    enemy_marker: EnemyMarker,
    collision_marker: CollisionMarker,
}
impl ZombieBundle {
    fn new(x: f32, y: f32) -> ZombieBundle {
        ZombieBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::LIME_GREEN, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                velocity: 2.,
            },
            enemy_marker: EnemyMarker,
            collision_marker: CollisionMarker,
        }
    }

    pub fn spawn(mut commands: Commands) {
        let y = if rand::random::<bool>() { 1. } else { -1. };
        let x = if rand::random::<bool>() { 1. } else { -1. };
        commands.spawn((ZombieBundle::new(SCREEN_CENTER * x, 100. * y), GameScreenMarker));
    }

    pub fn set_directions(
        mut enemy_query: Query<(&mut Movement, &Transform), With<EnemyMarker>>,
        player_query: Query<&Transform, With<PlayerMarker>>
    ) {
        for player_transform in player_query.iter() {
            let player_position = player_transform.translation;
            for (enemy_movement, enemy_transform) in enemy_query.iter_mut() {
                let enemy_position = enemy_transform.translation;
                set_directions_to_target(enemy_movement, enemy_position, player_position);
            }
        }
    }
}
