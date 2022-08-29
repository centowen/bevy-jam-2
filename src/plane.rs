use crate::{assets, collision, crab, spawner};
use bevy::prelude::*;
use bevy_turborand::*;
use std::f32::consts::PI;
use std::time::Duration;

const PLANE_SPEED: f32 = 50.0;
const INITIAL_PLANE_ALTITUDE: f32 = 100.0;
const PLANE_DESCENT_SPEED: f32 = 5.0;
const SUN_ANGLE: f32 = PI / 2.0 / 3.0;

#[derive(Component)]
pub struct Plane;

#[derive(Component)]
pub struct Altitude(pub f32);

#[derive(Component)]
pub struct PlaneShadow;

#[derive(Component)]
pub struct SmokeSource {
    mean_time: Duration,
    timer: Timer,
    rng: RngComponent,
}

#[derive(Component)]
pub struct Smoke {
    life_time: Timer,
}

impl SmokeSource {
    fn new(mean_time: Duration, global_rng: &mut GlobalRng) -> Self {
        SmokeSource {
            mean_time,
            timer: Timer::new(mean_time, false),
            rng: RngComponent::from(global_rng),
        }
    }

    fn reset(&mut self) {
        let duration = self.mean_time.mul_f32(self.rng.f32() + self.rng.f32());
        self.timer.set_duration(duration);
        self.timer.reset();
    }
}

pub fn spawn_plane(
    commands: &mut Commands,
    image_assets: &assets::ImageAssets,
    global_rng: &mut GlobalRng,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.plane.clone(),
            transform: Transform {
                translation: Vec3::new(1000.0, 0.0, 100.0),
                ..default()
            },
            sprite: Sprite {
                flip_x: true,
                custom_size: Some(Vec2::new(100.0, 90.0)),
                ..default()
            },
            ..default()
        })
        .insert(Altitude(INITIAL_PLANE_ALTITUDE))
        .insert(Plane)
        .insert(Name::new("Plane"))
        .insert(collision::Collisions::new())
        .insert(SmokeSource::new(Duration::from_millis(400), global_rng))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: image_assets.plane_shadow.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, -1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        flip_x: true,
                        custom_size: Some(Vec2::new(100.0, 90.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(PlaneShadow)
                .insert(Name::new("Shadow"));
        });
}

pub fn move_plane(
    mut q_plane: Query<(&mut Transform, &mut Altitude), With<Plane>>,
    time: Res<Time>,
) {
    let (mut transform, mut altitude) = q_plane.single_mut();

    altitude.0 -= PLANE_DESCENT_SPEED * time.delta_seconds();
    if altitude.0 < 0.0 {
        altitude.0 = 0.0;
    } else {
        transform.translation.x -= PLANE_SPEED * time.delta_seconds();
    }
}

pub fn collide_with_world(
    q_crabs: Query<(&crab::Crab, &Transform)>,
    q_plane: Query<&collision::Collisions, With<Plane>>,
    mut commands: Commands,
    images: Res<assets::ImageAssets>,
) {
    let collisions = q_plane.single();
    for collision in collisions.collisions.iter() {
        let r_crab = q_crabs.get(collision.entity);
        if let Ok((_crab, transform)) = r_crab {
            // commands.entity(*entity).remove::<crab::Crab>().insert(crab::DeadCrab);
            commands.entity(collision.entity).despawn_recursive();
            spawner::spawn_dead_crab(transform.translation, &mut commands, &images);
        }
    }
}

pub fn move_plane_shadow(
    mut q_plane_shadow: Query<&mut Transform, With<PlaneShadow>>,
    q_plane: Query<&Altitude, With<Plane>>,
) {
    let altitude = q_plane.single().0;
    let mut transform = q_plane_shadow.single_mut();
    transform.translation.x = -altitude * SUN_ANGLE.sin();
    transform.translation.y = -altitude * SUN_ANGLE.cos();
}

pub fn spawn_smoke(
    mut commands: Commands,
    mut q_plane: Query<(&Transform, &mut SmokeSource, &Altitude), With<Plane>>,
    image_assets: Res<assets::ImageAssets>,
    time: Res<Time>,
) {
    for (transform, mut smoke_source, altitude) in q_plane.iter_mut() {
        if altitude.0 <= 0.0 {
            continue;
        }
        smoke_source.timer.tick(time.delta());
        if smoke_source.timer.just_finished() {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: image_assets.smoke.clone(),
                    transform: Transform {
                        translation: transform.translation + 75.0 * Vec3::X,
                        ..default()
                    },
                    ..default()
                })
                .insert(Smoke {
                    life_time: Timer::new(Duration::from_secs(3), false),
                });
            smoke_source.reset();
        }
    }
}

pub fn move_smoke(
    mut commands: Commands,
    mut q_smoke: Query<(Entity, &mut Transform, &mut Smoke)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut smoke) in q_smoke.iter_mut() {
        smoke.life_time.tick(time.delta());
        if smoke.life_time.finished() {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        transform.translation.y += 20.0 * time.delta_seconds();
    }
}
