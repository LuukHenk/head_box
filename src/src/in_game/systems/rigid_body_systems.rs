use std::f32::consts::PI;
use bevy::prelude::{Quat, Query, Transform, With};
use bevy_rapier2d::prelude::{RigidBody, Velocity};

pub struct RigidBodySystems;

impl RigidBodySystems {

    pub fn rotate(mut query: Query<(&mut Transform, &Velocity)>) {
        let (mut transform, velocity) = query.single_mut();
        let mut degrees = transform.rotation.to_axis_angle().1 * 180.0 / PI;

        println!("Input: {:#?}", degrees);

        if velocity.linvel[0] < 0. && velocity.linvel[1] == 0. {degrees = 90.0_f32}
        else if velocity.linvel[0] < 0. && velocity.linvel[1] > 0. {degrees = 45.0_f32}
        else if velocity.linvel[0] == 0. && velocity.linvel[1] > 0. {degrees = 0.0_f32}
        else if velocity.linvel[0] > 0. && velocity.linvel[1] > 0. {degrees = 315.0_f32}
        else if velocity.linvel[0] > 0. && velocity.linvel[1] == 0. {degrees = 270.0_f32}
        else if velocity.linvel[0] > 0. && velocity.linvel[1] < 0. {degrees = 225.0_f32}
        else if velocity.linvel[0] == 0. && velocity.linvel[1] < 0. {degrees = 180.0_f32}
        else if velocity.linvel[0] < 0. && velocity.linvel[1] < 0. {degrees = 135.0_f32};

        println!("output: {:#?}", degrees);

        transform.rotation = Quat::from_rotation_z(degrees.to_radians());
    }
    pub fn rotate_single(mut transform: Transform, velocity: &Velocity) -> Transform{
        if velocity.linvel[0] < 0. && velocity.linvel[1] == 0. {
            transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians())

        } else if velocity.linvel[0] < 0. && velocity.linvel[1] > 0. {
            transform.rotation = Quat::from_rotation_z(45.0_f32.to_radians())

        } else if velocity.linvel[0] == 0. && velocity.linvel[1] > 0. {
            transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians())

        } else if velocity.linvel[0] > 0. && velocity.linvel[1] > 0. {
            transform.rotation = Quat::from_rotation_z(315.0_f32.to_radians())

        } else if velocity.linvel[0] > 0. && velocity.linvel[1] == 0. {
            transform.rotation = Quat::from_rotation_z(270.0_f32.to_radians())

        } else if velocity.linvel[0] > 0. && velocity.linvel[1] < 0. {
            transform.rotation = Quat::from_rotation_z(225.0_f32.to_radians())

        } else if velocity.linvel[0] == 0. && velocity.linvel[1] < 0. {
            transform.rotation = Quat::from_rotation_z(180.0_f32.to_radians())

        } else if velocity.linvel[0] < 0. && velocity.linvel[1] < 0. {
            transform.rotation = Quat::from_rotation_z(135.0_f32.to_radians())
        };
        transform
    }
}