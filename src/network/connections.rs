//! Module level docs for handling all connections.

use serde::{Serialize, Deserialize};
use bevy::prelude::*;

/// Plugin defining connection entities.
pub struct ConnectionsPlugin;
impl Plugin for ConnectionsPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

/// Generic marker component separating the types of connections.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ConnectionKind<K> {
    /// Type marker for Query filtering.
    connection_kind: K
}

/// Represents a connection to a client.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ClientConnection;

/// Represents a connection to a server. At the moment, a Granite app can only have one of these at a time, and not if you're the host!
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ServerConnection;

/// Generic marker component separating connections by connection status.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ConnectionStatus<S>{
    /// Type marker for Query filtering.
    connection_status: S
}

/// This connection hasn't sent a keep-alive in a little while.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct LostConnection;

/// This connection has been deliberately closed.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct Disconnected;

/// This connection is in the process of establishing itself.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct Connecting;

/// This connection has been established.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct Connected;

/// Generic marker component separating connections by level, Workbench or Material.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct ConnectionLevel<L>{
    /// Type marker for Query filtering.
    connection_level: L
}

/// This connection is at the Workbench level.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct WorkbenchConnection;

/// This connection is at the Material level
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct MaterialConnection;