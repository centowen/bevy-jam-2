use crate::assets;
use crate::audio;
use crate::collision;
use crate::crab;

use bevy::prelude::*;

const SPAWNER_THRESHOLD_ENERGY: i32 = 15;

#[derive(Component)]
pub struct Spawner {
    pub energy: i32,
}

pub fn spawn_tick(
    mut q_spawners: Query<(&mut Spawner, &Transform)>,
    mut commands: Commands,
    images: Res<assets::ImageAssets>,
) {
    for (mut spawner, &transform) in q_spawners.iter_mut() {
        if spawner.energy >= SPAWNER_THRESHOLD_ENERGY {
            spawn_crab(transform.translation, &mut commands, &images);
            spawner.energy = 0;
        }
        spawner.energy += 1;
    }
}

fn spawn_crab(translation: Vec3, commands: &mut Commands, images: &assets::ImageAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 10.0)),
                ..default()
            },
            texture: images.crab.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .insert(crab::Velocity::default())
        .insert(crab::Crab)
        .insert(audio::AudioEvent { played: false })
        .insert(Name::new("Crab"))
        .insert(collision::Collisions::new());
}
