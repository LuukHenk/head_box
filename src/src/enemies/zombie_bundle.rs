

use bevy::prelude::*;
use super::game_components::*;
use super::game_constants::*;
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
        let y = if rand::random::<bool>() {1.}  else {-1.};
        let x = if rand::random::<bool>() {1.}  else {-1.};
        commands.spawn((ZombieBundle::new(SCREEN_CENTER * x, OUTER_Y_COORDINATES * y), GameScreenMarker));
    }

    pub fn set_directions(
        mut enemy_query: Query<(&mut Movement, &Transform), With<EnemyMarker>>,
        player_query: Query<&Transform, With<PlayerMarker>>
    ) {
        let player_transform = player_query.single();
        let player_position = player_transform.translation;
        for (mut enemy_movement, enemy_transform) in enemy_query.iter_mut() {
            let enemy_position = enemy_transform.translation;
            let distance_x_with_target =  player_position[0] - enemy_position[0];
            let distance_y_with_target =  player_position[1] - enemy_position[1];
            enemy_movement.direction_x = ZombieBundle::set_direction(
                distance_x_with_target,
                enemy_movement.velocity
            );
            enemy_movement.direction_y = ZombieBundle::set_direction(
                distance_y_with_target,
                enemy_movement.velocity
            );
            println!("{:#?}", enemy_movement);
        }
    }

    fn set_direction(target_distance: f32, enemy_velocity: f32) -> f32 {
        if target_distance > enemy_velocity {1.} else if target_distance < -enemy_velocity {-1.} else {0.}
    }
}
