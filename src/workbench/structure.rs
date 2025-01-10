//! The basis for the workbench UI, defining the space where it will take place.

use serde::{Serialize, Deserialize};
use bevy::{ui::RelativeCursorPosition, prelude::*};

/// The base UI logic of the Workbench level.
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
        mut commands: Commands,
        mut granite_root_query: Query<(Entity, &mut Node), With<GraniteRoot>>
    ) {
        let Ok((granite_node_entity, mut granite_node)) = granite_root_query.get_mut(trigger.entity()) else {
            error!("Could not get newly spawned GraniteRoot!");
            return;
        }; 

        commands.entity(granite_node_entity).insert(Name::new("Granite Root Node"));
        granite_node.height = Val::Percent(100.0);
        granite_node.width = Val::Percent(100.0);

        // These are for the temporary, debug UI should the Workbench be unavailable.
        granite_node.padding = UiRect::all(Val::Px(5.0));
        granite_node.display = Display::Flex;
        granite_node.flex_direction = FlexDirection::Row;
        granite_node.flex_wrap = FlexWrap::WrapReverse;
        granite_node.justify_content = JustifyContent::FlexStart;
        granite_node.align_items = AlignItems::FlexEnd;
        granite_node.align_content = AlignContent::FlexEnd;
        granite_node.row_gap = Val::Px(5.0);
        granite_node.column_gap = Val::Px(5.0);
    }
}

/// Marker component for the root UI `Node` for a Granite instance. Everything `bevy_granite` will be constrained to these root nodes.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[require(Node, RelativeCursorPosition)]
pub struct GraniteRoot;