//! crate level doc for `bevy_granite`
//!
//! # Features
//! Write the feature things here, eventually.
//! We got uuuuuuuuuuuuuuuuhhhhh... `std` and `networking`, which are default.

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

#[cfg(feature = "networking")]
use network::NetworkPlugin;
#[cfg(feature = "networking")]
mod network;

use workbench::WorkbenchPlugin;

mod workbench;

/// Re-export of all of Granite's common tools.
#[expect(
    clippy::pub_use,
    reason = "Easier to control what is exposed during early development. Organize into pub modules later when the final design is clearer. Or don't!"
)]
pub mod prelude {

    #[cfg(feature = "networking")]
    #[doc(hidden)]
    pub use crate::network::{
        avatar::{Avatar, LocusColor},
        lightyear::{
            get_hostserver_config, get_local_client_config, get_netcode_client_config,
            get_server_config,
        },
        state::LocalNetworkState,
    };

    #[doc(hidden)]
    pub use crate::{
        workbench::structure::{GraniteRoot, OrdonnanceStratum},
        GranitePlugin,
    };
}

/// Primary documentation for the Granite plugin.
pub struct GranitePlugin;
impl Plugin for GranitePlugin {
    fn build(&self, app: &mut App) {
        // Load first, has Resources used in later plugins.
        #[cfg(feature = "networking")]
        app.add_plugins(NetworkPlugin);

        app.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .add_plugins(WorkbenchPlugin);
    }
}
