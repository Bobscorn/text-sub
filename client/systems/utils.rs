use bevy::prelude::*;
use crate::components::*;

pub fn do_lifetime(
    time: Res<Time>,
    mut commands: Commands, 
    mut query: Query<(Entity, &mut Lifetime)>
) {
    let dt = time.delta_seconds();

    for (ent, mut life) in &mut query {
        life.lifetime -= dt;
        if life.lifetime <= 0. {
            commands.get_entity(ent).map(|x| { x.despawn_recursive(); Some(()) });
        }
    }
}

pub fn print_position_system(query: Query<&Transform>) {
    for transform in &query {
        println!("position: {:?}", transform.translation);
    }
}
