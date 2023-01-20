use bevy::prelude::*;

use crate::components::bullet::Bullet;

pub fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}
