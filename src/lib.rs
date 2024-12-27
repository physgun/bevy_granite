//! crate level doc for bevy_granite

use bevy::prelude::*;
use network::NetworkPlugin;
use workbench::WorkbenchPlugin;

pub mod network;
pub mod workbench;

pub struct GranitePlugin;
impl Plugin for GranitePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
            })
            .add_plugins(NetworkPlugin)
            .add_plugins(WorkbenchPlugin);
    }
}