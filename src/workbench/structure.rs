//! The basis for the workbench UI, defining the space where it will take place.

use bevy::{prelude::*, ui::RelativeCursorPosition};
use lightyear::prelude::{AppComponentExt, ChannelDirection, ClientReplicate, Replicated};
use serde::{Deserialize, Serialize};

/// The base UI logic of the Workbench level.
pub(crate) struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "networking")]
        {
            app.register_component::<LociStratum>(ChannelDirection::Bidirectional);
            app.register_component::<OrdonnanceStratum>(ChannelDirection::ServerToClient);
        };

        app.add_event::<LocalLociStratumSpawned>()
            .add_event::<LocalOrdonnanceStratumSpawned>()
            .add_observer(Self::observer_configures_new_local_loci_stratum)
            .add_observer(Self::observer_configures_new_local_ordonnance_stratum)
            .add_observer(Self::observer_configures_new_granite_root);
    }
}
impl StructurePlugin {
    /// Observer to configure newly spawned [`GraniteRoot`] nodes.
    fn observer_configures_new_granite_root(
        trigger: Trigger<OnAdd, GraniteRoot>,
        mut commands: Commands,
        mut granite_root_query: Query<(Entity, &mut Node), With<GraniteRoot>>,
    ) {
        let Ok((granite_node_entity, mut granite_node)) =
            granite_root_query.get_mut(trigger.entity())
        else {
            error!("[STRUCTURE] Could not get newly spawned GraniteRoot for configuration!");
            return;
        };

        let ordonnance_stratum = commands
            .spawn(OrdonnanceStratum)
            .trigger(LocalOrdonnanceStratumSpawned)
            .id();

        commands
            .entity(granite_node_entity)
            .insert(Name::new("Granite Root Node"))
            .add_child(ordonnance_stratum);

        // Root nodes will eat up 100% of camera space by default.
        // Unknown if future functionality will allow users to do more exotic things.
        granite_node.height = Val::Percent(100.0);
        granite_node.width = Val::Percent(100.0);
    }

    /// Observer to configure newly spawned [`OrdonnanceStratum`] nodes.
    /// Only configures locally spawned ones! Replicated ones don't need configuring and aren't touched.
    fn observer_configures_new_local_ordonnance_stratum(
        trigger: Trigger<LocalOrdonnanceStratumSpawned>,
        mut commands: Commands,
        mut ordonnance_stratum_query: Query<
            (Entity, &mut Node),
            (With<OrdonnanceStratum>, Without<Replicated>),
        >,
    ) {
        let Ok((ordonnance_stratum_entity, mut ordonnance_stratum_node)) =
            ordonnance_stratum_query.get_mut(trigger.entity())
        else {
            error!("[STRUCTURE] Could not get newly spawned OrdonnanceStratum for configuration!");
            return;
        };

        let loci_stratum = commands
            .spawn(LociStratum)
            .trigger(LocalLociStratumSpawned)
            .id();

        // Stratum nodes are layered over the GraniteRoot node, for wholesale network replication.
        commands
            .entity(ordonnance_stratum_entity)
            .insert((Name::new("Ordonnance Stratum Node"),))
            .add_child(loci_stratum);

        ordonnance_stratum_node.height = Val::Percent(100.0);
        ordonnance_stratum_node.width = Val::Percent(100.0);

        // These are for the temporary, debug UI should the Workbench be unavailable (it is).
        ordonnance_stratum_node.padding = UiRect::all(Val::Px(5.0));
        ordonnance_stratum_node.display = Display::Flex;
        ordonnance_stratum_node.flex_direction = FlexDirection::Row;
        ordonnance_stratum_node.flex_wrap = FlexWrap::WrapReverse;
        ordonnance_stratum_node.justify_content = JustifyContent::FlexStart;
        ordonnance_stratum_node.align_items = AlignItems::FlexEnd;
        ordonnance_stratum_node.row_gap = Val::Px(5.0);
        ordonnance_stratum_node.column_gap = Val::Px(5.0);
    }

    /// Observer to configure newly spawned [`LociStratum`] nodes.
    /// Local ones only, this observer doesn't touch replicated ones.
    fn observer_configures_new_local_loci_stratum(
        trigger: Trigger<LocalLociStratumSpawned>,
        mut commands: Commands,
        mut loci_stratum_query: Query<
            (Entity, &mut Node),
            (With<LociStratum>, Without<Replicated>),
        >,
    ) {
        let Ok((loci_stratum_entity, mut loci_stratum_node)) =
            loci_stratum_query.get_mut(trigger.entity())
        else {
            error!("[STRUCTURE] Could not get newly spawned LociStratum for configuration!");
            return;
        };

        // Stratum nodes are layered over the GraniteRoot node, for wholesale network replication.
        commands.entity(loci_stratum_entity).insert((
            Name::new("Loci Stratum Node"),
            GlobalZIndex(1_000_000),
            ClientReplicate::default()
        ));
        loci_stratum_node.height = Val::Percent(100.0);
        loci_stratum_node.width = Val::Percent(100.0);
    }
}

/// Marker component for the root UI `Node` for a Granite instance. Everything `bevy_granite` will be constrained to these root nodes.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[require(Node)]
pub struct GraniteRoot;

/// Marker component for the stratum node containing all [`Avatar`](crate::network::avatar::Avatar) loci. This node, and all of its children, is replicated to remote servers.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[require(Node, RelativeCursorPosition)]
pub(crate) struct LociStratum;

/// Event to be triggered when a new local [`LociStratum`] is spawned.
/// This is so observers can configure just the local one, and ignore replicated ones.
#[derive(Event, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub(crate) struct LocalLociStratumSpawned;

/// Marker component for the stratum node containing all of the Workbench UI elements. Replicated to clients if hosting on the Workbench level.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[require(Node)]
pub struct OrdonnanceStratum;

/// Event to be triggered when a new local [`OrdonnanceStratum`] is spawned.
/// This is so observers can configure just the local one, and ignore replicated ones.
#[derive(Event, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub(crate) struct LocalOrdonnanceStratumSpawned;
