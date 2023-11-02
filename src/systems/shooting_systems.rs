use std::time::Duration;

use bevy::prelude::*;
use crate::components::asset_components::{PistolSoundHandle, UziSoundHandle};

use crate::components::generic_components::GameScreenMarker;
use crate::components::shooting_components::{ActiveGun, DamagePerHit, GunMarker, GunType, ShootingCoolDownTimer};
use crate::events::shooting_events::{BulletSpawnEvent, ShootRequestEvent};

#[derive(Bundle)]
struct Gun {
    // Game screen components
    game_screen_marker: GameScreenMarker,

    // Gun specific components
    shooting_cooldown_timer: ShootingCoolDownTimer,
    damage_per_hit: DamagePerHit,
    gun_marker: GunMarker,
    gun_type: GunType,
    shooting_sound: Handle<AudioSource>
}


pub struct ShootingSystems;

impl ShootingSystems {
    pub fn spawn_guns(
        mut commands: Commands,
        pistol_sound: Query<&PistolSoundHandle>,
        uzi_sound: Query<&UziSoundHandle>
    ) {
        let pistol = Gun {
            // Game screen components
            game_screen_marker: GameScreenMarker,

            // Gun specific components
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(
                Duration::from_secs_f32(0.8),
                TimerMode::Once,
            )),
            gun_marker: GunMarker,
            damage_per_hit: DamagePerHit(2.),
            gun_type: GunType::Pistol,
            shooting_sound: pistol_sound.single().0.clone(),
        };
        let uzi = Gun {
            // Game screen components
            game_screen_marker: GameScreenMarker,

            // Gun specific components
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(
                Duration::from_secs_f32(0.1),
                TimerMode::Once,
            )),
            gun_marker: GunMarker,
            damage_per_hit: DamagePerHit(0.05),
            gun_type: GunType::Uzi,
            shooting_sound: uzi_sound.single().0.clone(),
        };

        commands.spawn((pistol, ActiveGun));
        commands.spawn((uzi));

    }

    pub fn shoot(
        mut player_shoot_event: EventReader<ShootRequestEvent>,
        mut bullet_spawn_event: EventWriter<BulletSpawnEvent>,
        mut active_gun_query: Query<&mut ShootingCoolDownTimer, With<ActiveGun>>,
        time: Res<Time>,
    ) {
        let mut cooldown_timer = active_gun_query.single_mut();
        cooldown_timer.0.tick(time.delta());

        for _shoot_event in player_shoot_event.iter() {
            if cooldown_timer.0.finished() {
                bullet_spawn_event.send(BulletSpawnEvent);
                cooldown_timer.0.reset();
            };
        }
    }

}