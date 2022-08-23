use crate::assets;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(commands: &mut Commands, image_assets: &assets::ImageAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.player.clone(),
            transform: Transform::from_scale(Vec3::splat(0.2)),
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = q_player.single_mut();
    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
    };
}
