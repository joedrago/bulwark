mod gamepad;
mod input_mapping;
mod states;
mod ui;

use bulwark_core::config::{self, AppConfig, DisplayMode, UserConfig};
use bulwark_core::input::{self, InputAction};
use gamepad::GamepadState;
use input_mapping::{key_name_to_keycode, keycode_to_name};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

fn window_conf() -> Conf {
    let exe_dir = config::exe_dir();
    let app_config: AppConfig = config::load_config(&exe_dir.join("app.toml"));

    Conf {
        window_title: "Bulwark".to_string(),
        window_width: app_config.window.width as i32,
        window_height: app_config.window.height as i32,
        fullscreen: matches!(app_config.window.mode, DisplayMode::Fullscreen),
        window_resizable: true,
        ..Default::default()
    }
}

/// Collected input actions for the current frame.
pub struct FrameInput {
    pub actions: Vec<InputAction>,
    pub last_key_name: String,
    pub mouse_pos: (f32, f32),
}

/// Mutable config state accessible to settings screens.
pub struct ConfigContext {
    pub app_config: AppConfig,
    pub user_config: UserConfig,
    pub app_config_path: PathBuf,
    pub user_config_path: PathBuf,
}

impl ConfigContext {
    pub fn save_app_config(&self) {
        if let Err(e) = config::save_config(&self.app_config_path, &self.app_config) {
            eprintln!("Warning: could not save app config: {}", e);
        }
    }

    pub fn save_user_config(&self) {
        if let Err(e) = config::save_config(&self.user_config_path, &self.user_config) {
            eprintln!("Warning: could not save user config: {}", e);
        }
    }

    /// Rebuild the keyboard binding map from current user config.
    pub fn rebuild_key_bindings(&self) -> HashMap<KeyCode, InputAction> {
        let config_bindings = input::bindings_from_config(&self.user_config.controls);
        let mut map = HashMap::new();
        for (key_name, action) in &config_bindings {
            if let Some(kc) = key_name_to_keycode(key_name) {
                map.insert(kc, *action);
            }
        }
        map
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let exe_dir = config::exe_dir();
    let user_dir = config::user_config_dir();

    let app_config_path = exe_dir.join("app.toml");
    let user_config_path = user_dir.join("user.toml");

    let app_config: AppConfig = config::load_config(&app_config_path);
    let user_config: UserConfig = config::load_user_config(&user_config_path);

    println!("Bulwark App v{}", bulwark_core::VERSION);
    println!("App config:\n{app_config}");
    println!("User config:\n{user_config}");

    let mut ctx = ConfigContext {
        app_config,
        user_config,
        app_config_path,
        user_config_path,
    };

    let mut key_bindings = ctx.rebuild_key_bindings();
    let mut gamepad = GamepadState::new();
    gamepad.set_button_map(&input::gamepad_bindings_from_config(
        &ctx.user_config.controls.gamepad,
    ));
    let mut last_key_name = String::from("(none)");

    // State machine
    let mut state = states::AppState::Splash(states::splash::SplashState::new());

    loop {
        // -- Gather input --
        gamepad.update();
        let mut actions = Vec::new();

        if let Some(key) = get_last_key_pressed() {
            last_key_name = keycode_to_name(key).to_string();
        }

        for (&keycode, &action) in &key_bindings {
            if is_key_pressed(keycode) {
                actions.push(action);
            }
        }

        // Enter and Escape always work as Accept/Cancel regardless of bindings
        if is_key_pressed(KeyCode::Enter) && !actions.contains(&InputAction::Accept) {
            actions.push(InputAction::Accept);
        }
        if is_key_pressed(KeyCode::Escape) && !actions.contains(&InputAction::Cancel) {
            actions.push(InputAction::Cancel);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            actions.push(InputAction::Accept);
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            actions.push(InputAction::Cancel);
        }

        for &action in InputAction::ALL {
            if gamepad.action_pressed(action) && !actions.contains(&action) {
                actions.push(action);
            }
        }

        let frame_input = FrameInput {
            actions,
            last_key_name: last_key_name.clone(),
            mouse_pos: mouse_position(),
        };

        // -- Update & draw current state --
        let dt = get_frame_time();
        if let Some(next) = state.update(dt, &frame_input, &mut ctx) {
            // Rebuild bindings on state transitions
            key_bindings = ctx.rebuild_key_bindings();
            gamepad.set_button_map(&input::gamepad_bindings_from_config(
                &ctx.user_config.controls.gamepad,
            ));
            state = next;
        }
        state.draw();

        next_frame().await
    }
}
