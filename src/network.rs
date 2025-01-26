//! module level docs for the network plugin

use bevy::prelude::*;

pub(crate) mod avatar;
pub(crate) mod clients;
pub(crate) mod connections;
pub(crate) mod servers;
pub(crate) mod state;

use avatar::AvatarPlugin;
use clients::LocalClientPlugin;
use connections::ConnectionsPlugin;
use servers::LocalServerPlugin;
use state::NetworkStatePlugin;

/// Plugin to enable network features.
pub(crate) struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LocalServerPlugin)
            .add_plugins(LocalClientPlugin)
            .add_plugins(NetworkStatePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(ConnectionsPlugin);
    }
}
