//! Module level docs for handling the networking state machine.

use core::net::Ipv4Addr;

use bevy::prelude::*;

/// Plugin containing the networking state machine.
pub(crate) struct NetworkStatePlugin;
impl Plugin for NetworkStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LocalNetworkState>();
    }
}

/// Main network state machine for the local Granite instance.
#[non_exhaustive]
#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum LocalNetworkState {
    #[default]
    /// No network connectivity, no servers, no nothing.
    Offline,
    /// In a hostserver or listenserver setup, with the user as a local client.
    HostServer {
        /// Address of the server.
        server_addr: Ipv4Addr,
        /// Connection port the server will use.
        server_port: u16,
    },
    /// Standalone dedicated server, running as a headless app.
    Server {
        /// Address of the server.
        server_addr: Ipv4Addr,
        /// Connection port the server will use.
        server_port: u16,
    },
    /// Connecting to a remote server as a client.
    Client {
        /// Address of the server.
        server_addr: Ipv4Addr,
        /// Connection port the server will use.
        server_port: u16,
        /// ID of the client.
        client_id: u64,
        /// Client's address??
        client_addr: Ipv4Addr,
        /// Client's port??
        client_port: u16,
    },
}
