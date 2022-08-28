use crate::assets;
use bevy::prelude::*;
use std::f32::consts::PI;

const PLANE_SPEED: f32 = 50.0;
const INITIAL_PLANE_ALTITUDE: f32 = 100.0;
const PLANE_DESCENT_SPEED: f32 = 5.0;
const SUN_ANGLE: f32 = PI/2.0/3.0;

#[derive(Component)]
pub struct Plane;

#[derive(Component)]
pub struct Altitude(pub f32);

#[derive(Component)]
pub struct PlaneShadow;

pub fn spawn_plane(commands: &mut Commands, image_assets: &assets::ImageAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.plane.clone(),
            transform: Transform {
                translation: Vec3::new(1000.0, 0.0, 100.0),
                ..default()
            },
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        })
        .insert(Altitude(INITIAL_PLANE_ALTITUDE))
        .insert(Plane)
        .insert(Name::new("Plane"))
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

pub fn move_plane_shadow(
    mut q_plane_shadow: Query<&mut Transform, With<PlaneShadow>>,
    q_plane: Query<&Altitude, With<Plane>>,
) {
    let altitude = q_plane.single().0;
    let mut transform = q_plane_shadow.single_mut();
    transform.translation.x = -altitude*SUN_ANGLE.sin();
    transform.translation.y = -altitude*SUN_ANGLE.cos();
}
