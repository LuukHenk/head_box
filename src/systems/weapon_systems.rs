
use std::time::Duration;

use bevy::prelude::*;
use crate::components::asset_components::{PistolSoundHandle, PistolTextureHandle};

use crate::components::generic_components::GameScreenMarker;
use crate::components::player_components::PlayerMarker;
use crate::components::weapon_components::{ActiveWeapon, BulletsRotationOffsetPerShot, DamagePerHit, WeaponMarker, WeaponType, Owner, AttackCoolDownTimer};
use crate::events::atttack_events::{BulletSpawnEvent, AttackRequestEvent, WeaponSelectionEvent};

#[derive(Bundle)]
struct Weapon {
    // Game screen components
    game_screen_marker: GameScreenMarker,

    // Weapon specific components
    attack_cooldown_timer: AttackCoolDownTimer,
    damage_per_hit: DamagePerHit,
    weapon_marker: WeaponMarker,
    weapon_type: WeaponType,
    bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot,
    attacking_sound: Handle<AudioSource>,
    owner: Owner,

    // Physics
    transform: Transform,
    global_transform: GlobalTransform,

    // Visibility
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
}


pub struct WeaponSystems;

impl WeaponSystems {
    pub fn spawn_default_player_weapons(
        mut commands: Commands,
        pistol_sound: Query<&PistolSoundHandle>,
        player_query: Query<(Entity, &Transform), With<PlayerMarker>>,
        pistol_texture_handle_query: Query<&PistolTextureHandle>,
    ) {
        let (player_entity_id, player_transform) = player_query.single();
        let pistol_texture_handle = pistol_texture_handle_query.single();
        let pistol = Weapon {
            // Game screen components
            game_screen_marker: GameScreenMarker,

            // Weapon specific components
            attack_cooldown_timer: AttackCoolDownTimer(Timer::new(
                Duration::from_secs_f32(1.),
                TimerMode::Once,
            )),
            weapon_marker: WeaponMarker,
            damage_per_hit: DamagePerHit(2.),
            weapon_type: WeaponType::Pistol,
            bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot(vec![0_f32]),
            attacking_sound: pistol_sound.single().0.clone(),
            owner: Owner(Option::Some(player_entity_id)),

            // Physics
            transform: *player_transform,
            global_transform: GlobalTransform::default(),

            // Visibility
            texture: pistol_texture_handle.0.clone(),
            sprite: Sprite::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        };

        commands.spawn((pistol, ActiveWeapon));

    }

    pub fn attack(
        mut player_attack_event: EventReader<AttackRequestEvent>,
        mut bullet_spawn_event: EventWriter<BulletSpawnEvent>,
        mut active_weapon_query: Query<(&mut AttackCoolDownTimer, &BulletsRotationOffsetPerShot), With<ActiveWeapon>>,
        time: Res<Time>,
    ) {
        let (mut cooldown_timer, bullets_rotation_offset) = active_weapon_query.single_mut();
        cooldown_timer.0.tick(time.delta());

        for _attack_event in player_attack_event.read() {
            if cooldown_timer.0.finished() {
                for bullet in bullets_rotation_offset.0.to_vec() {
                    bullet_spawn_event.send(BulletSpawnEvent(bullet));
                }
                cooldown_timer.0.reset();
            };
        }
    }

    pub fn set_active_weapon(
        mut commands: Commands,
        mut weapon_selection_events: EventReader<WeaponSelectionEvent>,
        active_weapon_query: Query<Entity, With<ActiveWeapon>>,
        weapon_query: Query<(Entity, &WeaponType), With<WeaponMarker>>
    ) {
        for weapon_selection_event in weapon_selection_events.read() {
            for (weapon_entity, weapon_type) in weapon_query.iter() {
                if &weapon_selection_event.0 == weapon_type {
                    let active_weapon_entity = active_weapon_query.single();
                    commands.entity(active_weapon_entity).remove::<ActiveWeapon>();
                    commands.entity(weapon_entity).insert(ActiveWeapon);
                }
            }
        }
    }
}