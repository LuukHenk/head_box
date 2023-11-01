use bevy::prelude::*;


#[derive(Component)]
pub struct ActiveGunMarker;
pub fn spawn_default_gun(mut commands: Commands) {
    commands.spawn((Pistol::new(), GameScreenMarker, ActiveGunMarker));
}
pub fn shoot_active_gun(
    shoot_event: EventReader<ShootEvent>,
    mut active_gun_query: Query<(&CoolDownTime, &BulletsPerShot, &mut Bullet), With<ActiveGunMarker>>,
) {
    let (cooldown_time, bullets_per_shot, mut bullet) = active_gun_query.single_mut();
    if cooldown_time.0.finished() {
        for _i in 0..bullets_per_shot.0 {
            println!("shoot!")
            // bullet.spawn(x, y, commands);
        }
    }
}

// pub fn handle_cooldowns(ac)