use bevy::prelude::*;
use crate::implementations::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::input::*;
use bevy_ggrs::*;

pub fn reload_torpedo(inputs: Res<PlayerInputs<GgrsConfig>>, mut query: Query<(&mut BulletReady, &Player)>) {
    for (mut can_fire, player) in query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if !fire(input) {
            can_fire.0 = true;
        }
    }
}

pub fn fire_torpedo(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    images: Res<ImageAssets>,
    mut player_query: Query<(&Transform, &Player, &mut BulletReady)>
) {
    for (transform, player, mut bullet_ready) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if fire(input) && bullet_ready.0 {
            commands.spawn((
                SpriteBundle {
                transform: Transform::from_translation(transform.translation),
                texture: images.bullet.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5., 5.)),
                    ..default()
                },
                ..default()
            }, 
            Torpedo::default(),
            Velocity {
                value: Vec2::new(20., 0.)
            }, Acceleration(
                Vec2::new(5., 0.)
            )
        )).add_rollback();

            bullet_ready.0 = false;
        }
    }
}

pub fn spawn_torpedos(mut spawn_events: EventReader<SpawnTorpedoEvent>, mut commands: Commands, fonts: Res<FontResource>) {

    let text_style = fonts.p1_font.clone();

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

pub fn process_torpedo_collision(
    mut torpedo_events: EventWriter<TorpedoCollisionEvent>, 
    torpedo_query: Query<(Entity, &GlobalTransform, &Torpedo)>, 
    structure_query: Query<(&GlobalTransform, &Structure), Without<Torpedo>>, 
    mut commands: Commands
) {
    for (t_ent, torpedo_trans, torpedo) in &torpedo_query {
        let t_pos = torpedo_trans.translation();

        for (struc_trans, _struc) in &structure_query {
            let s_pos = struc_trans.translation();

            let d_sq = (s_pos - t_pos).length_squared();
            if d_sq < (torpedo.detonate_radius * torpedo.detonate_radius) {
                info!("Sending torpedo collision event!");
                torpedo_events.send(TorpedoCollisionEvent { position: t_pos, damage: torpedo.damage, radius: torpedo.explosion_radius });
                commands.get_entity(t_ent).map(|x| { x.despawn_recursive(); Some(()) });
                break; // break to not send the event more than once
            }
        }
    }
}

pub fn do_torpedo_events(
    sprites: Res<ImageAssets>,
    mut commands: Commands, 
    mut t_events: EventReader<TorpedoCollisionEvent>, 
    mut struc_query: Query<(Entity, &GlobalTransform, &mut Structure), Without<Torpedo>>) {
    for event in t_events.iter() {
        let pos = event.position;
        let dmg = event.damage;
        let radius_sq = event.radius * event.radius;

        info!("Torpedo Collision Event at {:?}, with dmg {} and radius {}!", pos, dmg, event.radius);

        commands.spawn((
            SpriteBundle{
                sprite: Sprite{
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..default()
                },
                texture: sprites.explosion.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            }, Lifetime{ lifetime: 3. }
        ));

        for (ent, trans, mut struc) in &mut struc_query {
            let dif = pos - trans.translation();
            
            if dif.length_squared() >= radius_sq {
                continue;
            }

            let new_health = struc.integrity as i32 - dmg as i32;
            
            info!("Damaging Structure: new: {}, old: {}, dmg: {}", new_health, struc.integrity, dmg);
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
