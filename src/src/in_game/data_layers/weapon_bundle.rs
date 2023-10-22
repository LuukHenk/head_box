use bevy::prelude::Bundle;

use crate::in_game::data_classes::weapon_components::{CoolDownTime, WeaponMarker};

use super::bullet_bundle::BulletBundle;
#[derive(Bundle)]
pub struct WeaponBundle {
    cooldown_time: CoolDownTime,
    weapon_marker: WeaponMarker,
    bullet: BulletBundle,
}