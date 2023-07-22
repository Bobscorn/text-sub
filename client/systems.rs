use std::vec;

use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_ggrs::*;
use bevy_ggrs::ggrs::*;

use crate::constants::*;
use crate::components::*;
use crate::events::*;
use crate::game_state::GameState;
use crate::resources::*;
use crate::input::*;


pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    // A single byte should fit a bunch of inputs
    type Input = u8;
    type State = u8;
    
    type Address = PeerId;
}

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/extreme_bevy?next=2";
    info!("connecting to matchbox server: {:?}", room_url);

    let socket: MatchboxSocket<SingleChannel> = WebRtcSocketBuilder::new(room_url)
        .add_channel(ChannelConfig::reliable())
        .into();

    commands.insert_resource(socket);
}

pub fn wait_for_players(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>
) {
    if socket.get_channel(0).is_err() {
        return; // We've already started
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // Not enough players yet
    }

    info!("All peers have joined, going in game!");
    crate::log!("All peers have joined, going in game!");

    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(2)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        if player == PlayerType::Local {
            commands.insert_resource(LocalPlayerHandle(i));
        }
        session_builder = session_builder
            .add_player(player, i)
            .expect("Failed to add player");
    }

    // Move channel out of socket to give to ggrs
    let channel = socket.take_channel(0).unwrap();

    // Start the GGRS Session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("Failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));

    next_state.set(GameState::InGame);
}

pub fn player_action(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&mut Transform, &Player), With<Mothership>>
) {
    // Basic demonstrational movement for now
    for (mut transform, player) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];

        let direction = crate::input::direction(input);

        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 0.13;
        let move_delta = (direction * move_speed).extend(0.);
        transform.translation += move_delta;
    }
}

pub fn fire_torpedo(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    images: Res<ImageAssets>,
    player_query: Query<(&Transform, &Player)>
) {
    for (transform, player) in player_query.iter() {
        let (input, _) = inputs[player.handle];
        if fired(input) {
            commands.spawn((
                SpriteBundle {
                transform: Transform::from_translation(transform.translation),
                texture: images.bullet.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.3, 0.1)),
                    ..default()
                },
                ..default()
            }, Torpedo {
                damage: 1u8,
                detonate_radius: 1.,
                explosion_radius: 1.
            }, Velocity {
                value: Vec2::new(1., 0.)
            }));
        }
    }
}

pub fn setup_world(mut commands: Commands, mut font_res: ResMut<FontResource>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font: Handle<Font> = asset_server.load("fonts/FallingSkyBlack.otf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    font_res.font = text_style.clone();
    font_res.p1_font = TextStyle{ font: font.clone(), font_size: 60., color: Color::BLUE };
    font_res.p2_font = TextStyle{ font: font.clone(), font_size: 60., color: Color::ORANGE };
}

pub fn spawn_mothership(mut commands: Commands, fonts: Res<FontResource>, mut rip: ResMut<RollbackIdProvider>) {

    let text_style = fonts.font.clone();

    let bottom_left = Vec3::new(-(MOTHERSHIP_STRUCTURE_SPACING * 5.5), -(MOTHERSHIP_STRUCTURE_SPACING * 2.5), 0.);

    let width = 11;
    let height = 5;

    let chars = vec!["}", "{", "6", "=", "-", "/", ":", "]", "[", "!", "#", "%", "$"];

    let base_poses = vec![Vec3::new(-400., 0., 0.), Vec3::new(400., 0., 0.)];
    let text_styles = vec![fonts.p1_font.clone(), fonts.p2_font.clone()];

    for i in 0..2 {
        let ship_pos = base_poses[i];
        println!("Spawning ship at {:?}", ship_pos);
        commands.spawn((
            SpriteBundle{ transform: Transform::from_translation(ship_pos), ..default() }, 
            Mothership::default(), 
            Player{ handle: i },
            rip.next()
        ))
            .with_children(|parent| {
                for x in 0..width {
                    for y in 0..height {
                        parent.spawn((
                            Text2dBundle{ 
                                text: Text { 
                                    sections: vec![TextSection::new(chars[(x + y) % 13], text_styles[i].clone())],
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

