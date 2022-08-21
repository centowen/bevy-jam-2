use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::time::Duration;

struct CrabSpawnTimer {
    timer: Timer,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(CrabSpawnTimer {
        timer: Timer::new(Duration::from_secs(3), true),
    });
}

fn spawn_crab(mut commands: Commands, time: Res<Time>, mut crab_timer: ResMut<CrabSpawnTimer>) {
    crab_timer.timer.tick(time.delta());

    if crab_timer.timer.just_finished() {
        eprintln!("🦀");
        // commands.spawn_bundle(CrabBundle::new())
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
