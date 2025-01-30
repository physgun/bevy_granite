//! module level docs for the network plugin

use bevy::prelude::*;

pub(crate) mod avatar;
pub(crate) mod connections;
pub(crate) mod lightyear;
pub(crate) mod state;

use avatar::AvatarPlugin;
use connections::ConnectionsPlugin;
use lightyear::LightyearPlugin;
use state::NetworkStatePlugin;

/// Plugin to enable network features.
pub(crate) struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LightyearPlugin)
            .add_plugins(NetworkStatePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(ConnectionsPlugin);
    }
}
