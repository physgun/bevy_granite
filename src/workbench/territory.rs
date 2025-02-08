//! The primary element of the granite UI. 
//! Territories are a more flexible and easy style of organization designed to address issues found during user-testing of other styles.
//! Replaces docking, splitting, and windows functionality.

use serde::{Deserialize, Serialize};
use bevy::prelude::*;

use super::structure::Cardinal;

/// Contains definitions for the Workbench UI `Territory` element. 
pub(crate) struct TerritoryPlugin;
impl Plugin for TerritoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Territory>();
    }
}

/// Driving component for the Territory UI entity. 
/// It's the big, movable, draggable, resizable box that contains the tab bars.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Node, BackgroundColor, BorderColor, BorderRadius)]
pub(crate) struct Territory;

/// Driving component for a resizable UI element's border entity. 
/// 
/// Borders of a UI entity are their own entities themselves, for smoother mutable querying.
#[derive(Component, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub(crate) struct Border<D: Cardinal>{
    /// Which direction this border faces.
    direction: D,
    /// Location of the border.
    pos: f32,
    /// Midpoint of the border.
    midpoint: f32,
    /// The smaller, starting coordinate of the border.
    start_point: f32,
    /// The larger, ending coordinate of the border.
    end_point: f32,
    /// The smallest size the border can be.
    minimum_size: f32
}