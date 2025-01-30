//! Starter test box for setting up networking capabilties.

// Lints incompatible with Bevy
#![allow(
    clippy::needless_pass_by_value,
    reason = "Incompatible with Bevy's dependency injection techniques."
)]
#![allow(
    clippy::type_complexity,
    reason = "Incompatible with Bevy's ECS Query<> idioms."
)]

use core::fmt::Debug;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
    window::{PresentMode, WindowCreated},
};
use bevy_granite::prelude::{GranitePlugin, GraniteRoot, OrdonnanceStratum};
use lightyear::prelude::client::NetworkingState as ClientNetworkingState;
use lightyear::prelude::server::NetworkingState as ServerNetworkingState;
use lightyear::prelude::{client::ClientCommands, server::ServerCommands, Replicated};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: Level::INFO,
        filter: "wgpu=warn,lightyear=debug".to_string(),
        custom_layer: |_| None,
    }))
    .add_plugins(RemotePlugin::default())
    .add_plugins(RemoteHttpPlugin::default())
    .add_plugins(GranitePlugin)
    .add_event::<ButtonModified>()
    .add_systems(
        Startup,
        (setup_granite_root, setup_local_connection_box).chain(),
    )
    .add_systems(
        Update,
        (setup_new_windows).run_if(on_event::<WindowCreated>),
    );

    app.run();
}

/// Background color for the connection info boxes.
const INFO_BOX_BG: Color = Color::linear_rgba(0.1, 0.1, 0.1, 1.0);

/// Background color of a regular ol' button, just sitting around.
const BUTTON_BG_IDLE: Color = Color::linear_rgba(0.2, 0.2, 0.2, 1.0);

/// Background color of a button that's being hovered over.
const BUTTON_BG_HOVER: Color = Color::linear_rgba(0.6, 0.6, 0.6, 1.0);

/// Background color of a button being clicked.
const BUTTON_BG_CLICK: Color = Color::linear_rgba(0.3, 0.8, 0.9, 1.0);

/// Border color of button doing nothing.
const BUTTON_BORDER_IDLE: Color = Color::linear_rgba(0.3, 0.3, 0.3, 1.0);

/// Background color of a diabled button.
const BUTTON_BG_DISABLE: Color = Color::linear_rgba(1.0, 0.8, 0.8, 0.75);

/// Color of text by default.
const TEXT_DEFAULT: Color = Color::linear_rgba(0.7, 0.8, 0.9, 1.0);

/// Set up example root
fn setup_granite_root(mut commands: Commands) {
    let example_cam_one = commands.spawn(Camera2d).id();

    commands.spawn((GraniteRoot, TargetCamera(example_cam_one)));
}

/// Set up window, try to mitigate frame delay.
fn setup_new_windows(mut window_query: Query<&mut Window>) {
    for mut window in &mut window_query {
        window.name = Some("Granite Window".to_string());
        window.present_mode = PresentMode::AutoNoVsync;
    }
}

/// Spawn local app's connection status box debug UI
fn setup_local_connection_box(
    mut commands: Commands,
    local_ordonnance_stratum_query: Query<Entity, (With<OrdonnanceStratum>, Without<Replicated>)>,
) {
    let Ok(ord_stratum_entity) = local_ordonnance_stratum_query.get_single() else {
        error!("[Basic Network Example] Single Local Ordonnance Stratum not found!");
        return;
    };

    commands
        .entity(ord_stratum_entity)
        .with_children(|first_builder| {
            first_builder
                .spawn(info_box_node())
                .with_children(|second_builder| {
                    second_builder.spawn(quick_text_node(String::from("Local User")));
                    second_builder
                        .spawn(quick_button_node())
                        .insert(ServerFunction)
                        .observe(observer_changes_bg_on::<Pointer<Over>>(BUTTON_BG_HOVER))
                        .observe(observer_changes_bg_on::<Pointer<Out>>(BUTTON_BG_IDLE))
                        .observe(observer_changes_bg_on::<Pointer<Down>>(BUTTON_BG_CLICK))
                        .observe(observer_changes_bg_on::<Pointer<Up>>(BUTTON_BG_IDLE))
                        .observe(observer_changes_bg_on::<ButtonModified>(BUTTON_BG_IDLE))
                        .observe(observer_toggles_server_button_on::<Pointer<Click>>())
                        //.observe(observer_sets_button_disable_on::<WasDisconnected>(
                        //    Modifier::Enabled,
                        //))
                        //.observe(observer_sets_button_disable_on::<StartedConnecting>(
                        //    Modifier::Disabled,
                        //))
                        .with_child(quick_text_node(String::from("Host Local Server")));
                    second_builder
                        .spawn(quick_button_node())
                        .insert(ClientFunction)
                        .observe(observer_changes_bg_on::<Pointer<Over>>(BUTTON_BG_HOVER))
                        .observe(observer_changes_bg_on::<Pointer<Out>>(BUTTON_BG_IDLE))
                        .observe(observer_changes_bg_on::<Pointer<Down>>(BUTTON_BG_CLICK))
                        .observe(observer_changes_bg_on::<Pointer<Up>>(BUTTON_BG_IDLE))
                        .observe(observer_changes_bg_on::<ButtonModified>(BUTTON_BG_IDLE))
                        .observe(observer_toggles_client_button_on::<Pointer<Click>>())
                        //.observe(observer_sets_button_disable_on::<StoppedHostServer>(
                        //    Modifier::Enabled,
                        //))
                        //.observe(observer_sets_button_disable_on::<StartedHostServer>(
                        //    Modifier::Disabled,
                        //))
                        .with_child(quick_text_node(String::from("Connect To Local Server")));
                });
        });
}

/// Quick function to get the background of an info box to put everything in.
fn info_box_node() -> impl Bundle {
    (
        Node {
            width: Val::Px(225.0),
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::all(Val::Px(2.5)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(5.0),
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(INFO_BOX_BG),
    )
}

/// Get a basic button, text not included.
fn quick_button_node() -> impl Bundle {
    (
        Button,
        Node {
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(5.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BorderColor(BUTTON_BORDER_IDLE),
        BackgroundColor(BUTTON_BG_IDLE),
    )
}

/// Some text, on demand!
fn quick_text_node(text: String) -> impl Bundle {
    (
        Text::new(text),
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont::from_font_size(12.0),
        TextColor(TEXT_DEFAULT),
        PickingBehavior::IGNORE,
    )
}

/// Generic observer to enable and disable a button when something happens, depending on the [`Modifier`] passed in.
fn observer_sets_button_disable_on<E: Debug + Clone + Reflect>(
    modifier: Modifier,
) -> impl Fn(Trigger<E>, Commands, Query<Option<&Disabled>, With<Button>>) {
    move |trigger, mut commands, disabled_query: Query<Option<&Disabled>, With<Button>>| {
        let Ok(disabled) = disabled_query.get(trigger.entity()) else {
            warn!("Could not return of query of Option?");
            return;
        };

        let mut button = commands.entity(trigger.entity());

        match (modifier, disabled.is_some()) {
            (Modifier::Enabled, true) => {
                button.remove::<Disabled>().trigger(ButtonModified);
            }
            (Modifier::Enabled, false) | (Modifier::Disabled, true) => {}
            (Modifier::Disabled, false) => {
                button.insert(Disabled).trigger(ButtonModified);
            }
        }
    }
}

/// Generic observer that changes the background color.
fn observer_changes_bg_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<(&mut BackgroundColor, Option<&Disabled>)>) {
    move |trigger, mut bg_color_query| {
        let Ok((mut bg_color, disabled)) = bg_color_query.get_mut(trigger.entity()) else {
            warn!("Could not get the background color of entity that triggered observer_changes_bg_on!");
            return;
        };

        if disabled.is_none() {
            bg_color.0 = color;
        } else {
            bg_color.0 = BUTTON_BG_DISABLE;
        }
    }
}

/// Generic observer to Start and Stop the server when something happens to the "server now" button.
fn observer_toggles_server_button_on<E: Debug + Clone + Reflect>() -> impl Fn(
    Trigger<E>,
    Commands,
    Res<State<ServerNetworkingState>>,
    Query<(&Parent, &mut Text)>,
    Query<Option<&Disabled>, (With<Button>, With<ServerFunction>)>,
    Query<Entity, (With<Button>, With<ClientFunction>)>,
) {
    move |trigger,
          mut commands,
          server_state,
          mut text_parent_query: Query<(&Parent, &mut Text)>,
          disabled_query: Query<Option<&Disabled>, (With<Button>, With<ServerFunction>)>,
          client_buttons_query: Query<Entity, (With<Button>, With<ClientFunction>)>| {
        let Ok(disabled) = disabled_query.get(trigger.entity()) else {
            warn!("Could not return of query of Option?");
            return;
        };

        if disabled.is_some() {
            return;
        }

        for (parent_entity, mut button_text) in &mut text_parent_query {
            if parent_entity.get() == trigger.entity() {
                match *server_state.get() {
                    ServerNetworkingState::Stopped | ServerNetworkingState::Stopping => {
                        commands.start_server();
                        button_text.0 = String::from("Shut Down Local Server");
                    }
                    ServerNetworkingState::Started | ServerNetworkingState::Starting => {
                        commands.stop_server();
                        button_text.0 = String::from("Host Local Server");
                    }
                }
                return;
            }
        }
        // If we got here, no matches were ever found.
        warn!("No text found on the server button!");
    }
}

/// Generic observer to connect and disconnnect from a server when something happens to the "connect now" button.
fn observer_toggles_client_button_on<E: Debug + Clone + Reflect>() -> impl Fn(
    Trigger<E>,
    Commands,
    Res<State<ClientNetworkingState>>,
    Query<(&Parent, &mut Text)>,
    Query<Option<&Disabled>, (With<Button>, With<ClientFunction>)>,
    Query<Entity, (With<Button>, With<ServerFunction>)>,
) {
    move |trigger,
          mut commands,
          server_state,
          mut text_parent_query: Query<(&Parent, &mut Text)>,
          disabled_query: Query<Option<&Disabled>, (With<Button>, With<ClientFunction>)>,
          server_buttons_query: Query<Entity, (With<Button>, With<ServerFunction>)>| {
        let Ok(disabled) = disabled_query.get(trigger.entity()) else {
            warn!("Could not return of query of Option?");
            return;
        };

        if disabled.is_some() {
            return;
        }

        for (parent_entity, mut button_text) in &mut text_parent_query {
            if parent_entity.get() == trigger.entity() {
                match *server_state.get() {
                    ClientNetworkingState::Disconnected => {
                        commands.connect_client();
                        button_text.0 = String::from("Disconnect From Local Server");
                    }
                    ClientNetworkingState::Connected | ClientNetworkingState::Connecting => {
                        commands.disconnect_client();
                        button_text.0 = String::from("Connect To Local Server");
                    }
                }
                return;
            }
        }
        // If we got here, no matches were ever found.
        warn!("No text found on the server button!");
    }
}

/// Marker component for a disabled button.
#[derive(Component, Clone, Copy, Debug, Reflect)]
struct Disabled;

/// Marker component for a button that does Client things.
#[derive(Component, Clone, Copy, Debug, Reflect)]
struct ClientFunction;

/// Marker component for a button that does Server things.
#[derive(Component, Clone, Copy, Debug, Reflect)]
struct ServerFunction;

/// Event emitted when a button is modifed.
#[derive(Event, Clone, Copy, Debug, Reflect)]
struct ButtonModified;

/// Enum to make generic disable observer more readable.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
enum Modifier {
    /// Entity is enabled.
    #[default]
    Enabled,
    /// Entity has been disabled.
    Disabled,
}
