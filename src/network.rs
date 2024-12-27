//! module level docs for the network plugin

use bevy::prelude::*;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet2::RepliconRenetPlugins;

mod channels;
mod avatar;
mod messages;
mod input;
mod clientele;

use avatar::AvatarPlugin;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RepliconPlugins)
            .add_plugins(RepliconRenetPlugins)
            .add_plugins(AvatarPlugin);
    }
}