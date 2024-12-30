#![doc = include_str!("../README.md")]

use bevy::ecs::system::Res;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use command::CommandHints;
use config::ConsoleConfig;
use ui::ConsoleUiState; // Import the missing PickState type

#[cfg(feature = "builtin-parser")]
pub mod builtin_parser;
pub mod command;
pub mod config;
pub mod logging;
pub mod prelude;
pub mod ui;

/// Adds a Developer Console to your Bevy application.
///
/// Requires [custom_log_layer](logging::custom_log_layer).
pub struct DevConsolePlugin;
impl Plugin for DevConsolePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        #[cfg(feature = "builtin-parser")]
        {
            app.init_non_send_resource::<builtin_parser::Environment>();
            app.init_resource::<command::DefaultCommandParser>();
            #[cfg(feature = "builtin-parser-completions")]
            app.init_resource::<builtin_parser::completions::EnvironmentCache>();
        }
        #[cfg(feature = "completions")]
        app.init_resource::<command::AutoCompletions>();

        app.init_resource::<ConsoleUiState>()
            .init_resource::<CommandHints>()
            .init_resource::<ConsoleConfig>()
            .init_resource::<ConsoleWantsMouseInput>()
            .init_resource::<ConsoleWantsKeyboardInput>()
            .register_type::<ConsoleConfig>()
            .add_systems(
                Update,
                (
                    ui::read_logs,
                    (
                        ui::open_close_ui,
                        ui::render_ui_system.run_if(|s: Res<ConsoleUiState>| s.open),
                    )
                        .chain(),
                ),
            )
            .add_systems(
                PreUpdate,
                (check_console_inputs)
                    .after(bevy_egui::systems::process_input_system)
                    .before(bevy_egui::EguiSet::BeginFrame),
            );
    }
}

/// Lets you know if the mouse is hovering over / interacting with the console.
#[derive(Resource, Default)]
pub struct ConsoleWantsMouseInput(pub bool);

/// Lets you know if the keyboard is interacting with the console.
#[derive(Resource, Default)]
pub struct ConsoleWantsKeyboardInput(pub bool);

fn check_console_inputs(
    mut console_wants_mouse_input: ResMut<ConsoleWantsMouseInput>,
    mut console_wants_keyboard_input: ResMut<ConsoleWantsKeyboardInput>,
    mut contexts: EguiContexts,
) {
    if contexts.ctx_mut().is_pointer_over_area() || contexts.ctx_mut().wants_pointer_input() {
        console_wants_mouse_input.0 = true;
    } else {
        console_wants_mouse_input.0 = false;
    }

    if contexts.ctx_mut().wants_keyboard_input() {
        console_wants_keyboard_input.0 = true;
    } else {
        console_wants_keyboard_input.0 = false;
    }
}
