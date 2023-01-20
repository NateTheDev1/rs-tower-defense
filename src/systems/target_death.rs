use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query};

use crate::components::health::Health;

pub fn target_death(mut commands: Commands, mut targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}
