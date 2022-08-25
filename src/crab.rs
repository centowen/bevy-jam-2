use crate::crab;
use bevy::prelude::*;
use bevy_turborand::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Crab;

// KNARK: Probably shouldn't go here :^)
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub fn move_crabs(
    mut q_crabs: Query<(&mut Transform, &mut crab::Velocity, &mut Sprite), With<Crab>>,
    mut rng: ResMut<GlobalRng>,
) {
    for (mut transform, mut velocity, mut sprite) in q_crabs.iter_mut() {
        let mut v_norm = (velocity.0.x.powf(2.0) + velocity.0.y.powf(2.0)).powf(0.5);
        if v_norm < 0.0001 {
            velocity.0 = Vec2::new(
                -transform.translation.x + 0.5,
                -transform.translation.y + 0.5,
            ); // Vec2::new(rng.f32() * 2.0 - 1.0, rng.f32() * 2.0 - 1.0);
            v_norm = (velocity.0.x.powf(2.0) + velocity.0.y.powf(2.0)).powf(0.5);
            velocity.0 = velocity.0 / v_norm;
        }
        let nv = velocity.0 / v_norm;
        let perp_nv = Vec2::new(nv.y, -nv.x);
        velocity.0 += (rng.f32() * 0.2 - 0.1) * perp_nv;
        velocity.0 += (rng.f32() * 0.1 - 0.05) * nv;
        transform.translation += velocity.0.extend(0.0);
        let angle = (PI / 2.0) - f32::atan2(velocity.0.x, velocity.0.y);
        transform.rotation = Quat::from_rotation_z(angle);
        sprite.flip_y = angle > PI / 2.0;
    }
}

pub fn despawn_crabs(mut commands: Commands, q_crabs: Query<(Entity, &Transform), With<Crab>>) {
    for (entity, transform) in q_crabs.iter() {
        if transform.translation.length() > 600.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
