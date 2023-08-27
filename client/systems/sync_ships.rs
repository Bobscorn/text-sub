use bevy::prelude::*;
use bevy_ggrs::ggrs::{self, PlayerType};

use crate::{resources::*, enums::*, implementations::GgrsConfig};

/// Synchronises each peer's sub with each other
/// Ensures before entering ggrs that both peers have each other's ship data
/// For now it only expects 1 peer (ie. 2 people, a 1v1)
/// 
/// This is done using the WebRtcSocket (actually just the streams) taken from the MatchboxSocket,
/// This is intended to not interfere with matchbox messages or ggrs messages by taking: 1. after matchbox has connected the 
/// peers, and 2. before ggrs even starts sending messages.
/// Normally this type of communication between peers would have been enabled by the networking library (ggrs), but in this
/// case ggrs *only* transmits the Input of each player across the wire and nothing else, and does not provide a method
/// of transmitting any other meaningful information
/// 
/// It could later be expanded if the EnemySubmarine type became a component instead of a resource,
/// This would require determining which peer each ship belongs to and setting the appropriate field when spawning their ship
/// It would also require counting the number of ships/acks/ready(s) received and not proceeding before they have all been received
pub fn synchronise_peer_ships(
    mut sync_sock: ResMut<SyncSubsSocket>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>
) {
    let mut channel = sync_sock.channel.take().expect("Synchronising ships requires a socket!");
    let messages = channel.receive();

    let mut done_zo = false;

    for (peer_id, msg) in messages {

        // Deserialize the incoming packet
        let ship_message = match serde_json::from_slice::<SyncShipsMessageType>(&msg) {
            Ok(e) => e,
            Err(_) => continue
        };

        // v Process and Respond to Peer messages

        match ship_message {
            SyncShipsMessageType::SyncShip(enemy_sub) => {
                info!("Received enemy ship");
                for row in &enemy_sub.parts {
                    info!("{row:#?}");
                }
                commands.insert_resource(EnemySubmarine{ parts: enemy_sub.parts, rotations: enemy_sub.rotations });

                channel.send(serde_json::to_vec(&SyncShipsMessageType::SyncShipAck).unwrap().into(), peer_id);
            },

            SyncShipsMessageType::SyncShipAck => { 
                info!("Enemy has received our ship");
                channel.send(serde_json::to_vec(&SyncShipsMessageType::ReadyToStart).unwrap().into(), peer_id); 
            },
            
            SyncShipsMessageType::ReadyToStart => { 
                // To have multiple peers it would be necessary to only send ReadyToStart once we have received ALL peer's ships
                info!("Enemy is ready to start");
                channel.send(serde_json::to_vec(&SyncShipsMessageType::ReadyToStartAck).unwrap().into(), peer_id);                 
            },

            SyncShipsMessageType::ReadyToStartAck => {
                // To have multiple peers it would also be necessary to only start once all peers have acknolwedged
                // eg. num_peers_ready += 1; if num_peers_ready >= num_peers { start; }
                info!("Enemy acknowledges we are ready to start");
                done_zo = true;
                break;
            },
        }
    }

    if !done_zo {
        sync_sock.channel = Some(channel); // Return the channel back
        return; // Don't go into ggrs if not ready
    }

    // ^ Transmit sub data across the wire
    // v Spawn the subs here


    // ^ Sub Spawning
    // v Initializing and moving to GGRS

    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(2)
        .with_input_delay(2);

    for (i, player) in sync_sock.players.clone().into_iter().enumerate() {
        if player == PlayerType::Local {
            commands.insert_resource(LocalPlayerHandle(i));
        }
        session_builder = session_builder
            .add_player(player, i)
            .expect("Failed to add player");
    }

    // Start the GGRS Session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("Failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_state.set(GameState::InGame);
}

