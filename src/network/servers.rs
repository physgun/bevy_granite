//! Module level docs for the local server architecture

use core::net::{Ipv4Addr, SocketAddr};
use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet2::{
    netcode::{NativeSocket, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig}, 
    renet2::{ConnectionConfig, RenetServer}, RenetChannelsExt};

/// Plugin defining server entities and their interactions with network libraries.
pub struct LocalServerPlugin;
impl Plugin for LocalServerPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(Startup, Self::setup_local_replicon_renet_server);
    }
}
impl LocalServerPlugin {
    /// What port the server will listen on by default.
    const DEFAULT_PORT: u16 = 7142;
    /// Default protocol ID for the server.
    const PROTOCOL_ID: u64 = 0;

    /// On entering the Host state, set up the local replicon server with renet2 transport.
    fn setup_local_replicon_renet_server (
        mut commands: Commands,
        replicon_renet_channels: Res<RepliconChannels>
    ) {
        let server_channels_config = replicon_renet_channels.get_server_configs();
        let client_channels_config = replicon_renet_channels.get_client_configs();

        let replicon_renet_server = RenetServer::new(
            ConnectionConfig::from_channels( 
                server_channels_config,
                client_channels_config
            )
        );

        let Ok(current_time) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) else {
            error!("Could not get SystemTime?? Server creation aborted!");
            return;
        };

        let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), Self::DEFAULT_PORT);

        let Ok(socket) = UdpSocket::bind(public_addr) else {
            error!("Socket creation failed!");
            return;
        };
        let Ok(native_socket) = NativeSocket::new(socket) else {
            error!("NativeSocket creation failed!");
            return;
        };

        let server_config = ServerSetupConfig {
            current_time,
            max_clients: 256,
            protocol_id: Self::PROTOCOL_ID,
            authentication: ServerAuthentication::Unsecure,
            socket_addresses: vec![vec![public_addr]]
        };

        let Ok(transport) = NetcodeServerTransport::new(
            server_config, 
            native_socket
        ) else {
            error!("Transport protocol failed!");
            return;
        };

        commands.insert_resource(replicon_renet_server);
        commands.insert_resource(transport);
    }
}