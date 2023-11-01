use crate::display_handler::display_handler::InGameCamera;
use crate::in_game::data_classes::player_components::PlayerMarker;
use bevy::prelude::*;

pub const CAMERA_SCALE: f32 = 1_f32;
pub struct CameraSystems;
impl CameraSystems {
    pub fn zoom_camera(mut q: Query<&mut OrthographicProjection, With<InGameCamera>>) {
        let mut projection = q.single_mut();
        projection.scale *= CAMERA_SCALE;
    }

    pub fn reset_zoom(mut q: Query<&mut OrthographicProjection, With<InGameCamera>>) {
        let mut projection = q.single_mut();
        projection.scale *= 1. / CAMERA_SCALE;
    }

    pub fn follow_player(
        mut camera_transform_query: Query<&mut Transform, With<InGameCamera>>,
        mut player_query: Query<&GlobalTransform, With<PlayerMarker>>,
    ) {
        let mut camera_transform = camera_transform_query.single_mut();
        let player_transform = player_query.single_mut();
        camera_transform.translation = player_transform.translation();
    }
}
