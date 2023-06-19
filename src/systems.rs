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

pub fn process_torpedo_collision(mut commands: Commands, torpedo_query: Query<(Entity, &Transform, &Torpedo)>, structure_query: Query<(Entity, &Transform, &Structure), Without<Torpedo>>) {
    
    for (t_ent, torpedo_trans, torpedo) in &torpedo_query {

        let t_pos = torpedo_trans.translation;

        for (s_ent, struc_trans, struc) in &structure_query {
            let s_pos = struc_trans.translation;

            let d_sq = (s_pos - t_pos).length_squared();
            if d_sq < (torpedo.detonate_radius * torpedo.detonate_radius) {
                // detonate?
            }
        }
    }
}
