//! Module level docs for the local server architecture

use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;

use lightyear::prelude::server::{IoConfig, NetConfig, ServerTransport};
use lightyear::prelude::{Mode, SharedConfig};
use lightyear::server::config::{NetcodeConfig, ServerConfig};
use lightyear::server::plugin::ServerPlugins as LightyearServerPlugins;

/// Plugin defining server entities and their interactions with network libraries.
pub (crate) struct LocalServerPlugin;
impl Plugin for LocalServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LightyearServerPlugins::new(ServerConfig { 
                shared: SharedConfig {
                    mode: Mode::HostServer,
                    ..Default::default()
                },  
                net: vec![NetConfig::Netcode { 
                    config: NetcodeConfig::default(), 
                    io: IoConfig::from_transport(ServerTransport::UdpSocket(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 7142)))) 
                }],
                ..Default::default()
            }));
    }
}