use bevy::prelude::Bundle;
use crate::in_game::bullet::bullet_bundle::BulletBundle;
use crate::in_game::data_classes::weapon_components::{CoolDownTime, WeaponMarker};

#[derive(Bundle)]
pub struct WeaponBundle {
    cooldown_time: CoolDownTime,
    weapon_marker: WeaponMarker,
    bullet: BulletBundle,
}