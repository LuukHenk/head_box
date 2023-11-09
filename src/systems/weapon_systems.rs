
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;
use bevy_rapier2d::prelude::Velocity;
use crate::components::asset_components::{BulletTextureHandle, CharacterTextureHandles, CurrentAnimationFrame, KnifeAttackTextureHandle, KnifeSoundHandle, KnifeTextureMarker, PistolSoundHandle, PistolTextureMarker};
use crate::components::bullet_components::Damage;

use crate::components::generic_components::GameScreenMarker;
use crate::components::physics_components::RotationDegrees;
use crate::components::player_components::PlayerMarker;
use crate::components::weapon_components::{ActiveWeapon, BulletsRotationOffsetPerShot, BulletTexture, WeaponMarker, WeaponType, Owner, AttackCoolDownTimer, WeaponOwnerMarker, BulletCollisionGroups, BulletCollider};
use crate::events::atttack_events::{BulletSpawnEvent, AttackRequestEvent, WeaponSelectionEvent};
use crate::utils::generic_constants::SCALING;

#[derive(Bundle)]
struct Weapon {
    // Marker components
    game_screen_marker: GameScreenMarker,
    weapon_marker: WeaponMarker,

    // Weapon specific components
    attack_cooldown_timer: AttackCoolDownTimer,
    damage: Damage,
    weapon_type: WeaponType,
    attacking_sound: Handle<AudioSource>,
    owner: Owner,

    // Bullet specific components
    bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot,
    bullet_collision_groups: BulletCollisionGroups,
    bullet_texture: BulletTexture,
    bullet_collider: BulletCollider,

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

const BULLET_TO_WEAPON_OFFSET_Y: f32 = 2.;
pub struct WeaponSystems;

impl WeaponSystems {
    // pub fn spawn_default_player_weapons(
    //     mut commands: Commands,
    //     pistol_sound: Query<&PistolSoundHandle>,
    //     player_query: Query<(Entity, &Transform, &RotationDegrees, &CollisionGroups), With<PlayerMarker>>,
    //     pistol_texture_handles_query: Query<&CharacterTextureHandles, With<PistolTextureMarker>>,
    //     bullet_texture_query: Query<&BulletTextureHandle>,
    // ) {
    //     let (player_entity_id, player_transform, player_rotation, player_collision_groups) = player_query.single();
    //     let pistol_texture_handles = pistol_texture_handles_query.single();
    //
    //
    //     let mut pistol_transform = player_transform.clone();
    //     pistol_transform.translation = Self::set_translation_relative_to_owner(player_transform.translation, player_rotation.0);
    //
    //     let pistol = Self::new_pistol(
    //         pistol_sound.single().0.clone(),
    //         Owner(Option::Some(player_entity_id)),
    //         BulletCollisionGroups(*player_collision_groups),
    //         pistol_texture_handles.clone(),
    //         BulletTexture(bullet_texture_query.single().0.clone()),
    //         pistol_transform
    //     );
    //     commands.spawn((pistol, ActiveWeapon));
    //
    // }
    pub fn spawn_default_player_weapons(
        mut commands: Commands,
        player_query: Query<(Entity, &Transform, &RotationDegrees, &CollisionGroups), With<PlayerMarker>>,
        knife_sound_query: Query<&KnifeSoundHandle>,
        knife_texture_handles_query: Query<&CharacterTextureHandles, With<KnifeTextureMarker>>,
        attack_texture_handle_query: Query<&KnifeAttackTextureHandle>,
    ) {
        let (player_entity_id, player_transform, player_rotation, player_collision_groups) = player_query.single();
        let pistol_texture_handles = knife_texture_handles_query.single();


        let mut knife_transform = player_transform.clone();
        knife_transform.translation = Self::set_translation_relative_to_owner(player_transform.translation, player_rotation.0);
        
        let knife = Self::new_knife(
            knife_sound_query.single().0.clone(),
            Owner(Option::Some(player_entity_id)),
            BulletCollisionGroups(*player_collision_groups),
            pistol_texture_handles.clone(),
            BulletTexture(attack_texture_handle_query.single().0.clone()),
            knife_transform
        );
        commands.spawn((knife, ActiveWeapon));

    }

    fn random_secondary_function(x: Query<&Transform, With<PlayerMarker>>) {
        println!("hi")
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

    pub fn hide_weapon_when_owner_is_moving(
        weapon_owner_query: Query<(Entity, &Velocity), With<WeaponOwnerMarker>>,
        mut weapon_query: Query<(&Owner, &mut Visibility), (With<WeaponMarker>, Without<WeaponOwnerMarker>)>,
    ) {
        for (owner, mut visibility) in weapon_query.iter_mut() {
            if owner.0.is_none() {continue};
            let weapon_owner_id = owner.0.unwrap();
            for (found_owner_id, velocity) in weapon_owner_query.iter() {
                if weapon_owner_id == found_owner_id {
                    if velocity.linvel.x != 0. || velocity.linvel.y != 0. {
                        *visibility = Visibility::Hidden
                    } else {
                        *visibility = Visibility::Visible
                    }
                }
            }
        }
    }


    pub fn cooldown_weapons(
        mut query: Query<&mut AttackCoolDownTimer, With<WeaponMarker>>,
        time: Res<Time>,
    ) {
        let mut cooldown_timer = query.single_mut();
        cooldown_timer.0.tick(time.delta());
    }
    pub fn attack(
        mut attack_request_event_reader: EventReader<AttackRequestEvent>,
        mut bullet_spawn_event_writer: EventWriter<BulletSpawnEvent>,
        mut weapons_query: Query<(
            &Owner,
            &mut AttackCoolDownTimer,
            &BulletsRotationOffsetPerShot,
            &Transform,
            &BulletCollisionGroups,
            &Damage,
            &BulletTexture,
            &RotationDegrees,
            &BulletCollider,
        ), With<WeaponMarker>>
    ) {
        for attack_event in attack_request_event_reader.read() {
            for (owner, mut cooldown_timer, rotation_offset, transform, bullet_collision_groups, damage, bullet_texture, weapon_rotation, bullet_collider) in weapons_query.iter_mut() {
                if owner.0.is_some() && attack_event.0 == owner.0.unwrap() && cooldown_timer.0.finished() {
                    for bullet_rotation_offset in rotation_offset.0.to_vec() {
                        let bullet_rotation = weapon_rotation.0 + bullet_rotation_offset;
                        let bullet_rotation_quat = Quat::from_rotation_z(bullet_rotation.to_radians());
                        let bullet_direction = (bullet_rotation_quat * Vec3::Y).truncate().normalize();
                        let bullet_transform = Self::generate_bullet_transform(
                            bullet_rotation_quat,
                            transform,
                            bullet_direction,
                            bullet_collider.0.y,

                        );
                        let bullet_spawn_event = BulletSpawnEvent{
                            transform: bullet_transform,
                            collision_groups: bullet_collision_groups.0,
                            damage: Damage(damage.0),
                            texture: bullet_texture.0.clone(),
                            collider: bullet_collider.0,
                        };
                        bullet_spawn_event_writer.send(bullet_spawn_event);
                    }
                    cooldown_timer.0.reset();
                }
            }

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

    fn new_pistol(
        sound: Handle<AudioSource>,
        owner: Owner,
        bullet_collision_groups: BulletCollisionGroups,
        texture_handles: CharacterTextureHandles,
        bullet_texture: BulletTexture,
        transform: Transform,
    ) -> Weapon {
        let current_texture = texture_handles.front[0].clone();
        Weapon {
            // Marker components
            game_screen_marker: GameScreenMarker,
            weapon_marker: WeaponMarker,

            // Weapon specific components
            attack_cooldown_timer: AttackCoolDownTimer(Timer::new(
                Duration::from_secs_f32(1.),
                TimerMode::Once,
            )),
            damage: Damage(2.),
            weapon_type: WeaponType::Pistol,
            attacking_sound: sound,
            owner,

            // Bullet components
            bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot(vec![0_f32]),
            bullet_collision_groups,
            bullet_texture,
            bullet_collider: BulletCollider(Vec2::new(0.5, 100.)),

            // Physics
            velocity: Velocity::default(),
            rotation_degrees: RotationDegrees(180.),
            transform,
            global_transform: GlobalTransform::default(),

            // Visibility
            current_animation_frame: CurrentAnimationFrame(1),
            character_texture_handles: texture_handles,
            texture: current_texture,
            sprite: Sprite::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }

    fn new_knife(
        sound: Handle<AudioSource>,
        owner: Owner,
        bullet_collision_groups: BulletCollisionGroups,
        texture_handles: CharacterTextureHandles,
        bullet_texture: BulletTexture,
        transform: Transform,
    ) -> Weapon {
        let current_texture = texture_handles.front[0].clone();
        Weapon {
            // Marker components
            game_screen_marker: GameScreenMarker,
            weapon_marker: WeaponMarker,

            // Weapon specific components
            attack_cooldown_timer: AttackCoolDownTimer(Timer::new(
                Duration::from_secs_f32(1.),
                TimerMode::Once,
            )),
            damage: Damage(10.),
            weapon_type: WeaponType::Knife,
            attacking_sound: sound,
            owner,

            // Bullet components
            bullets_rotation_offset_per_shot: BulletsRotationOffsetPerShot(vec![0_f32]),
            bullet_collision_groups,
            bullet_texture,
            bullet_collider: BulletCollider(Vec2::new(8., 8.)),

            // Physics
            velocity: Velocity::default(),
            rotation_degrees: RotationDegrees(180.),
            transform,
            global_transform: GlobalTransform::default(),

            // Visibility
            current_animation_frame: CurrentAnimationFrame(1),
            character_texture_handles: texture_handles,
            texture: current_texture,
            sprite: Sprite::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
    fn set_translation_relative_to_owner(
        owner_translation: Vec3,
        owner_rotation: f32,
    ) -> Vec3 {
        let translation: Vec3;
        if owner_rotation > 0. && owner_rotation < 180. {
            translation = Vec3::new(
                owner_translation.x - 14_f32,
                owner_translation.y - 2_f32,
                owner_translation.z - 1_f32,
            )
        } else if owner_rotation > 180. {
            translation = Vec3::new(
                owner_translation.x - 3_f32,
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
                owner_translation.y - 2_f32,
                owner_translation.z - 1_f32,
            )
        };
        translation
    }

    fn generate_bullet_transform(
        bullet_rotation: Quat,
        weapon_transform: &Transform,
        bullet_direction: Vec2,
        bullet_length: f32,
    ) -> Transform {

        let translation_x = Self::get_bullet_start_axis(
            weapon_transform.translation.x,
            bullet_direction.x,
            SCALING.x,
            bullet_length,
        );
        let translation_y = Self::get_bullet_start_axis(
            weapon_transform.translation.y,
            bullet_direction.y,
            SCALING.y,
            bullet_length,
        );
        Transform {
            translation: Vec3::new(translation_x, translation_y + BULLET_TO_WEAPON_OFFSET_Y, weapon_transform.translation.z - 0.1),
            rotation: bullet_rotation,
            scale: SCALING,
        }
    }

    fn get_bullet_start_axis(
        shooter_axis: f32,
        shooter_direction: f32,
        scaling: f32,
        bullet_length: f32,
    ) -> f32 {
        shooter_axis + (bullet_length * scaling) * shooter_direction
    }
}