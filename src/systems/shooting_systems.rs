
use std::time::Duration;

use bevy::prelude::*;
use crate::components::asset_components::PistolSoundHandle;

use crate::components::generic_components::GameScreenMarker;
use crate::components::player_components::PlayerMarker;
use crate::components::shooting_components::{ActiveGun, BulletsRotationOffsetPerShot, DamagePerHit, GunMarker, GunType, Owner, ShootingCoolDownTimer};
use crate::events::shooting_events::{BulletSpawnEvent, ShootRequestEvent, WeaponSelectionEvent};

#[derive(Bundle)]
struct Gun {
    // Game screen components
    game_screen_marker: GameScreenMarker,

    // Gun specific components
    shooting_cooldown_timer: ShootingCoolDownTimer,
    damage_per_hit: DamagePerHit,
    gun_marker: GunMarker,
    gun_type: GunType,
    bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot,
    shooting_sound: Handle<AudioSource>,
    owner: Owner,
}


pub struct ShootingSystems;

impl ShootingSystems {
    pub fn spawn_default_player_weapons(
        mut commands: Commands,
        pistol_sound: Query<&PistolSoundHandle>,
        player_query: Query<Entity, With<PlayerMarker>>,
    ) {
        let player_entity_id = player_query.single();
        let pistol = Gun {
            // Game screen components
            game_screen_marker: GameScreenMarker,

            // Gun specific components
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(
                Duration::from_secs_f32(1.),
                TimerMode::Once,
            )),
            gun_marker: GunMarker,
            damage_per_hit: DamagePerHit(2.),
            gun_type: GunType::Pistol,
            bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot(vec![0_f32]),
            shooting_sound: pistol_sound.single().0.clone(),
            owner: Owner(Option::Some(player_entity_id)),
        };

        commands.spawn((pistol, ActiveGun));

    }

    pub fn shoot(
        mut player_shoot_event: EventReader<ShootRequestEvent>,
        mut bullet_spawn_event: EventWriter<BulletSpawnEvent>,
        mut active_gun_query: Query<(&mut ShootingCoolDownTimer, &BulletsRotationOffsetPerShot), With<ActiveGun>>,
        time: Res<Time>,
    ) {
        let (mut cooldown_timer, bullets_rotation_offset) = active_gun_query.single_mut();
        cooldown_timer.0.tick(time.delta());

        for _shoot_event in player_shoot_event.read() {
            if cooldown_timer.0.finished() {
                for bullet in bullets_rotation_offset.0.to_vec() {
                    bullet_spawn_event.send(BulletSpawnEvent(bullet));
                }
                cooldown_timer.0.reset();
            };
        }
    }

    pub fn set_active_gun(
        mut commands: Commands,
        mut weapon_selection_events: EventReader<WeaponSelectionEvent>,
        active_gun_query: Query<Entity, With<ActiveGun>>,
        gun_query: Query<(Entity, &GunType), With<GunMarker>>
    ) {
        for weapon_selection_event in weapon_selection_events.read() {
            for (gun_entity, gun_type) in gun_query.iter() {
                if &weapon_selection_event.0 == gun_type {
                    let active_gun_entity = active_gun_query.single();
                    commands.entity(active_gun_entity).remove::<ActiveGun>();
                    commands.entity(gun_entity).insert(ActiveGun);
                }
            }
        }
    }
}