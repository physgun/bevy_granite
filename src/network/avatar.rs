//! The network layer of Bevy Components representing the user, which are shared and replicated between hostservers.

use serde::{Serialize, Deserialize};
use bevy::{ecs::entity::MapEntities, input::mouse::MouseMotion, math::Vec2, prelude::*, ui::RelativeCursorPosition};
use bevy_replicon::{core::ClientId, prelude::{AppRuleExt, Replicated}};

use crate::workbench::structure::GraniteRoot;

pub struct AvatarPlugin;
impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app
            .replicate::<AvatarProfile>()
            .replicate::<LocusKind<MouseLocus>>()
            .replicate::<LocusKind<FocusLocus>>()
            .replicate::<LocusKind<TouchprintLocus>>()
            .replicate::<LocusKind<StylusLocus>>()
            .replicate::<LocusPosition>()
            .replicate::<LocusColor>()
            .replicate::<LocusDetected>()

            .add_observer(Self::add_local_loci_children)

            // TODO: Order before network stuff gets sent out.
            .add_systems(PreUpdate, (Self::detect_local_mouse_loci).run_if(on_event::<MouseMotion>));
    }
}
impl AvatarPlugin {
    /// Observer for adding local locus child entities to a freshly spawned [`GraniteRoot`] UI root node.
    fn add_local_loci_children(
        trigger: Trigger<OnAdd, GraniteRoot>,
        mut commands: Commands
    ) {
        let image_node_color = LocusColor::MATERIAL_LIGHT.tertiary_color();

        let mouse_locus_entity = commands.spawn((
            AvatarLocal,
            AvatarProfile::SERVER, 
            Visibility::Hidden,
            BackgroundColor(image_node_color),
            BorderRadius::new(Val::ZERO, Val::Px(2.5), Val::Px(7.5), Val::Px(2.5)),
            Node {
                position_type: PositionType::Absolute,
                height: Val::Px(7.5),
                width: Val::Px(7.5),
                ..Default::default()
            },
            ImageNode::solid_color(image_node_color),
            GlobalZIndex(1000000),
            LocusKind{ locus_type: MouseLocus }, 
            LocusPosition::default(),
            LocusColor::MATERIAL_LIGHT,
            Replicated)
        ).id();
        commands.entity(trigger.entity()).add_child(mouse_locus_entity);
    }

    /// Updates all local mouse loci by checking if its parent `GraniteRoot`'s `RelativeCursorPosition` detected a mouse.
    fn detect_local_mouse_loci(
        mut commands: Commands,
        root_detection_query: Query<&RelativeCursorPosition, With<GraniteRoot>>,
        mut local_mouse_locus: Query<
        (Entity, &Parent ,&mut Node, &mut LocusPosition, &mut Visibility, Option<&LocusDetected>),
        (With<LocusKind<MouseLocus>>, With<AvatarLocal>)>
    ) {
        for (
            mouse_locus_entity, 
            granite_root_parent, 
            mut mouse_locus_node,
            mut locus_pos,
            mut node_visibility,
            locus_detected) in &mut local_mouse_locus {

            let Ok(parent_relcurpos) = root_detection_query.get(granite_root_parent.get()) else {
                error!("Mouse locus parent GraniteRoot entity not found with RelativeCursorPosition component!");
                continue;
            };

            let Some(detected_relative_position) = parent_relcurpos.normalized else {
                if let Visibility::Inherited = *node_visibility { node_visibility.toggle_inherited_hidden(); } 
                if locus_detected.is_some() { commands.entity(mouse_locus_entity).remove::<LocusDetected>(); }
                continue;
            };

            // Stick a copy in here, to replicate to others if needed.
            locus_pos.set_pos(detected_relative_position);

            mouse_locus_node.left = Val::Percent(detected_relative_position.x * 100.0);
            mouse_locus_node.top = Val::Percent(detected_relative_position.y * 100.0);
            if let Visibility::Hidden =  *node_visibility { node_visibility.toggle_inherited_hidden(); }

            // Also needed for replication, as `Visibility` doesn't implement serde traits: 
            if locus_detected.is_none() { commands.entity(mouse_locus_entity).insert(LocusDetected); }
        }
    }
}

/// Marker component separating out entities that belong to the local user and are not replicated from a server.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct AvatarLocal;

/// Main component for the entity representation of a user. Attached to everything associated with that user.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct AvatarProfile {
    replicon_id: ClientId
}
impl Default for AvatarProfile {
    fn default() -> Self {
        AvatarProfile { replicon_id: ClientId::new(1) }
    }
}
impl AvatarProfile {
    const SERVER: AvatarProfile = AvatarProfile { replicon_id: ClientId::SERVER };

    fn new(rep_id: u64) -> Self {
        AvatarProfile { replicon_id: ClientId::new(rep_id) }
    }
}

/// Marker component for an entity that represents a user's presence, such as cursors or focuses.
/// 
/// ### Network Strategy
/// Entities with this component are sent over the network to facilitate user-to-user communication within the host's Workbench.
/// Loci data can be lost or dropped anytime without loss, and are not required to be saved away for a host transfer.
/// Best sent on Ordered, Unreliable channels, with the client ideally having replication authority over their own loci.
/// What kind of user representation this locus will convey. Generic for query filtering.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusKind<K> {
    locus_type: K
}

/// `MouseLocus` is a standard cursor that has an `x, y` relative screenspace position.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct MouseLocus;

/// `FocusLocus` is a highlighted section or button, for gamepad navigation.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct FocusLocus;

/// `TouchprintLocus` is a finger on a touchscreen.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct TouchprintLocus;

/// `StylusLocus` is a pressure-sensitive `Mouse` that may have a trail.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub struct StylusLocus;

/// Unique color scheme of the user's locus.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusColor {
    primary_color: Srgba,
    secondary_color: Srgba,
    tertiary_color: Srgba
}
impl Default for LocusColor {
    fn default() -> Self {
        LocusColor { 
            primary_color: bevy::color::palettes::css::WHITE, 
            secondary_color: bevy::color::palettes::css::GOLD, 
            tertiary_color: bevy::color::palettes::css::AQUA 
        }
    }
}
impl LocusColor {
    const MATERIAL_LIGHT: LocusColor = LocusColor {
        primary_color: bevy::color::palettes::css::WHITE, 
        secondary_color: bevy::color::palettes::css::GOLD, 
        tertiary_color: bevy::color::palettes::css::AQUA 
    };

    const MATERIAL_DARK: LocusColor = LocusColor {
        primary_color: bevy::color::palettes::css::BLACK, 
        secondary_color: bevy::color::palettes::css::GOLD, 
        tertiary_color: bevy::color::palettes::css::AQUA 
    };

    pub fn new(primary_color: Srgba, secondary_color: Srgba, tertiary_color: Srgba) -> Self {
        LocusColor { primary_color, secondary_color, tertiary_color }
    }

    /// Returns the primary color.
    pub fn primary_color(self) -> Color {
        self.primary_color.into()
    }

    /// Returns the secondary color.
    pub fn secondary_color(self) -> Color {
        self.secondary_color.into()
    }

    /// Returns the tertiary color.
    pub fn tertiary_color(self) -> Color {
        self.tertiary_color.into()
    }
}

/// In relative screenspace coordinates, from top-left `(0.0, 0.0)` to bottom right `(1.0, 1.0)`  
/// 
/// Here for the network replication, too difficult to send over the `ImageNode` positions.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusPosition {
    pos: Vec2
}
impl Default for LocusPosition {
    fn default() -> Self {
        LocusPosition { pos: Vec2::new(0.5, 0.5) }
    }
}
impl LocusPosition {
    /// Gets the position vector.
    pub fn get_pos(self) -> Vec2 {
        self.pos
    }

    /// Sets the position vector.
    pub fn set_pos(&mut self, new_pos: Vec2) -> &mut Self {
        self.pos = new_pos;
        self
    }
}

/// The `Window` `Entity` the locus was detected in.
/// TODO: Replace with the Ui Root construct
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusWindow {
    entity: Entity
}
impl Default for LocusWindow {
    fn default() -> Self {
        LocusWindow { entity: Entity::PLACEHOLDER }
    }
}
impl MapEntities for LocusWindow {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.entity = entity_mapper.map_entity(self.entity);
    }
}
impl LocusWindow {
    /// Gets the window `Entity`
    pub fn entity(self) -> Self {
        self.entity;
        self
    }

    /// Sets the window `Entity`
    pub fn set_entity(&mut self, new_entity: Entity) -> &mut Self {
        self.entity = new_entity;
        self
    }
}

/// Stores the GraniteRoot Ui Node Entity that this locus was last spotted on.
/// Needs to be later refactored to accept `Option<Entity` for mapping over the network.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusRoot {
    entity: Entity
}
impl Default for LocusRoot {
    fn default() -> Self {
        LocusRoot { entity: Entity::PLACEHOLDER }
    }
}
impl MapEntities for LocusRoot {
    /// We'll need to update this later to accept `Option<Entity>`
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.entity = entity_mapper.map_entity(self.entity);
    }
}

/// Marker component labeling the entity as having been detected, and will show up on rendering queries for that entity.  
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusDetected;

