//! The primary element of the granite UI.
//! Territories are a more flexible and easy style of organization designed to address issues found during user-testing of other styles.
//! Replaces docking, splitting, and windows functionality.

use bevy::prelude::*;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use super::structure::{Cardinal, East, North, OrdonnanceStratum, South, West};

/// Contains definitions for the Workbench UI `Territory` element.
pub(crate) struct TerritoryPlugin;
impl Plugin for TerritoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Territory>();

        let world = app.main_mut().world_mut();

        world.spawn((
            Observer::new(observer_spawns_territories),
            Name::new("[OBSERVER] Territory Spawner"),
        ));
    }
}

/// Temp color of border at idle.
pub(crate) const BORDER_IDLE: Color = Color::Oklcha(Oklcha {
    lightness: 0.75,
    chroma: 0.15,
    hue: 225.0,
    alpha: 0.25,
});
/// Temp color of border when hovered.
pub(crate) const BORDER_HOVER: Color = Color::Oklcha(Oklcha {
    lightness: 0.75,
    chroma: 0.15,
    hue: 225.0,
    alpha: 0.75,
});
/// Temp color of border when clicked.
pub(crate) const BORDER_CLICK: Color = Color::Oklcha(Oklcha {
    lightness: 0.80,
    chroma: 0.14,
    hue: 211.0,
    alpha: 1.0,
});

/// Driving component for the Territory UI entity.
/// It's the big, movable, draggable, resizable box that contains the tab bars.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Node, BackgroundColor, BorderColor, BorderRadius)]
pub struct Territory;

/// Event to trigger an observer that handles spawning a [`Territory`].
#[derive(Event, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct SpawnTerritory {
    /// A [`Rect`] describing the size and location of the [`Territory`] to be spawned.
    ///
    /// Units will be applied in a proportional way. `50.0` will mean `50.0%` of the parent's size.
    rect: Rect,
}
impl SpawnTerritory {
    /// Gets the [`Rect`] describing the size of the [`Territory`] to be spawned.
    pub(crate) fn rect(&self) -> Rect {
        self.rect
    }

    /// Initializes a new [`SpawnTerritory`].
    #[must_use = "You called for a new SpawnTerritory, but never used it!"]
    pub fn new(rect: Rect) -> SpawnTerritory {
        SpawnTerritory { rect }
    }
}

/// Event to trigger an observer that responds to a newly spawned [`Territory`].
#[derive(Event, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct TerritorySpawned;

/// Driving component for a resizable UI element's border entity.
///
/// Borders of a UI entity are their own entities themselves, for smoother mutable querying.
#[derive(Component, Clone, Debug, Default, PartialEq, Reflect)]
#[reflect(Component)]
#[require(Node, BackgroundColor, BorderColor, BorderRadius)]
pub(crate) struct Border<D: Cardinal> {
    /// Which direction this border faces. Looks nicer than `PhantomData`.
    direction: D,
    /// Location of the border.
    pos: f32,
    /// The smaller, starting coordinate of the border.
    start_point: f32,
    /// The larger, ending coordinate of the border.
    end_point: f32,
    /// The smallest size the border can be.
    minimum_size: f32,
}
impl Border<North> {
    /// A new northern border with fields generated from the given [`Rect`].
    fn from_rect(rect: Rect) -> Border<North> {
        let pos = rect.min.y;
        let start_point = rect.min.x;
        let end_point = rect.max.x;
        let minimum_size = 25.0;

        Border::<North> {
            direction: North,
            pos,
            start_point,
            end_point,
            minimum_size,
        }
    }
}
impl Border<East> {
    /// A new eastern border with fields generated from the given [`Rect`].
    fn from_rect(rect: Rect) -> Border<East> {
        let pos = rect.max.x;
        let start_point = rect.min.y;
        let end_point = rect.max.y;
        let minimum_size = 25.0;

        Border::<East> {
            direction: East,
            pos,
            start_point,
            end_point,
            minimum_size,
        }
    }
}
impl Border<South> {
    /// A new southern border with fields generated from the given [`Rect`].
    fn from_rect(rect: Rect) -> Border<South> {
        let pos = rect.max.y;
        let start_point = rect.min.x;
        let end_point = rect.max.x;
        let minimum_size = 25.0;

        Border::<South> {
            direction: South,
            pos,
            start_point,
            end_point,
            minimum_size,
        }
    }
}
impl Border<West> {
    /// A new western border with fields generated from the given [`Rect`].
    fn from_rect(rect: Rect) -> Border<West> {
        let pos = rect.min.x;
        let start_point = rect.min.y;
        let end_point = rect.max.y;
        let minimum_size = 25.0;

        Border::<West> {
            direction: West,
            pos,
            start_point,
            end_point,
            minimum_size,
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "This function will be split up when catenae are developed."
)]
/// Global observer that spawns a [`Territory`] with the [`SpawnTerritory`] event.
fn observer_spawns_territories(
    trigger: Trigger<SpawnTerritory>,
    mut commands: Commands,
    ordonnance_query: Query<Entity, With<OrdonnanceStratum>>,
) {
    let Ok(ordonnance_layer_entity) = ordonnance_query.get_single() else {
        warn!("[TERRITORY SPAWN] Ordonnance query did not have single!");
        return;
    };

    let spawn_rect = trigger.event().rect();

    let territory_entity = commands
        .spawn((
            Territory,
            Name::new("Territory"),
            Node {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::px(5.0),
                    GridTrack::px(10.0),
                    GridTrack::flex(1.0),
                    GridTrack::px(10.0),
                    GridTrack::px(5.0),
                ],
                grid_template_rows: vec![
                    GridTrack::px(5.0),
                    GridTrack::px(10.0),
                    GridTrack::flex(1.0),
                    GridTrack::px(10.0),
                    GridTrack::px(5.0),
                ],
                position_type: PositionType::Absolute,
                top: Val::Percent(spawn_rect.min.y * 100.0),
                left: Val::Percent(spawn_rect.min.x * 100.0),
                bottom: Val::Percent(spawn_rect.max.y * 100.0),
                right: Val::Percent(spawn_rect.max.x * 100.0),
                width: Val::Percent(spawn_rect.width() * 100.0),
                height: Val::Percent(spawn_rect.height() * 100.0),
                ..default()
            },
            BackgroundColor(Color::Oklcha(Oklcha {
                lightness: 0.75,
                chroma: 0.15,
                hue: 225.0,
                alpha: 0.05,
            })),
            BorderRadius::all(Val::Px(2.5)),
        ))
        .with_children(|first_layer_builder| {
            first_layer_builder
                .spawn((
                    Border::<North>::from_rect(spawn_rect),
                    Name::new("North Border"),
                    Button,
                    Node {
                        display: Display::Grid,
                        grid_row: GridPlacement::start(1),
                        grid_column: GridPlacement::start_end(1, 6),
                        ..default()
                    },
                    BackgroundColor(BORDER_IDLE),
                    BorderColor(BORDER_CLICK),
                    BorderRadius::all(Val::Px(2.5)),
                    PickingBehavior{should_block_lower: false, is_hoverable: true},
                ))
                .observe(observer_changes_bg_on::<Pointer<Over>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Move>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Out>>(BORDER_IDLE))
                .observe(observer_changes_bg_on::<Pointer<Down>>(BORDER_CLICK))
                .observe(observer_changes_bg_on::<Pointer<Up>>(BORDER_HOVER));
            first_layer_builder
                .spawn((
                    Border::<East>::from_rect(spawn_rect),
                    Name::new("East Border"),
                    Button,
                    Node {
                        display: Display::Grid,
                        grid_row: GridPlacement::start_end(1, 6),
                        grid_column: GridPlacement::start(5),
                        ..default()
                    },
                    BackgroundColor(BORDER_IDLE),
                    BorderColor(BORDER_CLICK),
                    BorderRadius::all(Val::Px(2.5)),
                    PickingBehavior{should_block_lower: false, is_hoverable: true},
                ))
                .observe(observer_changes_bg_on::<Pointer<Over>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Move>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Out>>(BORDER_IDLE))
                .observe(observer_changes_bg_on::<Pointer<Down>>(BORDER_CLICK))
                .observe(observer_changes_bg_on::<Pointer<Up>>(BORDER_HOVER));
            first_layer_builder
                .spawn((
                    Border::<South>::from_rect(spawn_rect),
                    Name::new("South Border"),
                    Button,
                    Node {
                        display: Display::Grid,
                        grid_row: GridPlacement::start(5),
                        grid_column: GridPlacement::start_end(1, 6),
                        ..default()
                    },
                    BackgroundColor(BORDER_IDLE),
                    BorderColor(BORDER_CLICK),
                    BorderRadius::all(Val::Px(2.5)),
                    PickingBehavior{should_block_lower: false, is_hoverable: true},
                ))
                .observe(observer_changes_bg_on::<Pointer<Over>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Move>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Out>>(BORDER_IDLE))
                .observe(observer_changes_bg_on::<Pointer<Down>>(BORDER_CLICK))
                .observe(observer_changes_bg_on::<Pointer<Up>>(BORDER_HOVER));
            first_layer_builder
                .spawn((
                    Border::<West>::from_rect(spawn_rect),
                    Name::new("West Border"),
                    Button,
                    Node {
                        display: Display::Grid,
                        grid_row: GridPlacement::start_end(1, 6),
                        grid_column: GridPlacement::start(1),
                        ..default()
                    },
                    BackgroundColor(BORDER_IDLE),
                    BorderColor(BORDER_CLICK),
                    BorderRadius::all(Val::Px(2.5)),
                    PickingBehavior{should_block_lower: false, is_hoverable: true},
                ))
                .observe(observer_changes_bg_on::<Pointer<Over>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Move>>(BORDER_HOVER))
                .observe(observer_changes_bg_on::<Pointer<Out>>(BORDER_IDLE))
                .observe(observer_changes_bg_on::<Pointer<Down>>(BORDER_CLICK))
                .observe(observer_changes_bg_on::<Pointer<Up>>(BORDER_HOVER));
        })
        .id();

    commands
        .entity(ordonnance_layer_entity)
        .add_child(territory_entity);
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
