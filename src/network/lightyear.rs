//! Module level docs for the local server architecture

use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;

use lightyear::prelude::{Mode, SharedConfig};

use lightyear::prelude::server::ServerTransport;
use lightyear::server::config::ServerConfig;

use lightyear::prelude::server::IoConfig as ServerIoConfig;
use lightyear::prelude::server::NetConfig as ServerNetConfig;
use lightyear::server::config::NetcodeConfig as ServerNetcodeConfig;
use lightyear::server::plugin::ServerPlugins as LightyearServerPlugins;

use lightyear::client::config::{ClientConfig, NetcodeConfig};
use lightyear::client::plugin::ClientPlugins as LightyearClientPlugins;
use lightyear::prelude::client::{Authentication, ClientTransport, IoConfig, NetConfig};

/// Plugin defining server entities and their interactions with network libraries.
#[rustfmt::skip]
pub(crate) struct LightyearPlugin;
impl Plugin for LightyearPlugin {
    fn build(&self, app: &mut App) {
        app
            // Load a hostserver config by default, even though we start offline. 
            // Just to have the resource hot and ready to go.
            .add_plugins(LightyearServerPlugins::new(get_hostserver_config(
                Ipv4Addr::LOCALHOST,
                TEST_PORT,
            )))
            .add_plugins(LightyearClientPlugins::new(get_local_client_config(0)));
    }
}

/// Simple test port.
const TEST_PORT: u16 = 7142;

/// Gets the lightyear server config for a default, basic hostserver configuration.
#[must_use = "You called for a ServerConfig, but never used it?"]
pub fn get_hostserver_config(addr: Ipv4Addr, port: u16) -> ServerConfig {
    ServerConfig {
        shared: SharedConfig {
            mode: Mode::HostServer,
            ..Default::default()
        },
        net: vec![ServerNetConfig::Netcode {
            config: ServerNetcodeConfig::default()
                .with_protocol_id(0)
                .with_key([0; 32]),
            io: ServerIoConfig::from_transport(ServerTransport::UdpSocket(SocketAddr::V4(
                SocketAddrV4::new(addr, port),
            ))),
        }],
        ..Default::default()
    }
}

/// Gets the lightyear local client config for a default, basic hostserver configuration.
#[must_use = "You called for a ClientConfig, but never used it?"]
pub fn get_local_client_config(id: u64) -> ClientConfig {
    ClientConfig {
        shared: SharedConfig {
            mode: Mode::HostServer,
            ..Default::default()
        },
        net: NetConfig::Local { id },
        ..Default::default()
    }
}

/// Gets the lightyear local client config for a default, basic hostserver configuration.
#[must_use = "You called for a ClientConfig, but never used it?"]
pub fn get_netcode_client_config(
    server_addr: Ipv4Addr,
    server_port: u16,
    client_id: u64,
    client_addr: Ipv4Addr,
    client_port: u16,
) -> ClientConfig {
    ClientConfig {
        shared: SharedConfig {
            mode: Mode::HostServer,
            ..Default::default()
        },
        net: NetConfig::Netcode {
            auth: Authentication::Manual {
                server_addr: SocketAddr::V4(SocketAddrV4::new(server_addr, server_port)),
                client_id,
                private_key: [0; 32],
                protocol_id: 0,
            },
            config: NetcodeConfig::default(),
            io: IoConfig::from_transport(ClientTransport::UdpSocket(SocketAddr::V4(
                SocketAddrV4::new(client_addr, client_port),
            ))),
        },
        ..Default::default()
    }
}
