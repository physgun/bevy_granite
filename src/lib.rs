//! crate level doc for `bevy_granite`

// Finally... a worthy opponent!
#![warn(clippy::pedantic)]
#![warn(clippy::style)]

// Clippy restriction-type lints
#![warn(clippy::absolute_paths)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::as_conversions)]
#![warn(clippy::as_underscore)]
#![warn(clippy::cfg_not_test)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_enum_variants_with_brackets)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::error_impl_error)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::exit)]
#![warn(clippy::expect_used)]
#![warn(clippy::field_scoped_visibility_modifiers)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::impl_trait_in_params)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::infinite_loop)]
#![warn(clippy::integer_division)]
#![warn(clippy::integer_division_remainder_used)]
#![warn(clippy::large_include_file)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::let_underscore_untyped)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::non_zero_suggestions)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::partial_pub_fields)]
#![warn(clippy::pattern_type_mismatch)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::pub_use)]
#![warn(clippy::pub_without_shorthand)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::renamed_function_params)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::semicolon_outside_block)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::single_char_lifetime_names)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unreachable)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_result_ok)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_debug)]

// Lints incompatible with Bevy
#![allow(clippy::needless_pass_by_value, reason = "Incompatible with Bevy's dependency injection techniques.")]
#![allow(clippy::type_complexity, reason = "Incompatible with Bevy's ECS Query<> idioms.")]

// Inspired by Leafwing Studios:
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use bevy::prelude::*;

use bevy::winit::{UpdateMode, WinitSettings};

use network::NetworkPlugin;
use workbench::WorkbenchPlugin;

pub mod network;
pub mod workbench;

/// Primary documentation for the Granite plugin.
pub struct GranitePlugin;
impl Plugin for GranitePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WinitSettings {
                focused_mode: UpdateMode::Continuous,
                unfocused_mode: UpdateMode::Continuous,
            })
            .add_plugins(NetworkPlugin)
            .add_plugins(WorkbenchPlugin);
    }
}