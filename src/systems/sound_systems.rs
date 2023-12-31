use bevy::audio::Volume;
use bevy::prelude::*;
use crate::components::asset_components::{InGameMusicHandle, MenuMusicHandle, ZombieTenseSoundHandle};
use crate::components::generic_components::GameScreenMarker;
use crate::components::weapon_components::ActiveWeapon;
use crate::components::sound_components::{BackgroundMusic, InGameBackgroundSound, MenuBackgroundMusic, ZombieTenseSound};
use crate::events::atttack_events::BulletSpawnEvent;


// const MAX_ZOMBIE_SOUND_DISTANCE: f32 = 300.;
// const MIN_ZOMBIE_SOUND_DISTANCE: f32 = 60.;
// const ZOMBIE_SOUND_RANGE: f32 = MAX_ZOMBIE_SOUND_DISTANCE - MIN_ZOMBIE_SOUND_DISTANCE;
// const MAX_ZOMBIE_VOLUME_LEVEL: f32 = 1.;

pub struct SoundSystems;


impl SoundSystems {
    pub fn spawn_zombie_tense_sounds(mut commands: Commands, sound_query: Query<&ZombieTenseSoundHandle>) {
        let mut playback_settings = PlaybackSettings::LOOP;
        playback_settings = playback_settings.with_volume(Volume::new_relative(0.3));
        commands.spawn(
            (
                AudioBundle {
                    source: sound_query.single().0.clone(),
                    settings: playback_settings,
            },
                ZombieTenseSound,
                GameScreenMarker,
                InGameBackgroundSound,
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

    pub fn spawn_menu_music(mut commands: Commands, sound_query: Query<&MenuMusicHandle>) {
        let mut playback_settings = PlaybackSettings::LOOP;
        playback_settings = playback_settings.with_volume(Volume::new_relative(0.5));
        commands.spawn(
            (
                AudioBundle {
                    source: sound_query.single().0.clone(),
                    settings: playback_settings,
                },
                BackgroundMusic,
                GameScreenMarker,
                MenuBackgroundMusic,
            )
        );
    }
    pub fn toggle_menu_music(music_controller: Query<&AudioSink, With<MenuBackgroundMusic>>) {
        for sink in music_controller.iter() {
            sink.toggle();
        }
    }

    pub fn spawn_in_game_background_sounds(mut commands: Commands, sound_query: Query<&InGameMusicHandle>) {
        let mut playback_settings = PlaybackSettings::LOOP;
        playback_settings = playback_settings.with_volume(Volume::new_relative(0.3));
        commands.spawn(
            (
                AudioBundle {
                    source: sound_query.single().0.clone(),
                    settings: playback_settings,
                },
                BackgroundMusic,
                GameScreenMarker,
                InGameBackgroundSound,
            )
        );
    }

    pub fn toggle_in_game_background_sounds(music_controller: Query<&AudioSink, With<InGameBackgroundSound>>) {
        for sink in music_controller.iter() {
            sink.toggle();
        }
    }
    pub fn play_attack_sound(
        mut commands: Commands,
        mut bullet_spawn_event: EventReader<BulletSpawnEvent>,
        sound_query: Query<&Handle<AudioSource>, With<ActiveWeapon>>,
    ) {
        let sound = sound_query.single();
        let mut playback_settings = PlaybackSettings::DESPAWN;
        playback_settings = playback_settings.with_volume(Volume::new_relative(0.3));
        for _attack_event in bullet_spawn_event.read() {
            commands.spawn(AudioBundle {
                source: sound.clone(),
                settings: playback_settings,
            });
        }
    }
}