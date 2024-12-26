//! Starter test box for setting up networking capabilties.

use bevy::{app::App, DefaultPlugins};
use bevy_granite::GranitePlugin;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(GranitePlugin);

    app.run();

}