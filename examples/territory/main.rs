//! Starter test box for manipulating territories with atomic ops.

// Lints incompatible with Bevy
#![allow(
    clippy::needless_pass_by_value,
    reason = "Incompatible with Bevy's dependency injection techniques."
)]
#![allow(
    clippy::type_complexity,
    reason = "Incompatible with Bevy's ECS Query<> idioms."
)]

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
    window::{PresentMode, WindowCreated},
};
use bevy_granite::prelude::{GranitePlugin, GraniteRoot, LocalOrdonnanceStratumSpawned, SpawnTerritory};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: Level::INFO,
        filter: "wgpu=warn".to_string(),
        custom_layer: |_| None,
    }))
    .add_plugins(RemotePlugin::default())
    .add_plugins(RemoteHttpPlugin::default())
    .add_plugins(GranitePlugin)
    .add_systems(Startup, setup_granite_root)
    .add_systems(Update, setup_new_windows.run_if(on_event::<WindowCreated>))

    .add_observer(observer_spawns_test_territories);

    app.run();
}
/// Set up example root
fn setup_granite_root(mut commands: Commands) {
    let example_cam_one = commands.spawn((Camera2d, IsDefaultUiCamera)).id();

    commands.spawn((GraniteRoot, TargetCamera(example_cam_one)));
}

/// Set up window, try to make macOS not flip out at buttons.
fn setup_new_windows(mut window_query: Query<&mut Window>) {
    for mut window in &mut window_query {
        window.name = Some("Granite Window".to_string());
        window.present_mode = PresentMode::AutoNoVsync;
    }
}

/// Observer to spawn some test territories to play with.
fn observer_spawns_test_territories(
    _trigger: Trigger<LocalOrdonnanceStratumSpawned>,
    mut commands: Commands
) {
    let rect_top_left = Rect::from_corners(Vec2 { x: 0.2, y: 0.2 }, Vec2 { x: 0.4, y: 0.4 });
    let rect_top_right = Rect::from_corners(Vec2 { x: 0.6, y: 0.2 }, Vec2 { x: 0.8, y: 0.4 });
    let rect_bottom_right = Rect::from_corners(Vec2 { x: 0.6, y: 0.6 }, Vec2 { x: 0.8, y: 0.8 });
    let rect_bottom_left = Rect::from_corners(Vec2 { x: 0.2, y: 0.6 }, Vec2 { x: 0.4, y: 0.8 });
    commands.trigger(SpawnTerritory::new(rect_top_left));
    commands.trigger(SpawnTerritory::new(rect_top_right));
    commands.trigger(SpawnTerritory::new(rect_bottom_right));
    commands.trigger(SpawnTerritory::new(rect_bottom_left));
}