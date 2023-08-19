use bevy::ecs::prelude::*;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn move_mothership(time: Res<Time>, mut query: Query<&mut Transform, With<Mothership>>) {
    let dt = time.delta_seconds();

    for mut transform in &mut query {
        let origin  = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(MOTHERSHIP_SPEED * dt);
        transform.rotate_around(origin, rotation);
    }
}

pub fn move_projectile(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_seconds();
    
    for (mut transform, velocity) in &mut query {
        let movement_2d = velocity.value * dt;
        transform.translation += Vec3::new(movement_2d.x, movement_2d.y, 0.0f32);
    }
}

pub fn accelerate_projectile(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    let dt = time.delta_seconds();

    for (mut vel, acc) in &mut query {
        let incr = acc.0 * dt;
        vel.value = vel.value + incr;
    }
}
