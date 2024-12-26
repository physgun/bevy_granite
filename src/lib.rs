//! crate level doc for bevy_granite

use bevy::prelude::*;
use network::NetworkPlugin;

mod network;

pub struct GranitePlugin;
impl Plugin for GranitePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkPlugin);
    }
}