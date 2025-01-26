//! Module level docs for handling all connections.

use core::net::{IpAddr, Ipv6Addr};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Plugin defining connection entities.
pub(crate) struct ConnectionsPlugin;
impl Plugin for ConnectionsPlugin {
    fn build(&self, app: &mut App) {}
}

// TODO: Honestly, the network libraries handle all of this. Do we really need any of it?

/// Main component for an entity representing some kind of network connection.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Connection {
    /// IP Address of the connection.
    address: IpAddr,
    /// Port of the connection.
    port: u16,
    /// What role the target of this conection plays in the network topology.
    topology: ConnectionTopology,
    /// The... connection status. Not much more to say about it.
    status: ConnectionStatus,
    /// Describes if the connection is sharing the material project or sharing both the material and workbench UI.
    level: ConnectionLevel,
}
impl Default for Connection {
    fn default() -> Self {
        Connection {
            address: IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            port: 7142,
            topology: ConnectionTopology::CentralServer,
            status: ConnectionStatus::Initialized,
            level: ConnectionLevel::Material,
        }
    }
}
impl Connection {
    /// Spawns a new [`Connection`] with custom parameters.
    pub fn new(
        address: IpAddr,
        port: u16,
        topology: ConnectionTopology,
        status: ConnectionStatus,
        level: ConnectionLevel,
    ) -> Self {
        Connection {
            address,
            port,
            topology,
            status,
            level,
        }
    }
}

/// What role the target of this conection plays in the network topology.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub enum ConnectionTopology {
    /// Represents a connection to a client, who we are sending simulation info to.
    #[default]
    ClientLeaf,
    /// Represents a connection to a central server, who has authority over the entire network.
    CentralServer,
    /// Represents a connection to a relay server, who interfaces with foreign data types.
    RelayServer,
    /// Represents a connection to a resource server, who is delegated simulation work.
    ResourceServer,
}

/// The... connection status. Not much more to say about it.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub enum ConnectionStatus {
    /// This connection has just been created, and needs configured.
    #[default]
    Initialized,
    /// This connection hasn't sent a keep-alive in a little while.
    LostConnection,
    /// This connection has been deliberately closed in some way.
    Disconnected,
    /// This connection is in the process of establishing itself.
    Connecting,
    /// This connection has been established.
    Connected,
}

/// Describes if the connection is sharing the material project or sharing both the material and workbench UI.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub enum ConnectionLevel {
    /// Sharing data on the project itself, using one's own workbench UI config.
    #[default]
    Material,
    /// Sharing data on both the project and the authority's workbench UI too.
    /// Allows one to interact directly with the UI of the central server.
    Workbench,
}
