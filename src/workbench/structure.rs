//! The basis for the workbench UI, defining the space where it will take place.

use serde::{Serialize, Deserialize};
use bevy::{ui::RelativeCursorPosition, prelude::*};

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        
    }
}

/// Marker component for the root UI `Node` for a Granite instance. Everything bevy_granite will be constrained to these root nodes.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[require(Node, RelativeCursorPosition)]
pub struct GraniteRoot;
impl GraniteRoot {
    pub fn new() {

    }
}