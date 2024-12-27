//! Starter test box for setting up networking capabilties.

use bevy::{prelude::*, remote::{http::RemoteHttpPlugin, RemotePlugin}};
use bevy_granite::{workbench::structure::GraniteRoot, GranitePlugin};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(GranitePlugin)

        .add_systems(Startup, setup_granite_root);

    app.run();

}

fn setup_granite_root(mut commands: Commands) {
    let example_cam_one = commands.spawn(Camera2d).id();
    commands.spawn((
        GraniteRoot,
        TargetCamera(example_cam_one)
    ));
}