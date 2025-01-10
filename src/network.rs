//! module level docs for the network plugin

use bevy::prelude::*;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet2::RepliconRenetPlugins;

mod state;
mod avatar;
mod connections;
mod servers;
mod clients;

use state::NetworkStatePlugin;
use avatar::AvatarPlugin;
use connections::ConnectionsPlugin;
use servers::LocalServerPlugin;

/// Plugin to enable network features.
pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RepliconPlugins)
            .add_plugins(RepliconRenetPlugins)
            .add_plugins(NetworkStatePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(ConnectionsPlugin)
            .add_plugins(LocalServerPlugin);
    }
}