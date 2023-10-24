use bevy::prelude::{Quat, Query, Transform, With};
use bevy_rapier2d::prelude::{RigidBody, Velocity};

pub struct RigidBodySystems;

impl RigidBodySystems {

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