use bevy::audio::Volume;
use bevy::prelude::*;
use crate::components::asset_components::{PistolSoundHandle, ZombieTenseSoundHandle};
use crate::components::generic_components::GameScreenMarker;
use crate::components::sound_components::ZombieTenseSound;
use crate::events::shooting_events::BulletSpawnEvent;


// const MAX_ZOMBIE_SOUND_DISTANCE: f32 = 300.;
// const MIN_ZOMBIE_SOUND_DISTANCE: f32 = 60.;
// const ZOMBIE_SOUND_RANGE: f32 = MAX_ZOMBIE_SOUND_DISTANCE - MIN_ZOMBIE_SOUND_DISTANCE;
// const MAX_ZOMBIE_VOLUME_LEVEL: f32 = 1.;

pub struct SoundSystems;


impl SoundSystems {
    pub fn play_zombie_tense_sounds(mut commands: Commands, sound_query: Query<&ZombieTenseSoundHandle>) {
        let mut playback_settings = PlaybackSettings::LOOP;
        playback_settings = playback_settings.with_volume(Volume::new_relative(0.2));
        commands.spawn(
            (
                AudioBundle {
                    source: sound_query.single().0.clone(),
                    settings: playback_settings,
            },
                ZombieTenseSound,
                GameScreenMarker,
            )
        );
    }

    // pub fn adjust_zombie_sounds(
    //     music_controller: Query<&AudioSink, With<ZombieTenseSound>>,
    //     zombie_location_query: Query<&Transform, With<ZombieMarker>>,
    //     player_location_query: Query<&Transform, With<PlayerMarker>>,
    // ) {
    //     let sink = music_controller.single();
    //     let mut closest_zombie_distance = MAX_ZOMBIE_SOUND_DISTANCE;
    //     for zombie_location in zombie_location_query.iter() {
    //         for player_location in player_location_query.iter() {
    //             let zombie_distance_x = (zombie_location.translation.x - player_location.translation.x).abs();
    //             let zombie_distance_y = (zombie_location.translation.y - player_location.translation.y).abs();
    //             let total_distance = zombie_distance_x + zombie_distance_y;
    //             if total_distance < closest_zombie_distance {
    //                 closest_zombie_distance = total_distance;
    //             }
    //         }
    //     }
    //
    //     let volume_level = MAX_ZOMBIE_VOLUME_LEVEL - ((closest_zombie_distance - MIN_ZOMBIE_SOUND_DISTANCE) * 0.9 / ZOMBIE_SOUND_RANGE );
    //     sink.set_volume(volume_level);
    // }

    pub fn play_shooting_sound(
        mut commands: Commands,
        mut bullet_spawn_event: EventReader<BulletSpawnEvent>,
        sound_query: Query<&PistolSoundHandle>
    ) {
        let sound = sound_query.single();
        for _shoot_event in bullet_spawn_event.iter() {
            commands.spawn(AudioBundle {
                source: sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}