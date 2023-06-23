use std::thread::spawn;
use std::vec;

use bevy::prelude::*;

use crate::constants::*;
use crate::components::*;
use crate::events::*;

pub fn setup_world(mut commands: Commands, mut font_res: ResMut<FontResource>, asset_server: Res<AssetServer>) {
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

    let font: Handle<Font> = asset_server.load("fonts/FallingSkyBlack.otf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    font_res.font = text_style.clone();
}

pub fn spawn_mothership(mut commands: Commands, fonts: Res<FontResource>) {

    let text_style = fonts.font.clone();

    let bottom_left = Vec3::new(-(MOTHERSHIP_STRUCTURE_SPACING * 5.5), -(MOTHERSHIP_STRUCTURE_SPACING * 2.5), 0.);

    let width = 11;
    let height = 5;

    let chars = vec!["}", "{", "6", "=", "-", "/", ":", "]", "[", "!", "#", "%", "$"];

    let base_poses = vec![Vec3::new(-400., 0., 0.), Vec3::new(400., 0., 0.)];

    for i in 0..2 {
        let ship_pos = base_poses[i];
        println!("Spawning ship at {:?}", ship_pos);
        commands.spawn((SpriteBundle{ transform: Transform::from_translation(ship_pos), ..default() }, Mothership::default()))
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

pub fn simple_input(mut spawn_events: EventWriter<SpawnTorpedoEvent>, keys: Res<Input<KeyCode>>) {

    let spawn_key = KeyCode::Space;

    if keys.just_pressed(spawn_key) {
        spawn_events.send(SpawnTorpedoEvent { position: Vec3::ZERO, velocity: Vec3::ZERO });
        println!("Spawn key pressed");
    }

}

pub fn spawn_torpedos(mut spawn_events: EventReader<SpawnTorpedoEvent>, mut commands: Commands, fonts: Res<FontResource>) {

    let text_style = fonts.font.clone();

    for spawn in spawn_events.iter() {
        commands.spawn((Text2dBundle{ 
            text: Text {
                sections: vec![TextSection::new("!", text_style.clone())],
                ..default()
            },
            transform: Transform::from_translation(spawn.position),
            ..default()
        }, Torpedo{ damage: 3, detonate_radius: 0.5, explosion_radius: 2.5 }));
    }
}

pub fn process_torpedo_collision(mut torpedo_events: EventWriter<TorpedoCollisionEvent>, torpedo_query: Query<(Entity, &Transform, &Torpedo)>, structure_query: Query<(Entity, &Transform, &Structure), Without<Torpedo>>) {
    
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

