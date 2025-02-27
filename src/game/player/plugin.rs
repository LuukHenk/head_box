//! https://bevyengine.org/examples/movement/physics-in-fixed-timestep/

use bevy::prelude::*;

use crate::{despawn_entities, GameState};

use super::physics::{
    advance_physics, handle_movement_input, interpolate_rendered_transform, AccumulatedInput, PhysicalTranslation, PreviousPhysicalTranslation, RotationDegrees, Velocity
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnExit(GameState::Game), despawn_entities::<Player>);
        app.add_systems(
            FixedUpdate,
            (advance_physics).run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            RunFixedMainLoop,
            (
                (handle_movement_input).in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                (interpolate_rendered_transform, update_camera)
                    .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            )
                .run_if(in_state(GameState::Game)),
        );
    }
}

const CAMERA_DECAY_RATE: f32 = 2.;

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(CircularSector::new(50.0, 1.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_scale(Vec3::splat(0.3)),
        RotationDegrees::default(),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
