//! The basis for the workbench UI, defining the space where it will take place.

use serde::{Serialize, Deserialize};
use bevy::{ui::RelativeCursorPosition, prelude::*};

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(Self::configure_new_granite_root);
    }
}
impl StructurePlugin {
    /// Configure newly spawned [`GraniteRoot`] nodes.
    fn configure_new_granite_root(
        trigger: Trigger<OnAdd, GraniteRoot>,
        mut granite_root_query: Query<&mut Node, With<GraniteRoot>>
    ) {
        let Ok(mut node) = granite_root_query.get_mut(trigger.entity()) else {
            error!("Could not get newly spawned GraniteRoot!");
            return;
        }; 

        node.height = Val::Percent(100.0);
        node.width = Val::Percent(100.0);
    }
}

/// Marker component for the root UI `Node` for a Granite instance. Everything bevy_granite will be constrained to these root nodes.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[require(Node, RelativeCursorPosition)]
pub struct GraniteRoot;