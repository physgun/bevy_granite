//! Starter test box for setting up networking capabilties.

// Lints incompatible with Bevy
#![allow(clippy::needless_pass_by_value, reason = "Incompatible with Bevy's dependency injection techniques.")]
#![allow(clippy::type_complexity, reason = "Incompatible with Bevy's ECS Query<> idioms.")]

use core::fmt::Debug;

use bevy::{prelude::*, remote::{http::RemoteHttpPlugin, RemotePlugin}, window::{PresentMode, WindowCreated}};
use bevy_granite::{prelude::{GraniteRoot, OrdonnanceStratum}, GranitePlugin};
use lightyear::prelude::{server::{NetworkingState, ServerCommands}, Replicated};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(GranitePlugin)

        .add_systems(Startup, 
            (setup_granite_root, setup_local_connection_box).chain()
        )

        .add_systems(Update, (setup_new_windows).run_if(on_event::<WindowCreated>));

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
const BUTTON_BG_DISABLE: Color = Color::linear_rgba(0.4, 0.4, 0.4, 0.25);

/// Color of text by default.
const TEXT_DEFAULT: Color = Color::linear_rgba(0.7, 0.8, 0.9, 1.0);

/// Color of faded text when button is disabled.
const TEXT_DISABLE: Color = Color::linear_rgba(0.7, 0.8, 0.9, 0.25);

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

/// Spawn local app's connection status box debug UI
fn setup_local_connection_box(
    mut commands: Commands,
    local_ordonnance_stratum_query: Query<Entity, (With<OrdonnanceStratum>, Without<Replicated>)>
) {
    let Ok(ord_stratum_entity) = local_ordonnance_stratum_query.get_single() else {
        error!("[Basic Network Example] Single Local Ordonnance Stratum not found!");
        return;
    };

    commands.entity(ord_stratum_entity).with_children(|first_builder| {
        first_builder.spawn(info_box_node())
            .with_children(|second_builder| {
                second_builder.spawn(quick_button_node())
                    .observe(observer_changes_bg_on::<Pointer<Over>>(BUTTON_BG_HOVER))
                    .observe(observer_changes_bg_on::<Pointer<Out>>(BUTTON_BG_IDLE))
                    .observe(observer_changes_bg_on::<Pointer<Down>>(BUTTON_BG_CLICK))
                    .observe(observer_changes_bg_on::<Pointer<Up>>(BUTTON_BG_IDLE))
                    .observe(observer_toggles_server_button_on::<Pointer<Click>>())
                    .with_child(quick_text_node(String::from("Host Local Server")));
                second_builder.spawn(quick_button_node())
                    .observe(observer_changes_bg_on::<Pointer<Over>>(BUTTON_BG_HOVER))
                    .observe(observer_changes_bg_on::<Pointer<Out>>(BUTTON_BG_IDLE))
                    .observe(observer_changes_bg_on::<Pointer<Down>>(BUTTON_BG_CLICK))
                    .observe(observer_changes_bg_on::<Pointer<Up>>(BUTTON_BG_IDLE))
                    .with_child(quick_text_node(String::from("Connect To Local Server")));
            });
    });
}

/// Quick function to get the background of an info box to put everything in.
fn info_box_node() -> impl Bundle {
    (
        Node{
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
        BackgroundColor(INFO_BOX_BG)
    )
}

/// Get a basic button, text not included.
fn quick_button_node() -> impl Bundle {
    (
        Button, 
        Node{
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(5.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)), 
        BorderColor(BUTTON_BORDER_IDLE),
        BackgroundColor(BUTTON_BG_IDLE)
    )
}

/// Some text, on demand!
fn quick_text_node(text: String) -> impl Bundle {
    (
        Text::new(text), 
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont::from_font_size(12.0),
        TextColor(TEXT_DEFAULT)
    )
}

/// Generic observer that changes the background color.
fn observer_changes_bg_on <E: Debug + Clone + Reflect> (color: Color) -> impl Fn(Trigger<E>, Query<&mut BackgroundColor>) {
    move |trigger, mut bg_color_query| {
        let Ok(mut bg_color) = bg_color_query.get_mut(trigger.entity()) else {
            warn!("Could not get the background color of entity that triggered observer_changes_bg_on!");
            return;
        };
    
        bg_color.0 = color;
    }
}

/// Observer to enable and disable a button when something happens.
fn placeholder () {

}


/// Generic observer to Start and Stop the server when something happens to the "server now" button.
fn observer_toggles_server_button_on <E: Debug + Clone + Reflect> () -> impl Fn(Trigger<E>, Commands, Res<State<NetworkingState>>, Query<(&Parent, &mut Text)>) {
    move |trigger, mut commands, server_state, mut text_parent_query: Query<(&Parent, &mut Text)>| {
        
        for (parent_entity, mut button_text) in &mut text_parent_query {
            if parent_entity.get() ==  trigger.entity() {
                match *server_state.get() {
                    NetworkingState::Stopped => {
                        commands.start_server();
                        button_text.0 = String::from("[Hosting Local Server]");
                    }
                    NetworkingState::Started => {
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