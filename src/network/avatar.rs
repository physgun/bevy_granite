//! The network layer of Bevy Components representing the user, which are shared and replicated between hostservers.

use serde::{Serialize, Deserialize};
use bevy::{color::palettes::css, input::mouse::MouseMotion, math::Vec2, prelude::*, ui::{RelativeCursorPosition, UiSystem}};
use lightyear::prelude::{client::ComponentSyncMode, AppComponentExt, ChannelDirection, ClientId, Linear, Replicated};

use lightyear::prelude::client::Replicate as ClientReplicate;

use crate::workbench::structure::{LocalLociStratumSpawned, LociStratum};

/// Plugin the user profiles, customizable representation, and so on.
pub (crate) struct AvatarPlugin;
impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<Avatar>(ChannelDirection::Bidirectional);
        app.register_component::<LocusPosition>(ChannelDirection::Bidirectional)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();
        app.register_component::<LocusColor>(ChannelDirection::Bidirectional);
        app.register_component::<MouseLocus>(ChannelDirection::Bidirectional);
        app.register_component::<FocusLocus>(ChannelDirection::Bidirectional);
        app.register_component::<TouchprintLocus>(ChannelDirection::Bidirectional);
        app.register_component::<StylusLocus>(ChannelDirection::Bidirectional);

        app
            .add_observer(Self::observer_adds_loci_to_local_stratum)

            .add_systems(Startup, Self::spawn_local_avatar)

            // TODO: Order before network stuff gets sent out. Find frame delay cause!!
            .add_systems(PreUpdate, 
                (Self::detect_local_mouse_loci)
                    .after(UiSystem::Focus)
                    .run_if(on_event::<MouseMotion>)
            );
    }
}
impl AvatarPlugin {
    /// Spawn the user's Avatar, the parent of all of their stuff. Replicated to servers with local client having authority.
    fn spawn_local_avatar (
        mut commands: Commands,
        local_avatar_query: Query<&LocalAvatar>
    ) {
        if !local_avatar_query.is_empty() {
            error!("Local Avatar detected on startup before spawning one?!");
            return;
        }

        commands.spawn((
            Name::new("Local Avatar"),
            LocalAvatar,
            Avatar::new_local(714),
            ClientReplicate::default()
        ));
    }

    /// Observer for adding local locus child entities to a freshly spawned [`LociStratum`].
    /// TODO: How to not spawn for foreign loci stratum
    fn observer_adds_loci_to_local_stratum(
        trigger: Trigger<LocalLociStratumSpawned>,
        mut commands: Commands
    ) {

        let image_node_color = LocusColor::MATERIAL_LIGHT.tertiary_as_color();

        let mouse_locus_entity = commands.spawn((
            Name::new("Local Avatar Mouse Locus"),
            Visibility::Hidden,
            BackgroundColor(image_node_color),
            BorderRadius::new(Val::ZERO, Val::Px(2.5), Val::Px(7.5), Val::Px(2.5)),
            Node {
                position_type: PositionType::Absolute,
                height: Val::Px(7.5),
                width: Val::Px(7.5),
                ..Default::default()
            },
            ImageNode::solid_color(image_node_color),
            MouseLocus, 
            LocusPosition::default(),
            LocusColor::MATERIAL_LIGHT,
            ClientReplicate::default()
        )).id();

        commands.entity(trigger.entity()).add_child(mouse_locus_entity);
    }

    /// Updates all local mouse loci by checking if its parent `GraniteRoot`'s `RelativeCursorPosition` detected a mouse.
    fn detect_local_mouse_loci(
        mut commands: Commands,
        loci_stratum_query: Query<&RelativeCursorPosition, With<LociStratum>>,
        mut local_mouse_locus_query: Query<
        (Entity, &Parent ,&mut Node, &mut LocusPosition, &mut Visibility, Option<&LocusDetected>),
        (With<MouseLocus>, Without<Replicated>)>
    ) {
        for (
            mouse_locus_entity, 
            loci_stratum_parent, 
            mut mouse_locus_node,
            mut locus_position,
            mut node_visibility,
            locus_detected) in &mut local_mouse_locus_query {

            let Ok(parent_relcurpos) = loci_stratum_query.get(loci_stratum_parent.get()) else {
                error!("Mouse locus parent GraniteRoot entity not found with RelativeCursorPosition component!");
                continue;
            };

            let Some(detected_relative_position) = parent_relcurpos.normalized else {
                if let Visibility::Inherited = *node_visibility { node_visibility.toggle_inherited_hidden(); } 
                if locus_detected.is_some() { commands.entity(mouse_locus_entity).remove::<LocusDetected>(); }
                continue;
            };

            // Stick a copy in here, to replicate to others if needed.
            locus_position.set_pos(detected_relative_position);

            mouse_locus_node.left = Val::Percent(detected_relative_position.x * 100.0);
            mouse_locus_node.top = Val::Percent(detected_relative_position.y * 100.0);
            if let Visibility::Hidden =  *node_visibility { node_visibility.toggle_inherited_hidden(); }

            // Also needed for replication, as `Visibility` doesn't implement serde traits: 
            if locus_detected.is_none() { commands.entity(mouse_locus_entity).insert(LocusDetected); }
        }
    }
}

/// Marker component separating out entities that belong to the local user and are not replicated from a server.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
struct LocalAvatar;

/// Main component for the entity representation of a user. Parent of all entities the user controls.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct Avatar {
    /// The Lightyear ID assigned this user.
    lightyear_id: ClientId
}
impl Default for Avatar {
    fn default() -> Self {
        Avatar { lightyear_id: ClientId::Local(1) }
    }
}
impl Avatar {
    /// Create a new local profile from scratch.
    fn new_local(local_id: u64) -> Self {
        Avatar { lightyear_id: ClientId::Local(local_id) }
    }
}

/// `MouseLocus` is a standard cursor that has an `x, y` relative screenspace position.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
struct MouseLocus;

/// `FocusLocus` is a highlighted section or button, for gamepad navigation.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
struct FocusLocus;

/// `TouchprintLocus` is a finger on a touchscreen.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
struct TouchprintLocus;

/// `StylusLocus` is a pressure-sensitive `Mouse` that may have a trail.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Reflect)]
struct StylusLocus;

/// Unique color scheme of the user's locus.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub struct LocusColor {
    /// Dominant color of the user.
    primary: Srgba,
    /// Accessory color, less prominent than the primary.
    secondary: Srgba,
    /// Rare accent color.
    tertiary: Srgba
}
impl Default for LocusColor {
    fn default() -> Self {
        LocusColor { 
            primary: css::WHITE, 
            secondary: css::GOLD, 
            tertiary: css::AQUA 
        }
    }
}
impl LocusColor {
    /// Default Light
    const MATERIAL_LIGHT: LocusColor = LocusColor {
        primary: css::WHITE, 
        secondary: css::GOLD, 
        tertiary: css::AQUA 
    };

    /// Default Dark
    const MATERIAL_DARK: LocusColor = LocusColor {
        primary: css::BLACK, 
        secondary: css::GOLD, 
        tertiary: css::AQUA 
    };

    /// Custom new color.
    #[must_use = "Struct is initialized, but is never used!"]
    pub fn new(primary_color: Srgba, secondary_color: Srgba, tertiary_color: Srgba) -> Self {
        LocusColor { primary: primary_color, secondary: secondary_color, tertiary: tertiary_color }
    }

    /// Returns the primary color as the [`Color`] type.
    #[must_use = "You called for a Color, but never used it!"]
    pub fn primary_as_color(self) -> Color {
        self.primary.into()
    }

    /// Returns the secondary color as the [`Color`] type.
    #[must_use = "You called for a Color, but never used it!"]
    pub fn secondary_as_color(self) -> Color {
        self.secondary.into()
    }

    /// Returns the tertiary color as the [`Color`] type.
    #[must_use = "You called for a Color, but never used it!"]
    pub fn tertiary_as_color(self) -> Color {
        self.tertiary.into()
    }
}

/// In relative screenspace coordinates, from top-left `(0.0, 0.0)` to bottom right `(1.0, 1.0)`  
/// 
/// Here for the network replication, too difficult to send over the `ImageNode` positions.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub (crate) struct LocusPosition {
    /// Cheap xy to send over the network.
    pos: Vec2
}
impl Default for LocusPosition {
    fn default() -> Self {
        LocusPosition { pos: Vec2::new(0.5, 0.5) }
    }
}
impl Linear for LocusPosition{
    fn lerp(start: &Self, other: &Self, t: f32) -> Self {
        *start // We'll have to figure this out later.
    }
}
impl LocusPosition {
    /// Gets the position vector.
    fn pos(self) -> Vec2 {
        self.pos
    }

    /// Sets the position vector.
    fn set_pos(&mut self, new_pos: Vec2) -> &mut Self {
        self.pos = new_pos;
        self
    }
}
/// Marker component labeling the entity as having been detected, and will show up on rendering queries for that entity.  
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
pub (crate) struct LocusDetected;

