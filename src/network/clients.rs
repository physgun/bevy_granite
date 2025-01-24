//! Logic around being a client.

use bevy::prelude::*;

use lightyear::client::config::ClientConfig;
use lightyear::client::plugin::ClientPlugins as LightyearClientPlugins;
use lightyear::prelude::client::NetConfig;
use lightyear::prelude::{Mode, SharedConfig};

/// Plugin defining server entities and their interactions with network libraries.
pub (crate) struct LocalClientPlugin;
impl Plugin for LocalClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LightyearClientPlugins::new(ClientConfig{
                shared: SharedConfig {
                    mode: Mode::HostServer,
                    ..Default::default()
                },
                net: NetConfig::Local { id: 714 },
                ..Default::default()
            }));
    }
}