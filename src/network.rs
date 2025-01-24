//! module level docs for the network plugin

use bevy::prelude::*;

pub (crate) mod servers;
pub (crate) mod clients;
pub (crate) mod state;
pub (crate) mod avatar;
pub (crate) mod connections;

use state::NetworkStatePlugin;
use avatar::AvatarPlugin;
use connections::ConnectionsPlugin;
use servers::LocalServerPlugin;
use clients::LocalClientPlugin;

/// Plugin to enable network features.
pub (crate) struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LocalServerPlugin)
            .add_plugins(LocalClientPlugin)
            .add_plugins(NetworkStatePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(ConnectionsPlugin);
    }
}