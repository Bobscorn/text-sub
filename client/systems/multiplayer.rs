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
    mut next_state: ResMut<NextState<GameState>>,
    sub: Res<Submarine>
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

    info!("Going to synchronise ships state");
    // Move channel out of socket to synchronise ships then give to ggrs
    let mut channel = socket.take_channel(0).unwrap();

    // Send our ship to all other peers
    for (_, player) in players.clone().into_iter().enumerate() {
        let PlayerType::<PeerId>::Remote(peer_id) = player else {
            continue;
        };
        channel.send(serde_json::to_vec(&SyncShipsMessageType::SyncShip(sub.clone())).unwrap().into(), peer_id);
    }

    // Send it off to the Ship Syncing state (in sync_ships.rs)
    commands.insert_resource(SyncSubsSocket{ players, channel: Some(channel), synced: false, ready: false });
    next_state.set(GameState::SubSyncing);
}

pub fn player_action(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&mut Velocity, &Player, &mut Transform, &mut AngularVelocity), With<SubsRoot>>
) {
    // Basic demonstrational movement for now
    for (mut velocity, player, mut transform, mut angular) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];

        let direction = direction(input);

        if direction != Vec2::ZERO { //player did not move
            let move_delta = direction * SUB_SPEED;
            velocity.value += move_delta;
        }

        let angular_direction = get_rotation(input);
        
        if angular_direction != 0.0_f32 {
            angular.rotation += angular_direction;            
        }   
        transform.rotate_local_z(angular.rotation * PI / 180.0);     
    }
}

pub fn spawn_mothersub(
    mut commands: Commands, 
    fonts: Res<FontResource>
) {
    let pos = SUB_STRUCTURE_SPACING * 10.5;

    let bottom_left = Vec3::new(-(SUB_STRUCTURE_SPACING * 5.5), -(SUB_STRUCTURE_SPACING * 2.5), 0.);


    let width = 11;
    let height = 5;

    let chars = vec!["}", "{", "6", "=", "-", "/", ":", "]", "[", "!", "#", "%", "$"];

    let base_poses = vec![Vec3::new(-pos, 0., 0.), Vec3::new(pos, 0., 0.)];

    for i in 0..2 {
        let sub_pos = base_poses[i];
        println!("Spawning sub at {:?}", sub_pos);
        commands.spawn((
            SpatialBundle{ transform: Transform::from_translation(sub_pos), ..default() }, 
            SubsRoot::default(), 
            Velocity{ value: Vec2::ZERO },
            AngularVelocity::default(),
            Player{ handle: i },
            BulletReady(true)
        )).add_rollback()
            .with_children(|parent| {
                for x in 0..width {
                    for y in 0..height {
                        parent.spawn((
                            Text2dBundle{ 
                                text: Text { 
                                    sections: vec![TextSection::new(chars[(x + y) % 13], fonts.p1_font.clone())],
                                    ..default()
                                },
                                transform: Transform::from_scale(Vec3::ONE * SUB_SCALE).with_translation(bottom_left + Vec3::new(x as f32 * SUB_STRUCTURE_SPACING, y as f32 * SUB_STRUCTURE_SPACING, 0.)),
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
