//! Module level docs for handling the networking state machine.

use bevy::prelude::*;

/// Plugin containing the networking state machine.
pub (crate) struct NetworkStatePlugin;
impl Plugin for NetworkStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<LocalNetworkState>()
            .add_sub_state::<ServerState>();
    }
}

/// Main network state for the local Granite instance.
#[non_exhaustive]
#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub (crate) enum LocalNetworkState {
    #[default]
    /// A general place to hang out while everyone gets ready.
    Staging,
    /// We are a server, and have authority over some shared computation.
    Server,
    /// We are a client, a leaf in a network.
    Client,
    /// The network we are a part of is undergoing some kind of change in server topology.
    Migration
}

/// Substate for the [`LocalNetworkState::Server`] Bevy State.
#[non_exhaustive]
#[derive(SubStates, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[source(LocalNetworkState = LocalNetworkState::Server)]
pub (crate) enum ServerState {
    #[default]
    /// We are the final authority in a network, and the core of the server topology.
    Central,
    /// We pass things along and interface between incompatible things.
    Relay,
    /// We are delegated some specialized task or responsibility.
    Resource
}
