//! module level docs for the network plugin

use bevy::prelude::*;

pub (crate) mod state;
pub (crate) mod avatar;
pub (crate) mod connections;
pub (crate) mod servers;
pub (crate) mod clients;

use state::NetworkStatePlugin;
use avatar::AvatarPlugin;
use connections::ConnectionsPlugin;
use servers::LocalServerPlugin;

/// Plugin to enable network features.
pub (crate) struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(NetworkStatePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(ConnectionsPlugin)
            .add_plugins(LocalServerPlugin);
    }
}