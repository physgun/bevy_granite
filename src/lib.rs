//! crate level doc for `bevy_granite`

// Lints incompatible with Bevy
#![allow(
    clippy::needless_pass_by_value,
    reason = "Incompatible with Bevy's dependency injection techniques."
)]
#![allow(
    clippy::type_complexity,
    reason = "Incompatible with Bevy's ECS Query<> idioms."
)]

use bevy::prelude::*;

use bevy::winit::{UpdateMode, WinitSettings};

use network::NetworkPlugin;
use workbench::WorkbenchPlugin;

mod network;
mod workbench;

/// Re-export of all of Granite's common tools.
#[expect(
    clippy::pub_use,
    reason = "Easier to control what is exposed during early development. Organize into pub modules later when the final design is clearer. "
)]
pub mod prelude {

    #[doc(hidden)]
    pub use crate::{
        network::avatar::{Avatar, LocusColor},
        network::servers::{
            StartedConnecting, StartedHostServer, StoppedHostServer, WasDisconnected,
        },
        workbench::structure::{GraniteRoot, OrdonnanceStratum},
        GranitePlugin,
    };
}

/// Primary documentation for the Granite plugin.
pub struct GranitePlugin;
impl Plugin for GranitePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .add_plugins(NetworkPlugin)
        .add_plugins(WorkbenchPlugin);
    }
}
