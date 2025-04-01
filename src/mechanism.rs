//! module level docs for the mechanism plugin

use bevy::prelude::*;

pub(crate) mod catena;

use catena::CatenaPlugin;

/// General overview of the mechanisms.
pub(crate) struct MechanismPlugin;
impl Plugin for MechanismPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CatenaPlugin);
    }
}
