mod gamepad;

use bulwark_core::config::{self, AppConfig, DisplayMode, UserConfig};
use bulwark_core::input::{self, InputAction};
use gamepad::GamepadState;
use macroquad::prelude::*;
use std::collections::HashMap;

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

// ---------------------------------------------------------------------------
// Key name <-> macroquad KeyCode mapping
// ---------------------------------------------------------------------------

fn key_name_to_keycode(name: &str) -> Option<KeyCode> {
    match name {
        "A" => Some(KeyCode::A),
        "B" => Some(KeyCode::B),
        "C" => Some(KeyCode::C),
        "D" => Some(KeyCode::D),
        "E" => Some(KeyCode::E),
        "F" => Some(KeyCode::F),
        "G" => Some(KeyCode::G),
        "H" => Some(KeyCode::H),
        "I" => Some(KeyCode::I),
        "J" => Some(KeyCode::J),
        "K" => Some(KeyCode::K),
        "L" => Some(KeyCode::L),
        "M" => Some(KeyCode::M),
        "N" => Some(KeyCode::N),
        "O" => Some(KeyCode::O),
        "P" => Some(KeyCode::P),
        "Q" => Some(KeyCode::Q),
        "R" => Some(KeyCode::R),
        "S" => Some(KeyCode::S),
        "T" => Some(KeyCode::T),
        "U" => Some(KeyCode::U),
        "V" => Some(KeyCode::V),
        "W" => Some(KeyCode::W),
        "X" => Some(KeyCode::X),
        "Y" => Some(KeyCode::Y),
        "Z" => Some(KeyCode::Z),
        "Up" => Some(KeyCode::Up),
        "Down" => Some(KeyCode::Down),
        "Left" => Some(KeyCode::Left),
        "Right" => Some(KeyCode::Right),
        "Space" => Some(KeyCode::Space),
        "Enter" => Some(KeyCode::Enter),
        "Escape" => Some(KeyCode::Escape),
        "Tab" => Some(KeyCode::Tab),
        "Backspace" => Some(KeyCode::Backspace),
        "LeftShift" => Some(KeyCode::LeftShift),
        "RightShift" => Some(KeyCode::RightShift),
        "LeftControl" => Some(KeyCode::LeftControl),
        "RightControl" => Some(KeyCode::RightControl),
        "1" => Some(KeyCode::Key1),
        "2" => Some(KeyCode::Key2),
        "3" => Some(KeyCode::Key3),
        "4" => Some(KeyCode::Key4),
        "5" => Some(KeyCode::Key5),
        "6" => Some(KeyCode::Key6),
        "7" => Some(KeyCode::Key7),
        "8" => Some(KeyCode::Key8),
        "9" => Some(KeyCode::Key9),
        "0" => Some(KeyCode::Key0),
        _ => None,
    }
}

fn keycode_to_name(key: KeyCode) -> &'static str {
    match key {
        KeyCode::A => "A",
        KeyCode::B => "B",
        KeyCode::C => "C",
        KeyCode::D => "D",
        KeyCode::E => "E",
        KeyCode::F => "F",
        KeyCode::G => "G",
        KeyCode::H => "H",
        KeyCode::I => "I",
        KeyCode::J => "J",
        KeyCode::K => "K",
        KeyCode::L => "L",
        KeyCode::M => "M",
        KeyCode::N => "N",
        KeyCode::O => "O",
        KeyCode::P => "P",
        KeyCode::Q => "Q",
        KeyCode::R => "R",
        KeyCode::S => "S",
        KeyCode::T => "T",
        KeyCode::U => "U",
        KeyCode::V => "V",
        KeyCode::W => "W",
        KeyCode::X => "X",
        KeyCode::Y => "Y",
        KeyCode::Z => "Z",
        KeyCode::Up => "Up",
        KeyCode::Down => "Down",
        KeyCode::Left => "Left",
        KeyCode::Right => "Right",
        KeyCode::Space => "Space",
        KeyCode::Enter => "Enter",
        KeyCode::Escape => "Escape",
        KeyCode::Tab => "Tab",
        KeyCode::Backspace => "Backspace",
        KeyCode::LeftShift => "LeftShift",
        KeyCode::RightShift => "RightShift",
        KeyCode::LeftControl => "LeftControl",
        KeyCode::RightControl => "RightControl",
        KeyCode::Key1 => "1",
        KeyCode::Key2 => "2",
        KeyCode::Key3 => "3",
        KeyCode::Key4 => "4",
        KeyCode::Key5 => "5",
        KeyCode::Key6 => "6",
        KeyCode::Key7 => "7",
        KeyCode::Key8 => "8",
        KeyCode::Key9 => "9",
        KeyCode::Key0 => "0",
        _ => "?",
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[macroquad::main(window_conf)]
async fn main() {
    let exe_dir = config::exe_dir();
    let user_dir = config::user_config_dir();

    let app_config: AppConfig = config::load_config(&exe_dir.join("app.toml"));
    let user_config: UserConfig = config::load_user_config(&user_dir.join("user.toml"));

    println!("Bulwark App v{}", bulwark_core::VERSION);
    println!("App config:\n{app_config}");
    println!("User config:\n{user_config}");

    // Build keyboard binding map: KeyCode -> InputAction
    let config_bindings = input::bindings_from_config(&user_config.controls);
    let mut key_bindings: HashMap<KeyCode, InputAction> = HashMap::new();
    for (key_name, action) in &config_bindings {
        if let Some(kc) = key_name_to_keycode(key_name) {
            key_bindings.insert(kc, *action);
        } else {
            eprintln!("Warning: unknown key name in config: {}", key_name);
        }
    }

    let mut gamepad = GamepadState::new();

    // Debug display state
    let mut last_key_name = String::from("(none)");
    let mut active_actions: Vec<InputAction> = Vec::new();

    let ocean_blue = Color::new(0.15, 0.35, 0.65, 1.0);

    // Test grid settings
    let grid_cols = 40;
    let grid_rows = 30;
    let grid_color_ground = Color::new(0.35, 0.55, 0.25, 1.0);
    let grid_color_water = Color::new(0.18, 0.40, 0.70, 1.0);
    let grid_line_color = Color::new(0.0, 0.0, 0.0, 0.15);

    loop {
        // -- Input processing --
        gamepad.update();
        active_actions.clear();

        // Track last key pressed
        if let Some(key) = get_last_key_pressed() {
            last_key_name = keycode_to_name(key).to_string();
        }

        // Check keyboard bindings
        for (&keycode, &action) in &key_bindings {
            if is_key_pressed(keycode) {
                active_actions.push(action);
            }
        }

        // Mouse -> Accept (left click) and Cancel (right click)
        if is_mouse_button_pressed(MouseButton::Left) {
            active_actions.push(InputAction::Accept);
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            active_actions.push(InputAction::Cancel);
        }

        // Gamepad actions
        for &action in InputAction::ALL {
            if gamepad.action_pressed(action) && !active_actions.contains(&action) {
                active_actions.push(action);
            }
        }

        // -- Rendering --
        clear_background(ocean_blue);

        // Draw test grid
        let padding = 40.0;
        let available_w = screen_width() - padding * 2.0;
        let available_h = screen_height() - padding * 2.0 - 160.0;
        let cell_size = (available_w / grid_cols as f32).min(available_h / grid_rows as f32);
        let grid_w = cell_size * grid_cols as f32;
        let grid_h = cell_size * grid_rows as f32;
        let offset_x = (screen_width() - grid_w) / 2.0;
        let offset_y = padding;

        for row in 0..grid_rows {
            for col in 0..grid_cols {
                let x = offset_x + col as f32 * cell_size;
                let y = offset_y + row as f32 * cell_size;
                let is_water = row == 0
                    || row == grid_rows - 1
                    || col == 0
                    || col == grid_cols - 1
                    || (row < 3 && col < 5)
                    || (row > grid_rows - 4 && col > grid_cols - 6);
                let color = if is_water {
                    grid_color_water
                } else {
                    grid_color_ground
                };
                draw_rectangle(x, y, cell_size, cell_size, color);
            }
        }
        for row in 0..=grid_rows {
            let y = offset_y + row as f32 * cell_size;
            draw_line(offset_x, y, offset_x + grid_w, y, 1.0, grid_line_color);
        }
        for col in 0..=grid_cols {
            let x = offset_x + col as f32 * cell_size;
            draw_line(x, offset_y, x, offset_y + grid_h, 1.0, grid_line_color);
        }

        // -- Debug overlay --
        let debug_y = offset_y + grid_h + 20.0;
        let line_h = 22.0;
        let text_size = 20.0;
        let label_color = Color::new(0.8, 0.8, 0.8, 1.0);
        let value_color = WHITE;

        draw_text(
            &format!("Bulwark v{}", bulwark_core::VERSION),
            10.0,
            debug_y,
            text_size,
            label_color,
        );

        // Keyboard
        draw_text("Keyboard:", 10.0, debug_y + line_h, text_size, label_color);
        draw_text(
            &format!("Last key: {}", last_key_name),
            120.0,
            debug_y + line_h,
            text_size,
            value_color,
        );

        // Mouse
        let (mx, my) = mouse_position();
        let ml = is_mouse_button_down(MouseButton::Left);
        let mr = is_mouse_button_down(MouseButton::Right);
        draw_text(
            "Mouse:",
            10.0,
            debug_y + line_h * 2.0,
            text_size,
            label_color,
        );
        draw_text(
            &format!(
                "({:.0}, {:.0}) L:{} R:{}",
                mx,
                my,
                if ml { "down" } else { "up" },
                if mr { "down" } else { "up" },
            ),
            120.0,
            debug_y + line_h * 2.0,
            text_size,
            value_color,
        );

        // Gamepad
        draw_text(
            "Gamepad:",
            10.0,
            debug_y + line_h * 3.0,
            text_size,
            label_color,
        );
        if gamepad.connected {
            draw_text(
                &format!(
                    "{} | btn: {} | axis: {}",
                    gamepad.gamepad_name, gamepad.last_button, gamepad.last_axis,
                ),
                120.0,
                debug_y + line_h * 3.0,
                text_size,
                value_color,
            );
        } else {
            draw_text(
                "(not connected)",
                120.0,
                debug_y + line_h * 3.0,
                text_size,
                Color::new(0.6, 0.6, 0.6, 1.0),
            );
        }

        // Active actions
        draw_text(
            "Actions:",
            10.0,
            debug_y + line_h * 4.0,
            text_size,
            label_color,
        );
        if active_actions.is_empty() {
            draw_text(
                "(none)",
                120.0,
                debug_y + line_h * 4.0,
                text_size,
                Color::new(0.6, 0.6, 0.6, 1.0),
            );
        } else {
            let action_names: Vec<&str> = active_actions.iter().map(|a| a.name()).collect();
            draw_text(
                &action_names.join(", "),
                120.0,
                debug_y + line_h * 4.0,
                text_size,
                Color::new(0.3, 1.0, 0.3, 1.0),
            );
        }

        // Bindings reference
        draw_text(
            "Bindings:",
            10.0,
            debug_y + line_h * 5.0,
            text_size,
            label_color,
        );
        let binds_str: Vec<String> = config_bindings
            .iter()
            .map(|(k, a)| format!("{}={}", a.name(), k))
            .collect();
        draw_text(
            &binds_str.join("  "),
            120.0,
            debug_y + line_h * 5.0,
            16.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );

        next_frame().await
    }
}
