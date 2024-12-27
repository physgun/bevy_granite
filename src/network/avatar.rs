//! The network layer of Bevy Components representing the user, which are shared and replicated between hostservers.

use serde::{Serialize, Deserialize};
use bevy::{ecs::entity::MapEntities, input::mouse::MouseMotion, math::Vec2, prelude::*};
use bevy_replicon::{core::ClientId, prelude::{AppRuleExt, Replicated}};

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
            .replicate_mapped::<LocusWindow>()
            .replicate::<LocusDetected>()
            
            .add_systems(Startup, Self::init_local_locus_entities)
            .add_systems(PreUpdate, Self::update_local_mouse_loci)
            .add_systems(Update, Self::draw_mouse_loci_gizmos);
    }
}
impl AvatarPlugin {
    /// Spawn in the locus-tracking, replicated entities on Startup for our listen server-client.
    fn init_local_locus_entities(mut commands: Commands) {

        // Only one mouse per user is supported. Don't see that changing, ever?
        commands.spawn((
            AvatarProfile::SERVER, 
            LocusKind{ locus_type: MouseLocus }, 
            LocusPosition::default(),
            LocusColor::MATERIAL_LIGHT,
            LocusWindow::default(),
            Replicated)
        );
    }

    /// Updates all local mouse "loci" by searching through all `Window`s for a `cursor_position()`. 
    /// Inserts or removes `LocusDetected` depending on the search results.
    fn update_local_mouse_loci(
        mut commands: Commands,
        windows_query: Query<(Entity, &Window)>,
        mut mouse_locus_query: Query<
            (Entity, &mut LocusPosition, &mut LocusWindow, Option<&LocusDetected>), 
            With<LocusKind<MouseLocus>>>,
        mouse_moved_event_box: EventReader<MouseMotion>
    ) {
        // Don't bother checking or updating if the mouse never moved.
        if mouse_moved_event_box.is_empty() {return;}

        let Ok(
            (mouse_locus_entity, mut locus_pos, mut locus_window, locus_detected)
        ) = mouse_locus_query.get_single_mut() else {
            warn!("Mouse locus entity was not found!");
            return;
        };

        for (window_entity, window) in &windows_query {

            let Some(mouse_pos) = window.cursor_position() else {
                continue;
            };

            // Convert coordinates from screenspace to relative screenspace.
            let new_pos = Vec2 { 
                x: mouse_pos.x / window.size().x, 
                y: mouse_pos.y / window.size().y 
            };

            println!("Found mouse! Was {:?}, saved as {:?}", mouse_pos, new_pos);

            locus_pos.set_pos(new_pos);
            locus_window.set_entity(window_entity);

            if locus_detected.is_none() { 
                commands.entity(mouse_locus_entity).insert(LocusDetected);  
            }
            return;
        }

        // If we reach this point, no cursor_position() was ever found.
        if locus_detected.is_some() {
            commands.entity(mouse_locus_entity).remove::<LocusDetected>();
        }
    }

    fn draw_mouse_loci_gizmos(
        mut gizmos: Gizmos,
        mouse_loci_query: Query<(&LocusColor, &LocusPosition), With<LocusKind<MouseLocus>>>
    ) {
        for (color, pos) in & mouse_loci_query {
            gizmos.circle_2d(
                Isometry2d::new(pos.get_pos(), Rot2::IDENTITY), 
                5.0, 
                color.primary_color()
            );
        }
    }
}

/// Main component for the entity representation of a user. Attached to everything associated with that user.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct AvatarProfile {
    replicon_id: ClientId
}
impl Default for AvatarProfile {
    fn default() -> Self {
        AvatarProfile { replicon_id: ClientId::new(777) }
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

    pub fn primary_color(self) -> Color {
        self.primary_color.into()
    }
}

/// In relative screenspace coordinates, from top-left `(0.0, 0.0)` to bottom right `(1.0, 1.0)`
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

/// Marker component labeling the entity as having been detected, and will show up on rendering queries for that entity.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusDetected;

