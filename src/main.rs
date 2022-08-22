use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_turborand::*;
use std::{f32::consts::PI, time::Duration};

struct CrabSpawnTimer {
    timer: Timer,
}

struct ImageAssets {
    crab: Handle<Image>,
}

#[derive(Component)]
struct Crab;

#[derive(Component, Default)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(CrabSpawnTimer {
        timer: Timer::new(Duration::from_millis(500), true),
    });
    commands.insert_resource(ImageAssets {
        crab: server.load("rustacean-flat-noshadow.png"),
    });
}

fn spawn_crab(
    mut commands: Commands,
    images: Res<ImageAssets>,
    time: Res<Time>,
    mut crab_timer: ResMut<CrabSpawnTimer>,
) {
    crab_timer.timer.tick(time.delta());

    if crab_timer.timer.just_finished() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: images.crab.clone(),
                transform: Transform {
                    scale: Vec3::splat(0.02),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::default())
            .insert(Crab);
    }
}

fn move_crabs(
    mut q_crabs: Query<(&mut Transform, &mut Velocity, &mut Sprite), With<Crab>>,
    mut rng: ResMut<GlobalRng>,
) {
    for (mut transform, mut velocity, mut sprite) in q_crabs.iter_mut() {
        velocity.0 += Vec2::new(rng.f32() * 2.0 - 1.0, rng.f32() * 2.0 - 1.0);
        velocity.0 *= 0.8;
        transform.translation += velocity.0.extend(0.0);
        let angle = (PI / 2.0) - f32::atan2(velocity.0.x, velocity.0.y);
        transform.rotation = Quat::from_rotation_z(angle);
        sprite.flip_y = angle > PI / 2.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RngPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .add_system(spawn_crab)
        .add_system(move_crabs)
        .run();
}
