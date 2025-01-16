//! Module level docs for the local server architecture

use bevy::prelude::*;

/// Plugin defining server entities and their interactions with network libraries.
pub (crate) struct LocalServerPlugin;
impl Plugin for LocalServerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
impl LocalServerPlugin {
    /// What port the server will listen on by default.
    const DEFAULT_PORT: u16 = 7142;
    /// Default protocol ID for the server.
    const DEFAULT_PROTOCOL_ID: u64 = 0;
}