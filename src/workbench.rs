//! module level docs for the workbench plugin

use bevy::prelude::*;

pub mod structure;

use structure::StructurePlugin;

/// Explain what a Workbench even is.
pub struct WorkbenchPlugin;
impl Plugin for WorkbenchPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(StructurePlugin);
    }
}