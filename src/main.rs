use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .run();
}
