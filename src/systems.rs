use bevy::prelude::*;
use bevy::ecs::component::Components;

use crate::constants::*;
use crate::components::*;

pub fn print_position_system(query: Query<&Transform>) {
    for transform in &query {
        println!("position: {:?}", transform.translation);
    }
}

pub fn move_projectile(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_seconds();
    
    for (mut transform, vel) in &mut query {
        let movement_2d = vel.value * dt;
        transform.translation += Vec3::new(movement_2d.x, movement_2d.y, 0.0f32);
    }
}

pub fn move_mothership(time: Res<Time>, mut query: Query<&mut Transform>) {
    let dt = time.delta_seconds();

    for mut transform in &mut query {
        let origin  = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(MOTHERSHIP_SPEED * dt);
        transform.rotate_around(origin, rotation);
    }
}

pub fn process_torpedo_collision(world: &World) {
    let entity_query = world.query::<&Components>().unwrap();

    for entity in entity_query.iter_entities(world)
}
