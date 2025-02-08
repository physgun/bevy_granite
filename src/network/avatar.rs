//! The network layer of Bevy Components representing the user, which are shared and replicated between hostservers.

use core::ops::{Add, Mul};

use bevy::{
    input::mouse::MouseMotion,
    math::Vec2,
    prelude::*,
    ui::{RelativeCursorPosition, UiSystem},
};
use lightyear::{
    prelude::{
        server::{AuthorityPeer, ControlledBy, ReplicationTarget},
        AppComponentExt, ChannelDirection, ClientId, Linear, NetworkTarget, Replicated,
        ServerComponentUpdateEvent, ServerReplicate,
    },
    shared::replication::delta::Diffable,
};
use serde::{Deserialize, Serialize};

use lightyear::prelude::client::Replicate as ClientReplicate;

use crate::workbench::structure::{LocalLociStratumSpawned, LociStratum};

/// Plugin the user profiles, customizable representation, and so on.
pub(crate) struct AvatarPlugin;
impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<Name>(ChannelDirection::Bidirectional);
        app.register_component::<Avatar>(ChannelDirection::Bidirectional);
        app.register_component::<LocusPosition>(ChannelDirection::Bidirectional);
        //.add_interpolation(ComponentSyncMode::Full);
        //.add_linear_interpolation_fn();
        app.register_component::<LocusDetected>(ChannelDirection::Bidirectional);
        app.register_component::<LocusColor>(ChannelDirection::Bidirectional);
        app.register_component::<BackgroundColor>(ChannelDirection::Bidirectional);
        app.register_component::<BorderColor>(ChannelDirection::Bidirectional);
        app.register_component::<BorderRadius>(ChannelDirection::Bidirectional);
        app.register_component::<MouseLocus>(ChannelDirection::Bidirectional);
        app.register_component::<FocusLocus>(ChannelDirection::Bidirectional);
        app.register_component::<TouchprintLocus>(ChannelDirection::Bidirectional);
        app.register_component::<StylusLocus>(ChannelDirection::Bidirectional);

        app.add_observer(Self::observer_spawns_local_loci_after_stratum)
            .add_systems(Startup, Self::spawn_local_avatar)
            .add_systems(
                PreUpdate,
                (Self::detect_local_mouse_loci)
                    .after(UiSystem::Focus)
                    .run_if(on_event::<MouseMotion>),
            )
            .add_systems(Update, constantly_update_node_from_locus_position)
            .add_observer(observer_configures_arriving_mouse_loci)
            .add_observer(observer_transfers_locus_position_to_node)
            .add_observer(observer_enables_visibility_on_detection)
            .add_observer(observer_disables_visibility_on_concealment);
    }
}
impl AvatarPlugin {
    /// Spawn the user's Avatar, the parent of all of their stuff. Replicated to servers with local client having authority.
    fn spawn_local_avatar(mut commands: Commands, local_avatar_query: Query<&LocalAvatar>) {
        if !local_avatar_query.is_empty() {
            error!("Local Avatar detected on startup before spawning one?!");
            return;
        }

        commands.spawn((
            Name::new("Avatar"),
            LocalAvatar,
            Avatar::new_local(714),
            ClientReplicate::default(),
        ));
    }

    /// Observer for spawning local locus entities after a [`LociStratum`] is spawned.
    fn observer_spawns_local_loci_after_stratum(
        _trigger: Trigger<LocalLociStratumSpawned>,
        mut commands: Commands,
    ) {
        commands.spawn(local_mouse_locus_template());
    }

    /// Updates all local mouse loci by checking if its parent `GraniteRoot`'s `RelativeCursorPosition` detected a mouse.
    fn detect_local_mouse_loci(
        mut commands: Commands,
        loci_stratum_query: Query<&RelativeCursorPosition, With<LociStratum>>,
        mut local_mouse_locus_query: Query<
            (
                Entity,
                &Parent,
                &mut Node,
                &mut LocusPosition,
                &mut Visibility,
                Option<&LocusDetected>,
            ),
            (With<MouseLocus>, With<LocalAvatar>),
        >,
    ) {
        for (
            mouse_locus_entity,
            loci_stratum_parent,
            mut mouse_locus_node,
            mut locus_position,
            mut node_visibility,
            locus_detected,
        ) in &mut local_mouse_locus_query
        {
            let Ok(parent_relcurpos) = loci_stratum_query.get(loci_stratum_parent.get()) else {
                error!("Mouse locus parent GraniteRoot entity not found with RelativeCursorPosition component!");
                continue;
            };

            let Some(detected_relative_position) = parent_relcurpos.normalized else {
                if let Visibility::Inherited = *node_visibility {
                    node_visibility.toggle_inherited_hidden();
                }
                if locus_detected.is_some() {
                    commands
                        .entity(mouse_locus_entity)
                        .remove::<LocusDetected>();
                }
                continue;
            };

            // Stick a copy in here, to replicate to others if needed.
            locus_position.set_pos(detected_relative_position);

            mouse_locus_node.left = Val::Percent(detected_relative_position.x * 100.0);
            mouse_locus_node.top = Val::Percent(detected_relative_position.y * 100.0);
            if let Visibility::Hidden = *node_visibility {
                node_visibility.toggle_inherited_hidden();
            }

            // Also needed for replication, as `Visibility` doesn't implement serde traits:
            if locus_detected.is_none() {
                commands.entity(mouse_locus_entity).insert(LocusDetected);
            }
        }
    }
}

/// Marker component separating out entities that belong to the local user and are not replicated from a server.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
struct LocalAvatar;

/// Main component for the entity representation of a user. Parent of all entities the user controls.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct Avatar {
    /// The Lightyear ID assigned this user.
    lightyear_id: ClientId,
}
impl Default for Avatar {
    fn default() -> Self {
        Avatar {
            lightyear_id: ClientId::Local(1),
        }
    }
}
impl Avatar {
    /// Create a new local profile from scratch.
    fn new_local(local_id: u64) -> Self {
        Avatar {
            lightyear_id: ClientId::Local(local_id),
        }
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
    primary: Color,
    /// Accessory color, less prominent than the primary.
    secondary: Color,
    /// Rare accent color.
    tertiary: Color,
}
impl Default for LocusColor {
    fn default() -> Self {
        LocusColor {
            primary: Color::oklcha(1.0, 0.0, 0.0, 1.0),
            secondary: Color::oklcha(0.7, 0.15, 85.0, 1.0),
            tertiary: Color::oklcha(0.75, 0.15, 225.0, 1.0),
        }
    }
}
impl LocusColor {
    /// Default Light
    const MATERIAL_LIGHT: LocusColor = LocusColor {
        primary: Color::oklcha(1.0, 0.0, 0.0, 1.0),
        secondary: Color::oklcha(0.7, 0.15, 85.0, 1.0),
        tertiary: Color::oklcha(0.75, 0.15, 225.0, 1.0),
    };

    /// Custom new color.
    #[must_use = "Struct is initialized, but is never used!"]
    pub fn new(primary: Color, secondary: Color, tertiary: Color) -> Self {
        LocusColor {
            primary,
            secondary,
            tertiary,
        }
    }

    /// Returns the primary color as the [`Color`] type.
    #[must_use = "You called for a Color, but never used it!"]
    pub fn primary(self) -> Color {
        self.primary
    }

    /// Returns the secondary color as the [`Color`] type.
    #[must_use = "You called for a Color, but never used it!"]
    pub fn secondary(self) -> Color {
        self.secondary
    }

    /// Returns the tertiary color as the [`Color`] type.
    #[must_use = "You called for a Color, but never used it!"]
    pub fn tertiary(self) -> Color {
        self.tertiary
    }
}

/// In relative screenspace coordinates, from top-left `(0.0, 0.0)` to bottom right `(1.0, 1.0)`
///
/// Here for network replication, would be too expensive to send over the whole Node.
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component, Serialize, Deserialize, Debug)]
pub(crate) struct LocusPosition {
    /// Cheap xy to send over the network.
    pos: Vec2,
}
impl Default for LocusPosition {
    fn default() -> Self {
        LocusPosition {
            pos: Vec2::new(0.5, 0.5),
        }
    }
}
impl Diffable for LocusPosition {
    type Delta = (f32, f32);

    fn base_value() -> Self {
        Self { pos: Vec2::ZERO }
    }

    fn diff(&self, new: &Self) -> Self::Delta {
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "f32 expressing percentages, will likely not overflow."
        )]
        let diff = new.pos() - self.pos();
        (diff.x, diff.y)
    }

    fn apply_diff(&mut self, delta: &Self::Delta) {
        let new_pos = Vec2::new(self.pos().x - delta.0, self.pos().y - delta.1);
        self.set_pos(new_pos);
    }
}
impl Add for LocusPosition {
    type Output = LocusPosition;

    fn add(self, rhs: Self) -> Self::Output {
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "f32 expressing percentages, will likely not overflow."
        )]
        LocusPosition {
            pos: self.pos() + rhs.pos(),
        }
    }
}
impl Mul<f32> for LocusPosition {
    type Output = LocusPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "f32 expressing percentages, will likely not overflow."
        )]
        LocusPosition {
            pos: self.pos() * rhs,
        }
    }
}
impl Linear for LocusPosition {
    fn lerp(start: &Self, _other: &Self, _t: f32) -> Self {
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
pub(crate) struct LocusDetected;

/// Easy template for a local mouse locus, until BSN lands.
fn local_mouse_locus_template() -> impl Bundle {
    (
        Name::new("Mouse Locus"),
        LocalAvatar,
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            height: Val::Px(10.0),
            width: Val::Px(10.0),
            border: UiRect::all(Val::Px(1.0)),
            ..Default::default()
        },
        BackgroundColor(LocusColor::MATERIAL_LIGHT.tertiary()),
        BorderColor(LocusColor::MATERIAL_LIGHT.secondary()),
        BorderRadius::new(Val::ZERO, Val::Px(5.0), Val::Px(7.5), Val::Px(5.0)),
        MouseLocus,
        LocusPosition::default(),
        LocusColor::MATERIAL_LIGHT,
        ClientReplicate::default(),
        Replicated {
            from: Some(ClientId::Local(0)),
        },
    )
}

/// Global observer that configures newly spawned remote client-controlled mouse loci on the server.
/// Mostly adds non-replicable components, and adds it to the local loci stratum.
fn observer_configures_arriving_mouse_loci(
    trigger: Trigger<OnAdd, MouseLocus>,
    mut commands: Commands,
    loci_stratum_query: Query<Entity, With<LociStratum>>,
    replicated_mouse_locus_query: Query<&Replicated, (With<MouseLocus>, Without<Parent>)>,
) {
    if replicated_mouse_locus_query.contains(trigger.entity()) {
        let Ok(loci_stratum_entity) = loci_stratum_query.get_single() else {
            warn!("[AVATAR] Single LociStratum not found in query!");
            return;
        };

        let Ok(replicated) = replicated_mouse_locus_query.get(trigger.entity()) else {
            warn!("[AVATAR] Triggered remote mouse locus optional Replicate component not found?");
            return;
        };

        info!("Recieved Mouse Loci: Configuring...");

        if let Some(client_id) = replicated.from {
            // If we got a client_id of some kind, we are the server. So, we replicate it back to everyone else.
            // Also, we add nodes that can't be serialized over the network, if not local. Local already has them.
            match client_id {
                ClientId::Local(id) => {
                    commands.entity(trigger.entity()).insert((
                        ServerReplicate {
                        controlled_by: ControlledBy {
                            target: NetworkTarget::Single(ClientId::Local(id)),
                            ..default()
                        },
                        authority: AuthorityPeer::Client(ClientId::Local(id)),
                        target: ReplicationTarget {
                            target: NetworkTarget::AllExceptSingle(ClientId::Local(id)),
                        },
                        ..default()
                    },));
                }
                ClientId::Netcode(id) => {
                    info!("Got a client_id, so we got this from a client, meaning we are the server. Adding ServerReplicate...");
                    commands.entity(trigger.entity()).insert((
                        Visibility::Inherited,
                        Node {
                            position_type: PositionType::Absolute,
                            height: Val::Px(10.0),
                            width: Val::Px(10.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..Default::default()
                        },
                        ServerReplicate {
                            controlled_by: ControlledBy {
                                target: NetworkTarget::Single(ClientId::Netcode(id)),
                                ..default()
                            },
                            authority: AuthorityPeer::Client(ClientId::Netcode(id)),
                            target: ReplicationTarget {
                                target: NetworkTarget::AllExceptSingle(ClientId::Netcode(id)),
                            },
                            ..default()
                        },
                    ));
                }
                ClientId::Steam(_id) => {
                    info!("A Steam connection?? idk wtf to do with this yet");
                }
            }
        } else {
            // If None, then the server sent us this, and we are a client.
            // So, we just add the non-serializable components.
            info!("Got a None, so we are a client. Adding nonserializable components...");
            commands.entity(trigger.entity()).insert((
                Visibility::Inherited,
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(10.0),
                    width: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..Default::default()
                },
            ));
        }
        // Now we stash it on the Loci stratum, marking it as processed.
        commands
            .entity(loci_stratum_entity)
            .add_child(trigger.entity());
    }
}

/// Global observer edits the `Visibility` of any entity with a new `LocusDetected`.
fn observer_enables_visibility_on_detection(
    trigger: Trigger<OnAdd, LocusDetected>,
    mut general_locus_query: Query<&mut Visibility>,
) {
    let Ok(mut locus_visibility) = general_locus_query.get_mut(trigger.entity()) else {
        warn!("[AVATAR] Unable to get Visibility component for detected entity!");
        return;
    };

    match *locus_visibility {
        Visibility::Inherited => {}
        Visibility::Hidden => {
            locus_visibility.toggle_inherited_hidden();
        }
        Visibility::Visible => {
            warn!("[AVATAR] LocusDetected functionality ignored due to Visibility::Visible!");
        }
    };
}

/// Global observer edits the `Visibility` of any entity that had `LocusDetected` removed.
fn observer_disables_visibility_on_concealment(
    trigger: Trigger<OnRemove, LocusDetected>,
    mut general_locus_query: Query<&mut Visibility>,
) {
    let Ok(mut locus_visibility) = general_locus_query.get_mut(trigger.entity()) else {
        warn!("[AVATAR] Unable to get Visibility component for concealed entity!");
        return;
    };

    match *locus_visibility {
        Visibility::Inherited => {
            locus_visibility.toggle_inherited_hidden();
        }
        Visibility::Hidden => {}
        Visibility::Visible => {
            warn!("[AVATAR] LocusDetected functionality ignored due to Visibility::Visible!");
        }
    };
}

/// Global observer updates the node of any entity with changes to `LocusPosition`.
fn observer_transfers_locus_position_to_node(
    trigger: Trigger<ServerComponentUpdateEvent<LocusPosition>>,
    mut remote_locus_query: Query<(&LocusPosition, &mut Node)>,
) {
    info!("Observer for transferring LocusPosition to Node was triggered!!");

    let Ok((locus_pos, mut locus_node)) = remote_locus_query.get_mut(trigger.entity()) else {
        warn!("[AVATAR] Unable to get LocusPosition & Node components for triggered entity!");
        return;
    };

    locus_node.left = Val::Percent(locus_pos.pos().x * 100.0);
    locus_node.top = Val::Percent(locus_pos.pos().y * 100.0);
}

/// Constant change detection system while I figure out why the above observer isn't seeing changes.
fn constantly_update_node_from_locus_position(
    mut update_query: Query<(&LocusPosition, &mut Node), Changed<LocusPosition>>,
) {
    for (locus_pos, mut locus_node) in &mut update_query {
        locus_node.left = Val::Percent(locus_pos.pos().x * 100.0);
        locus_node.top = Val::Percent(locus_pos.pos().y * 100.0);
    }
}
