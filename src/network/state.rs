//! Module level docs for handling the networking state machine.

use bevy::prelude::*;

/// Plugin containing the networking state machine.
pub(crate) struct NetworkStatePlugin;
impl Plugin for NetworkStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LocalNetworkState>()
            .enable_state_scoped_entities::<LocalNetworkState>();
    }
}

/// Main network state machine for the local Granite instance.
#[non_exhaustive]
#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Reflect)]
pub enum LocalNetworkState {
    #[default]
    /// No network connectivity, no servers, no nothing.
    Offline,
    /// In a hostserver or listenserver setup, with the user as a local client.
    HostServer,
    /// Standalone dedicated server, running as a headless app.
    Server,
    /// Connecting to a remote server as a client.
    Client,
}
