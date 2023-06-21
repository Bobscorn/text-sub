use std::vec;

use bevy::prelude::*;

use crate::constants::*;
use crate::components::*;
use crate::events::TorpedoCollisionEvent;

pub fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle{ 
        sprite: Sprite {
            color: Color::rgb(0.25, 0.75, 0.25),
            custom_size: Some(Vec2::new(50., 100.)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}

pub fn spawn_mothership(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = asset_server.load("fonts/FallingSkyBlack.otf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    let bottom_left = Vec3::new(-(MOTHERSHIP_STRUCTURE_SPACING * 5.5), -(MOTHERSHIP_STRUCTURE_SPACING * 2.5), 0.);
    let mothership_pos = Vec3::new(0., 0., 0.);

    let width = 11;
    let height = 5;

    let chars = vec!["}", "{", "6", "=", "-", "/", ":", "]", "[", "!", "#", "%", "$"];

    commands.spawn((SpriteBundle{ transform: Transform::from_translation(mothership_pos), ..default() }, Mothership::default()))
        .with_children(|parent| {
            for x in 0..width {
                for y in 0..height {
                    parent.spawn((
                        Text2dBundle{ 
                            text: Text { 
                                sections: vec![TextSection::new(chars[(x + y) % 13], text_style.clone())],
                                ..default()
                            },
                            transform: Transform::from_translation(bottom_left + Vec3::new(x as f32 * MOTHERSHIP_STRUCTURE_SPACING, y as f32 * MOTHERSHIP_STRUCTURE_SPACING, 0.)),
                            ..default()
                        }, 
                        Structure{ integrity: 5, max_integrity: 5 }
                    ));
                }
            }
        }
    );
}

pub fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = asset_server.load("fonts/FallingSkyBlack.otf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    let count = 10;
    let start_pos = Vec3::new(-150., -50., 0.);
    let end_pos = start_pos + Vec3::new(MOTHERSHIP_STRUCTURE_SPACING, 0., 0.) * count as f32;

    let chars = vec!["}", "{", "6", "=", "-", "/", ":", "]", "[", "!", "#", "%", "$"];
    for i in 0..count {
        let pos = start_pos.lerp(end_pos, i as f32 / count as f32);

        commands.spawn(Text2dBundle{
            text: Text {
                sections: vec![TextSection::new(chars[i], text_style.clone())],
                ..default()
            },
            transform: Transform::from_translation(pos),
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                custom_size: Some(Vec2::new(5., 5.)),
                ..default()
            },
            transform: Transform::from_translation(pos),
            ..default()
        });
    }
}

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

pub fn move_mothership(time: Res<Time>, mut query: Query<&mut Transform, With<Mothership>>) {
    let dt = time.delta_seconds();

    for mut transform in &mut query {
        let origin  = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(MOTHERSHIP_SPEED * dt);
        transform.rotate_around(origin, rotation);
    }
}

pub fn process_torpedo_collision(mut torpedo_events: EventWriter<TorpedoCollisionEvent>, mut commands: Commands, torpedo_query: Query<(Entity, &Transform, &Torpedo)>, structure_query: Query<(Entity, &Transform, &Structure), Without<Torpedo>>) {
    
    for (t_ent, torpedo_trans, torpedo) in &torpedo_query {
        let t_pos = torpedo_trans.translation;

        for (s_ent, struc_trans, struc) in &structure_query {
            let s_pos = struc_trans.translation;

            let d_sq = (s_pos - t_pos).length_squared();
            if d_sq < (torpedo.detonate_radius * torpedo.detonate_radius) {
                torpedo_events.send(TorpedoCollisionEvent { position: t_pos, torpedo: t_ent, damage: torpedo.damage, radius: torpedo.explosion_radius });
                break; // break to not send the event more than once
            }
        }
    }
}

pub fn do_torpedo_events(mut commands: Commands, mut t_events: EventReader<TorpedoCollisionEvent>, mut struc_query: Query<(Entity, &Transform, &mut Structure), Without<Torpedo>>) {
    for event in t_events.iter() {
        let pos = event.position;
        let dmg = event.damage;
        let radius_sq = event.radius * event.radius;

        for (ent, trans, mut struc) in &mut struc_query {
            let dif = pos - trans.translation;
            
            if dif.length_squared() >= radius_sq {
                continue;
            }

            let new_health = struc.integrity as i32 - dmg as i32;
            if new_health <= 0 {
                struc.integrity = 0;
                if let Some(mut coms) = commands.get_entity(ent) {
                    coms.despawn();
                }
            }
            else {
                struc.integrity = new_health as u8;
            }
        }
    }
}

