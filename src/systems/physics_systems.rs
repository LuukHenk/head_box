





use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::player_components::PlayerMarker;
use crate::components::physics_components::RotationDegrees;


pub struct PhysicsSystems;

impl PhysicsSystems {
    pub fn set_rotation_degrees(
        mut query: Query<(&mut RotationDegrees, &Velocity), With<PlayerMarker>>,
    ) {
        let (mut rotation_degrees, velocity) = query.single_mut();
        if velocity.linvel[0] < 0. && velocity.linvel[1] == 0. {
            rotation_degrees.0 = 90.0_f32
        } else if velocity.linvel[0] < 0. && velocity.linvel[1] > 0. {
            rotation_degrees.0 = 45.0_f32
        } else if velocity.linvel[0] == 0. && velocity.linvel[1] > 0. {
            rotation_degrees.0 = 0.0_f32
        } else if velocity.linvel[0] > 0. && velocity.linvel[1] > 0. {
            rotation_degrees.0 = 315.0_f32
        } else if velocity.linvel[0] > 0. && velocity.linvel[1] == 0. {
            rotation_degrees.0 = 270.0_f32
        } else if velocity.linvel[0] > 0. && velocity.linvel[1] < 0. {
            rotation_degrees.0 = 225.0_f32
        } else if velocity.linvel[0] == 0. && velocity.linvel[1] < 0. {
            rotation_degrees.0 = 180.0_f32
        } else if velocity.linvel[0] < 0. && velocity.linvel[1] < 0. {
            rotation_degrees.0 = 135.0_f32
        };
    }
}