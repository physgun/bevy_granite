//! The main unit of work are [`Catenae`]. Exactly what they do is a work in progress! They're entities, I know that much.

use core::fmt::Debug;
use bevy::prelude::*;

/// Contains definitions for the Granite mechanism [`Catena`] element.
pub(crate) struct CatenaPlugin;
impl Plugin for CatenaPlugin {
    fn build(&self, app: &mut App) {

    }
}

/// The atomic unit of operations in Granite.
/// 
/// A [`Catena`] component is... something. Still need to figure out what that is.
pub struct Catena;

/// Test observer to get a feel for what's needed.
pub(crate) fn observer_moves_cardinal_territory_border_on<E: Debug + Clone + Reflect> (

) -> impl Fn(Trigger<E>) {
    move |trigger: Trigger<E>| {

    }
}