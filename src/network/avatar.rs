//! The network layer of Bevy Components representing the user, which are shared and replicated between hostservers.

use serde::{Serialize, Deserialize};
use bevy::{ecs::entity::MapEntities, math::Vec2, prelude::*};
use bevy_replicon::{core::ClientId, prelude::AppRuleExt};

pub struct AvatarPlugin;
impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app
            .replicate_group::<(Avatar, AvatarID)>()
            .replicate_group::<(AvatarWorkbenchLocus, LocusType, LocusPosition)>()
            .replicate_mapped::<LocusWindow>();
    }
}

/// Marker component for the entity representation of the user. Requires all functional components.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(AvatarID)]
pub struct Avatar;

/// Primary ID of the user. Stores every ID that libraries will need.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AvatarID {
    replicon_id: ClientId
}
impl Default for AvatarID {
    fn default() -> Self {
        AvatarID { replicon_id: ClientId::SERVER }
    }
}
impl AvatarID {
    fn new(rep_id: u64) -> Self {
        AvatarID { replicon_id: ClientId::new(rep_id) }
    }
}

/// Marker component for an entity that represents a user's presence, such as cursors or focuses.
/// 
/// ### Network Strategy
/// Entities with this component are sent over the network to facilitate user-to-user communication within the host's Workbench.
/// Loci data can be lost or dropped anytime without loss, and are not required to be saved away for a host transfer.
/// Best sent on Ordered, Unreliable channels, with the client ideally having replication authority over their own loci.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(LocusType, LocusPosition, LocusWindow)]
pub struct AvatarWorkbenchLocus;

/// What kind of user representation this locus will convey.
/// 
/// - `Mouse` is a standard cursor that has an `x, y` position.
/// - `Focus` is a highlighted section or button, for gamepad navigation.
/// - `Touchprint` is a finger on a touchscreen.
/// - `Stylus` is a pressure-sensitive `Mouse` that may have a trail.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub enum LocusType {
    #[default]
    Mouse,
    Focus,
    Touchprint,
    Stylus
}

/// In relative screenspace coordinates, from top-left `(0.0, 0.0)` to bottom right `(1.0, 1.0)`
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
    pub fn pos(self) -> Vec2 {
        self.pos
    }

    /// Sets the position vector.
    pub fn set_pos(&mut self, new_pos: Vec2) -> &mut Self {
        self.pos = new_pos;
        self
    }
}

/// The `Window` `Entity` the locus was detected in.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

