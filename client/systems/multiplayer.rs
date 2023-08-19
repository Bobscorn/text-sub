use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use crate::enums::*;
use bevy_ggrs::*;
use bevy_ggrs::ggrs::*;
use crate::implementations::*;
use crate::resources::*;
use crate::components::*;
use crate::constants::*;
use crate::input::*;
use std::f32::consts::*;
use crate::events::*;
use bevy::window::*;

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
    mut player_query: Query<(&mut Velocity, &Player, &mut Transform, &mut AngularVelocity), With<Mothership>>
) {
    // Basic demonstrational movement for now
    for (mut velocity, player, mut transform, mut angular) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];

        let direction = direction(input);

        if direction != Vec2::ZERO { //player did not move
            let move_delta = direction * MOTHERSHIP_SPEED;
            velocity.value += move_delta;
        }

        let angular_direction = get_rotation(input);
        
        if angular_direction != 0.0_f32 {
            angular.rotation += angular_direction;            
        }   
        transform.rotate_local_z(angular.rotation * PI / 180.0);     
    }
}

pub fn setup_world(mut commands: Commands, mut font_res: ResMut<FontResource>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection { scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(250.), ..default() },        
        ..default()
    });
    commands.insert_resource(ClearColor {0: Color::BLACK});

    let font: Handle<Font> = asset_server.load("fonts/FallingSkyBlack.otf");

    font_res.font = font.clone();
    font_res.p1_font = TextStyle{ font: font.clone(), font_size: TEXT_FONT_SIZE, color: Color::LIME_GREEN };
}

pub fn spawn_mothership(
    mut commands: Commands, 
    fonts: Res<FontResource>, 
    mut rip: ResMut<RollbackIdProvider>,
    wind_q: Query<&Window, With<PrimaryWindow>>
) {
    //let window = wind_q.single();
    //let win_width = window.width();

    //let ship_width = 5.5 * MOTHERSHIP_STRUCTURE_SPACING;
    //let min_gap = 0.1; // 0.1 = 10% of window width
    //let pos = (0.25 * win_width).max(min_gap * view_width + ship_width * 0.5);
    let pos = MOTHERSHIP_STRUCTURE_SPACING * 10.5;

    let bottom_left = Vec3::new(-(MOTHERSHIP_STRUCTURE_SPACING * 5.5), -(MOTHERSHIP_STRUCTURE_SPACING * 2.5), 0.);


    let width = 11;
    let height = 5;

    let chars = vec!["}", "{", "6", "=", "-", "/", ":", "]", "[", "!", "#", "%", "$"];

    let base_poses = vec![Vec3::new(-pos, 0., 0.), Vec3::new(pos, 0., 0.)];

    for i in 0..2 {
        let ship_pos = base_poses[i];
        println!("Spawning ship at {:?}", ship_pos);
        commands.spawn((
            SpriteBundle{ transform: Transform::from_scale(Vec3::ONE * 0.2f32) * Transform::from_translation(ship_pos), ..default() }, 
            Mothership::default(), 
            Velocity{ value: Vec2::ZERO },
            AngularVelocity::default(),
            Player{ handle: i },
            BulletReady(true),
            rip.next()
        ))
            .with_children(|parent| {
                for x in 0..width {
                    for y in 0..height {
                        parent.spawn((
                            Text2dBundle{ 
                                text: Text { 
                                    sections: vec![TextSection::new(chars[(x + y) % 13], fonts.p1_font.clone())],
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
