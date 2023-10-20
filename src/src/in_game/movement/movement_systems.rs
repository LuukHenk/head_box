


use bevy::prelude::{
    Query,
    Transform,
    With,
    Quat,
};

use super::movement_components::Movement;



pub struct MovementSystems;

impl MovementSystems{
    pub fn move_objects(mut query: Query<(&mut Transform, &mut Movement), With<Movement>>) {
        for (mut transform, mut movement) in query.iter_mut() {
            Self::rotate_object(&mut *transform, &mut *movement);
            transform.translation.x += movement.direction_x * movement.current_velocity;
            transform.translation.y += movement.direction_y * movement.current_velocity;
            movement.current_velocity = movement.default_velocity;
        }
    }

    fn rotate_object(transform: &mut Transform, movement: &mut Movement) {
        if movement.direction_y == 0. && movement.direction_x == -1. {
            transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians())
        } else if movement.direction_y == 1. && movement.direction_x == -1. {
            transform.rotation = Quat::from_rotation_z(45.0_f32.to_radians())
        } else if movement.direction_y == 1. && movement.direction_x == 0. {
            transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians())
        } else if movement.direction_y == 1. && movement.direction_x == 1. {
            transform.rotation = Quat::from_rotation_z(315.0_f32.to_radians())
        } else if movement.direction_y == 0. && movement.direction_x == 1. {
            transform.rotation = Quat::from_rotation_z(270.0_f32.to_radians())
        } else if movement.direction_y == -1. && movement.direction_x == 1. {
            transform.rotation = Quat::from_rotation_z(225.0_f32.to_radians())
        } else if movement.direction_y == -1. && movement.direction_x == 0. {
            transform.rotation = Quat::from_rotation_z(180.0_f32.to_radians())
        } else if movement.direction_y == -1. && movement.direction_x == -1. {
            transform.rotation = Quat::from_rotation_z(135.0_f32.to_radians())
        };
    }
}