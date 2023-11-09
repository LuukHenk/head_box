
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::components::asset_components::{CharacterTextureHandles, CurrentAnimationFrame, PistolSoundHandle, PistolTextureMarker};

use crate::components::generic_components::GameScreenMarker;
use crate::components::physics_components::RotationDegrees;
use crate::components::player_components::PlayerMarker;
use crate::components::weapon_components::{ActiveWeapon, BulletsRotationOffsetPerShot, DamagePerHit, WeaponMarker, WeaponType, Owner, AttackCoolDownTimer, WeaponOwnerMarker};
use crate::events::atttack_events::{BulletSpawnEvent, AttackRequestEvent, WeaponSelectionEvent};

#[derive(Bundle)]
struct Weapon {
    // Marker components
    game_screen_marker: GameScreenMarker,
    weapon_marker: WeaponMarker,

    // Weapon specific components
    attack_cooldown_timer: AttackCoolDownTimer,
    damage_per_hit: DamagePerHit,
    weapon_type: WeaponType,
    bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot,
    attacking_sound: Handle<AudioSource>,
    owner: Owner,

    // Physics
    rotation_degrees: RotationDegrees,
    velocity: Velocity,
    transform: Transform,
    global_transform: GlobalTransform,

    // Visibility
    current_animation_frame: CurrentAnimationFrame,
    character_texture_handles: CharacterTextureHandles,
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
        player_query: Query<(Entity, &Transform, &RotationDegrees), With<PlayerMarker>>,
        pistol_texture_handles_query: Query<&CharacterTextureHandles, With<PistolTextureMarker>>,
    ) {
        let (player_entity_id, player_transform, player_rotation) = player_query.single();
        let pistol_texture_handles = pistol_texture_handles_query.single();
        let current_texture = pistol_texture_handles.front[0].clone();

        let mut pistol_transform = player_transform.clone();
        pistol_transform.translation = Self::set_translation_relative_to_owner(player_transform.translation, player_rotation.0);
        let pistol = Weapon {
            // Marker components
            game_screen_marker: GameScreenMarker,
            weapon_marker: WeaponMarker,

            // Weapon specific components
            attack_cooldown_timer: AttackCoolDownTimer(Timer::new(
                Duration::from_secs_f32(1.),
                TimerMode::Once,
            )),
            damage_per_hit: DamagePerHit(2.),
            weapon_type: WeaponType::Pistol,
            bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot(vec![0_f32]),
            attacking_sound: pistol_sound.single().0.clone(),
            owner: Owner(Option::Some(player_entity_id)),

            // Physics
            velocity: Velocity::default(),
            rotation_degrees: RotationDegrees(180.),
            transform: pistol_transform,
            global_transform: GlobalTransform::default(),

            // Visibility
            current_animation_frame: CurrentAnimationFrame(1),
            character_texture_handles: pistol_texture_handles.clone(),
            texture: current_texture,
            sprite: Sprite::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        };

        commands.spawn((pistol, ActiveWeapon));

    }

    pub fn update_transform(
        weapon_owner_query: Query<(Entity, &Transform, &RotationDegrees), With<WeaponOwnerMarker>>,
        mut weapon_query: Query<(&Owner, &mut Transform, &mut RotationDegrees), (With<WeaponMarker>, Without<WeaponOwnerMarker>)>,
    ) {
        for (owner, mut weapon_transform, mut rotation_degrees) in weapon_query.iter_mut() {
            if owner.0.is_none() {continue};
            let weapon_owner_id = owner.0.unwrap();
            for (found_owner_id, owner_transform, owner_rotation) in weapon_owner_query.iter() {
                if weapon_owner_id == found_owner_id {
                    weapon_transform.translation = Self::set_translation_relative_to_owner(owner_transform.translation, owner_rotation.0);
                    rotation_degrees.0 = owner_rotation.0;
                }
            }
        }
    }

    fn set_translation_relative_to_owner(
        owner_translation: Vec3,
        owner_rotation: f32,
    ) -> Vec3 {
        let translation: Vec3;
        if owner_rotation > 0. && owner_rotation < 180. {
            translation = Vec3::new(
                owner_translation.x - 11_f32,
                owner_translation.y - 5_f32,
                owner_translation.z + 1_f32,
            )
        } else if owner_rotation > 180. {
            translation = Vec3::new(
                owner_translation.x - 11_f32,
                owner_translation.y - 5_f32,
                owner_translation.z + 1_f32,
            )
        } else if owner_rotation == 180. {
            translation = Vec3::new(
                owner_translation.x - 11_f32,
                owner_translation.y - 5_f32,
                owner_translation.z + 1_f32,
            )
        } else {
            translation = Vec3::new(
                owner_translation.x - 11_f32,
                owner_translation.y - 5_f32,
                owner_translation.z + 1_f32,
            )
        };
        translation
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