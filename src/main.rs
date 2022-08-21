use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::time::Duration;

struct CrabSpawnTimer {
    timer: Timer,
}

struct ImageAssets {
    crab: Handle<Image>,
}

#[derive(Component)]
struct Crab;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(CrabSpawnTimer {
        timer: Timer::new(Duration::from_secs(3), true),
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
                    scale: Vec3::splat(0.2),
                    ..default()
                },
                ..default()
            })
            .insert(Crab);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .add_system(spawn_crab)
        .run();
}
