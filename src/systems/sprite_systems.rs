

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::components::asset_components::{CurrentAnimationFrame, PlayerTextureHandles};
use crate::components::player_components::{PlayerMarker, RotationDegrees};

pub const TOTAL_TEXTURES: usize = 3;
pub const FRAMES_PER_TEXTURE: usize = 8;
pub const TOTAL_FRAMES: usize = TOTAL_TEXTURES * FRAMES_PER_TEXTURE;

pub struct SpriteSystems;

impl SpriteSystems {
    pub fn change_sprite(
        mut player_query: Query<(
            &RotationDegrees,
            &Velocity,
            &mut CurrentAnimationFrame,
            &mut Sprite,
            &mut Handle<Image>,
        ), With<PlayerMarker>>, // TODO: Make this not player specific
        player_textures_query: Query<&PlayerTextureHandles>, // TODO: Make this not player specific
    ) {
        let player_textures = player_textures_query.single();
        let (rotation_degrees, velocity, current_animation_frame, sprite, mut player_texture) = player_query.single_mut();

        let texture_set = Self::select_texture_set(player_textures, sprite, rotation_degrees);
        *player_texture = Self::select_texture(texture_set, current_animation_frame, velocity);
    }

    fn select_texture_set(texture_sets: &PlayerTextureHandles, mut sprite: Mut<Sprite>, rotation_degrees: &RotationDegrees) -> Vec<Handle<Image>> {
        let mut texture_set = texture_sets.back.clone();
        sprite.flip_x = false;
        if rotation_degrees.0 > 0. && rotation_degrees.0 < 180. {
            sprite.flip_x = true;
            texture_set = texture_sets.right.clone();
        } else if rotation_degrees.0 > 180. {
            texture_set = texture_sets.right.clone();
        } else if rotation_degrees.0 == 180. {
            texture_set = texture_sets.front.clone();
        };

        texture_set
    }

    fn select_texture(texture_set: Vec<Handle<Image>>, mut current_animation_frame: Mut<CurrentAnimationFrame>, velocity: &Velocity) -> Handle<Image> {
        if velocity.linvel.x == 0. && velocity.linvel.y == 0. || current_animation_frame.0 >= TOTAL_FRAMES {
            current_animation_frame.0 = 1;
        } else  {
            current_animation_frame.0 += 1;
        }

        let current_texture_index = (current_animation_frame.0 -1) / FRAMES_PER_TEXTURE;
        texture_set[current_texture_index].clone()
    }
}