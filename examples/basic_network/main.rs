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
use core::net::Ipv4Addr;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
    window::{PresentMode, WindowCreated},
};
use bevy_granite::prelude::{GranitePlugin, GraniteRoot, OrdonnanceStratum, LocalNetworkState, get_hostserver_config, get_local_client_config, get_netcode_client_config};
use lightyear::{client::config::ClientConfig, server::config::ServerConfig};
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
    .add_systems(
        Startup,
        (setup_granite_root, setup_local_connection_box).chain(),
    )
    .add_systems(
        Update,
        setup_new_windows.run_if(on_event::<WindowCreated>),
    )
    .add_systems(OnEnter(LocalNetworkState::Offline), setup_offline_buttons)
    .add_systems(OnEnter(LocalNetworkState::HostServer), setup_hostserver_buttons)
    .add_systems(OnEnter(LocalNetworkState::Server), setup_server_buttons)
    .add_systems(OnEnter(LocalNetworkState::Client), setup_client_buttons)
    
    .add_observer(observer_adds_observers_to_button);

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
    mut next_state: ResMut<NextState<LocalNetworkState>>,
    local_ordonnance_stratum_query: Query<Entity, (With<OrdonnanceStratum>, Without<Replicated>)>,
) {
    let Ok(ord_stratum_entity) = local_ordonnance_stratum_query.get_single() else {
        error!("[Basic Network Example] Single Local Ordonnance Stratum not found!");
        return;
    };

    commands
        .entity(ord_stratum_entity)
        .with_children(|first_level_builder| {
            first_level_builder
                .spawn(info_box_node())
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_cycles_to_previous_network_state_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("<")));
                    second_level_builder
                        .spawn(button_box_node())
                        .insert(NetworkModeButtonBox);
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_cycles_to_next_network_state_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from(">")));
                        
                });
        });

    // Clean slate for further setup logic.
    next_state.set(LocalNetworkState::HostServer);
}

/// Set up the `Offline` mode "buttons" when entering [`LocalNetworkState::Offline`].
fn setup_offline_buttons(
    mut commands: Commands,
    button_box_query: Query<Entity, With<NetworkModeButtonBox>>
) {
    commands.disconnect_client();
    commands.stop_server();

    for button_box_entity in &button_box_query {
        commands.entity(button_box_entity).with_children(|first_level_builder| {
            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Offline))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_text_node(String::from("[OFFLINE]")));
                });
        }); 
    }   
}

/// Set up the `HostServer` mode buttons when entering [`LocalNetworkState::HostServer`].
fn setup_hostserver_buttons(
    mut commands: Commands,
    mut lightyear_server_configs: ResMut<ServerConfig>,
    mut lightyear_client_configs: ResMut<ClientConfig>,
    button_box_query: Query<Entity, With<NetworkModeButtonBox>>
) {
    commands.disconnect_client();
    commands.stop_server();

    *lightyear_server_configs = get_hostserver_config(Ipv4Addr::UNSPECIFIED, 7142);
    *lightyear_client_configs = get_local_client_config(0 );

    for button_box_entity in &button_box_query {
        commands.entity(button_box_entity).with_children(|first_level_builder| {
            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::HostServer))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_text_node(String::from("[HOSTSERVER MODE]")));
                });

            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::HostServer))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_starts_server_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("Host Server")));
                });

            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::HostServer))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_stops_server_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("Shut Down Server")));
                });
        }); 
    }
}

/// Set up the `Server` mode buttons when entering [`LocalNetworkState::Server`].
fn setup_server_buttons(
    mut commands: Commands,
    mut lightyear_server_configs: ResMut<ServerConfig>,
    button_box_query: Query<Entity, With<NetworkModeButtonBox>>
) {
    commands.disconnect_client();
    commands.stop_server();

    *lightyear_server_configs = get_hostserver_config(Ipv4Addr::UNSPECIFIED, 7142);

    for button_box_entity in &button_box_query {
        commands.entity(button_box_entity).with_children(|first_level_builder| {
            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Server))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_text_node(String::from("[SERVER MODE]")));
                });

            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Server))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_starts_server_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("Start Server")));
                });

            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Server))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_stops_server_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("Stop Server")));
                });
        }); 
    }
}

/// Set up the `Client` mode buttons when entering [`LocalNetworkState::Client`].
fn setup_client_buttons(
    mut commands: Commands,
    mut lightyear_client_configs: ResMut<ClientConfig>,
    button_box_query: Query<Entity, With<NetworkModeButtonBox>>
) {
    commands.disconnect_client();
    commands.stop_server();

    *lightyear_client_configs = get_netcode_client_config(
        Ipv4Addr::UNSPECIFIED, 
        7142, 
        1, 
        Ipv4Addr::LOCALHOST, 
        7142
    );

    for button_box_entity in &button_box_query {
        commands.entity(button_box_entity).with_children(|first_level_builder| {
            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Client))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_text_node(String::from("[CLIENT MODE]")));
                });

            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Client))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_connects_client_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("Connect")));
                });

            first_level_builder
                .spawn(sub_row_button_node())
                .insert(StateScoped(LocalNetworkState::Client))
                .with_children(|second_level_builder| {
                    second_level_builder
                        .spawn(quick_button_node())
                        .observe(observer_disconnects_client_on::<Pointer<Click>>())
                        .with_child(quick_text_node(String::from("Disconnect")));
                });
        }); 
    }
    
}

/// Quick function to get the background of an info box to put everything in.
fn info_box_node() -> impl Bundle {
    (
        Name::new("Main Info Box Node"),
        Node {
            width: Val::Px(225.0),
            height: Val::Px(125.0),
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::all(Val::Px(2.5)),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(5.0),
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(INFO_BOX_BG),
    )
}

/// Quick function to get the background to put buttons in.
fn button_box_node() -> impl Bundle {
    (
        Name::new("Bottom Box Node"),
        Node {
            width: Val::Percent(80.0),
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::all(Val::Px(2.5)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(2.5),
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(INFO_BOX_BG),
    )
}

/// Quick function to get the background to put buttons in.
fn sub_row_button_node() -> impl Bundle {
    (
        Name::new("Row Background Node"),
        Node {
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::all(Val::Px(2.5)),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(5.0),
            column_gap: Val::Px(2.5),
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(INFO_BOX_BG),
    )
}

/// Get a basic button, text not included.
fn quick_button_node() -> impl Bundle {
    (
        Name::new("Button Node"),
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
        Name::new("Text Node"),
        Text::new(text),
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont::from_font_size(12.0),
        TextColor(TEXT_DEFAULT),
        PickingBehavior::IGNORE,
    )
}



/// Global observer to add other observers to a freshly spawned `Button` entity.
fn observer_adds_observers_to_button(
    trigger: Trigger<OnAdd, Button>,
    mut commands: Commands
) {
    commands
        .entity(trigger.entity())
        .observe(observer_changes_bg_on::<Pointer<Over>>(BUTTON_BG_HOVER))
        .observe(observer_changes_bg_on::<Pointer<Move>>(BUTTON_BG_HOVER))
        .observe(observer_changes_bg_on::<Pointer<Out>>(BUTTON_BG_IDLE))
        .observe(observer_changes_bg_on::<Pointer<Down>>(BUTTON_BG_CLICK))
        .observe(observer_changes_bg_on::<Pointer<Up>>(BUTTON_BG_HOVER));
}

/// Generic observer that changes the background color.
fn observer_changes_bg_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&mut BackgroundColor>) {
    move |trigger, mut bg_color_query| {
        let Ok(mut bg_color) = bg_color_query.get_mut(trigger.entity()) else {
            warn!("Could not get the background color of entity that triggered observer_changes_bg_on!");
            return;
        };
        bg_color.0 = color;
    }
}

/// Generic observer cycles to next [`LocalNetworkState`].
fn observer_cycles_to_next_network_state_on<E: Debug + Clone + Reflect>(

) -> impl Fn(Trigger<E>, Res<State<LocalNetworkState>>, ResMut<NextState<LocalNetworkState>>) {
    move |_trigger, current_state, mut next_state| {
        match **current_state {
            LocalNetworkState::Offline => {next_state.set(LocalNetworkState::HostServer);}
            LocalNetworkState::HostServer => {next_state.set(LocalNetworkState::Server);}
            LocalNetworkState::Server => {next_state.set(LocalNetworkState::Client);}
            LocalNetworkState::Client => {next_state.set(LocalNetworkState::Offline);}
            _ => {error!("[NETWORK] Unexpected match wildcard for LocalNetworkState!");}
        }
    }
}

/// Generic observer cycles to previous [`LocalNetworkState`].
fn observer_cycles_to_previous_network_state_on<E: Debug + Clone + Reflect>(

) -> impl Fn(Trigger<E>, Res<State<LocalNetworkState>>, ResMut<NextState<LocalNetworkState>>) {
    move |_trigger, current_state, mut next_state| {
        match **current_state {
            LocalNetworkState::Offline => {next_state.set(LocalNetworkState::Client);}
            LocalNetworkState::Client => {next_state.set(LocalNetworkState::Server);}
            LocalNetworkState::Server => {next_state.set(LocalNetworkState::HostServer);}
            LocalNetworkState::HostServer => {next_state.set(LocalNetworkState::Offline);}
            _ => {error!("[NETWORK] Unexpected match wildcard for LocalNetworkState!");}
        }
    }
}

/// Generic observer starts the lightyear server.
fn observer_starts_server_on<E: Debug + Clone + Reflect>(

) -> impl Fn(Trigger<E>, Commands) {
    move |_trigger, mut commands| {
        commands.start_server();
    }
}

/// Generic observer stops the lightyear server.
fn observer_stops_server_on<E: Debug + Clone + Reflect>(

) -> impl Fn(Trigger<E>, Commands) {
    move |_trigger, mut commands| {
        commands.stop_server();
    }
}

/// Generic observer connects the lightyear client.
fn observer_connects_client_on<E: Debug + Clone + Reflect>(

) -> impl Fn(Trigger<E>, Commands) {
    move |_trigger, mut commands| {
        commands.connect_client();
    }
}

/// Generic observer disconnects the lightyear client.
fn observer_disconnects_client_on<E: Debug + Clone + Reflect>(

) -> impl Fn(Trigger<E>, Commands) {
    move |_trigger, mut commands| {
        commands.disconnect_client();
    }
}

/// Marker component for the main local user button box.
#[derive(Component, Clone, Copy, Debug, Reflect)]
struct NetworkModeButtonBox;