//! Starter test box for setting up networking capabilties.

use bevy::{prelude::*, remote::{http::RemoteHttpPlugin, RemotePlugin}, window::{PresentMode, WindowCreated}};
use bevy_granite::{prelude::GraniteRoot, GranitePlugin};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(GranitePlugin)

        .add_systems(Startup, setup_granite_root)

        .add_systems(Update, (setup_new_windows).run_if(on_event::<WindowCreated>));

    app.run();

}

/// Set up example root
fn setup_granite_root(mut commands: Commands) {

    let example_cam_one = commands.spawn(Camera2d).id();

    commands.spawn((
        GraniteRoot,
        TargetCamera(example_cam_one)
    ));
}

/// Set up window, try to mitigate frame delay.
fn setup_new_windows(
    mut window_query: Query<&mut Window>
) {
    for mut window in &mut window_query {
        window.name = Some("Granite Window".to_string());
        window.present_mode = PresentMode::AutoNoVsync;
    }
}