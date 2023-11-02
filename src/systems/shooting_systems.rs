use std::time::Duration;

use bevy::prelude::*;

use crate::components::generic_components::GameScreenMarker;
use crate::components::shooting_components::{ActiveGun, GunMarker, GunName, ShootingCoolDownTimer};

#[derive(Bundle)]
struct Gun {
    // Game screen components
    game_screen_marker: GameScreenMarker,

    // Gun specific components
    shooting_cooldown_timer: ShootingCoolDownTimer,
    gun_marker: GunMarker,
    gun_name: GunName,
}


pub struct ShootingSystems;

impl ShootingSystems {
    pub fn spawn_guns(mut commands: Commands) {
        let pistol = Gun {
            // Game screen components
            game_screen_marker: GameScreenMarker,

            // Gun specific components
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(
                Duration::from_secs_f32(0.8),
                TimerMode::Once,
            )),
            gun_marker: GunMarker,
            gun_name: GunName("Pistol".parse().unwrap()),
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
            gun_name: GunName("Uzi".parse().unwrap()),
        };

        commands.spawn((pistol, ActiveGun));
        commands.spawn(uzi);

    }

}