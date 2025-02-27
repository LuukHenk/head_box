use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(Vec3);

#[derive(Component)]
pub struct RotationDegrees(pub f32);

const SPEED: f32 = 100.0;

pub fn handle_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Velocity, &mut RotationDegrees)>,
) {
    for (mut input, mut velocity, mut rotation) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }
        println!("Input {}", input.0);
        velocity.0 = input.extend(0.0).normalize_or_zero() * SPEED;
        rotation.0 = map_input_to_rotation(input.0, rotation.0);
    }
}

pub fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
    ) in query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        input.0 = Vec2::ZERO;
    }
}

pub fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
        // println!("{}", transform.translation);
    }
}

fn map_input_to_rotation(input: Vec2, current_rotation: f32) -> f32 {
    if input.x < 0. && input.y == 0. {
        90.0_f32
    } else if input.x < 0. && input.y > 0. {
        45.0_f32
    } else if input.x == 0. && input.y > 0. {
        0.0_f32
    } else if input.x > 0. && input.y > 0. {
        315.0_f32
    } else if input.x > 0. && input.y == 0. {
        270.0_f32
    } else if input.x > 0. && input.y < 0. {
        225.0_f32
    } else if input.x == 0. && input.y < 0. {
        180.0_f32
    } else if input.x < 0. && input.y < 0. {
        135.0_f32
    } else {
        current_rotation
    }
}
