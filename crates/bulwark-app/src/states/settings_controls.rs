use super::AppState;
use crate::input_mapping::keycode_to_name;
use crate::ui;
use crate::{ConfigContext, FrameInput};
use bulwark_core::input::InputAction;
use macroquad::prelude::*;

const ACTIONS: &[InputAction] = InputAction::ALL;
const ACTION_COUNT: usize = 8; // InputAction::ALL.len()

/// All known gamepad button names for the rebind picker.
const GAMEPAD_BUTTONS: &[&str] = &[
    "A",
    "B",
    "X",
    "Y",
    "LB",
    "RB",
    "LT",
    "RT",
    "DPadUp",
    "DPadDown",
    "DPadLeft",
    "DPadRight",
    "L3",
    "R3",
    "Menu",
    "Options",
];

const SIZE: f32 = 22.0;
const SPACING: f32 = 32.0;
const START_Y: f32 = 150.0;

/// Row indices:
/// 0            = "-- Keyboard --" header
/// 1..=8        = keyboard bindings (ACTION_COUNT)
/// 9            = "" spacer
/// 10           = "-- Gamepad --" header
/// 11..=18      = gamepad bindings (ACTION_COUNT)
/// 19           = "" spacer
/// 20           = "Reset Defaults"
/// 21           = "Back"
const KEYBOARD_HEADER: usize = 0;
const KEYBOARD_START: usize = 1;
const GAMEPAD_HEADER: usize = KEYBOARD_START + ACTION_COUNT + 1; // 10
const GAMEPAD_START: usize = GAMEPAD_HEADER + 1; // 11
const RESET_ROW: usize = GAMEPAD_START + ACTION_COUNT + 1; // 20
const BACK_ROW: usize = RESET_ROW + 1; // 21
const TOTAL_ROWS: usize = BACK_ROW + 1; // 22

fn is_selectable(row: usize) -> bool {
    // Headers and spacers are not selectable
    row != KEYBOARD_HEADER
        && row != KEYBOARD_START + ACTION_COUNT
        && row != GAMEPAD_HEADER
        && row != GAMEPAD_START + ACTION_COUNT
}

pub struct ControlsSettingsState {
    selected: usize,
    key_bindings: Vec<String>,
    gamepad_bindings: Vec<String>,
    /// Waiting for keyboard key press on this action index.
    waiting_for_key: Option<usize>,
    /// Waiting for gamepad button press on this action index.
    waiting_for_button: Option<usize>,
    initialized: bool,
}

impl ControlsSettingsState {
    pub fn new() -> Self {
        let kd = bulwark_core::config::ControlsConfig::default();
        let gd = bulwark_core::config::GamepadConfig::default();
        Self {
            selected: KEYBOARD_START,
            key_bindings: vec![
                kd.key_up,
                kd.key_down,
                kd.key_left,
                kd.key_right,
                kd.key_accept,
                kd.key_cancel,
                kd.key_rotate_cw,
                kd.key_rotate_ccw,
            ],
            gamepad_bindings: vec![
                gd.btn_up,
                gd.btn_down,
                gd.btn_left,
                gd.btn_right,
                gd.btn_accept,
                gd.btn_cancel,
                gd.btn_rotate_cw,
                gd.btn_rotate_ccw,
            ],
            waiting_for_key: None,
            waiting_for_button: None,
            initialized: false,
        }
    }

    fn init_from_config(&mut self, ctx: &ConfigContext) {
        let c = &ctx.user_config.controls;
        self.key_bindings = vec![
            c.key_up.clone(),
            c.key_down.clone(),
            c.key_left.clone(),
            c.key_right.clone(),
            c.key_accept.clone(),
            c.key_cancel.clone(),
            c.key_rotate_cw.clone(),
            c.key_rotate_ccw.clone(),
        ];
        let g = &c.gamepad;
        self.gamepad_bindings = vec![
            g.btn_up.clone(),
            g.btn_down.clone(),
            g.btn_left.clone(),
            g.btn_right.clone(),
            g.btn_accept.clone(),
            g.btn_cancel.clone(),
            g.btn_rotate_cw.clone(),
            g.btn_rotate_ccw.clone(),
        ];
        self.initialized = true;
    }

    fn move_selection(&mut self, dir: i32) {
        let mut next = self.selected as i32 + dir;
        loop {
            next = next.rem_euclid(TOTAL_ROWS as i32);
            if is_selectable(next as usize) {
                break;
            }
            next += dir;
        }
        self.selected = next as usize;
    }

    pub fn update(
        &mut self,
        _dt: f32,
        input: &FrameInput,
        ctx: &mut ConfigContext,
    ) -> Option<AppState> {
        if !self.initialized {
            self.init_from_config(ctx);
        }

        // Waiting for keyboard key press
        if let Some(action_idx) = self.waiting_for_key {
            if let Some(key) = get_last_key_pressed() {
                let name = keycode_to_name(key);
                if name != "?" {
                    self.key_bindings[action_idx] = name.to_string();
                }
                self.waiting_for_key = None;
            }
            return None;
        }

        // Waiting for gamepad button press
        if let Some(action_idx) = self.waiting_for_button {
            // Check if any gamepad button name appears in last_button
            // We read from FrameInput's last_key_name... but we need gamepad button.
            // Actually, the gamepad events are consumed in main loop. For rebinding,
            // we need to detect raw button presses. Let's check last_key_name for now
            // and also accept keyboard to type a button name... Actually let's just
            // use a picker approach: Accept cycles through known buttons.
            // For simplicity: Left/Right cycle through GAMEPAD_BUTTONS, Accept confirms.
            for action in &input.actions {
                match action {
                    InputAction::Left => {
                        let cur = GAMEPAD_BUTTONS
                            .iter()
                            .position(|&b| b == self.gamepad_bindings[action_idx])
                            .unwrap_or(0);
                        let next =
                            (cur as i32 - 1).rem_euclid(GAMEPAD_BUTTONS.len() as i32) as usize;
                        self.gamepad_bindings[action_idx] = GAMEPAD_BUTTONS[next].to_string();
                    }
                    InputAction::Right => {
                        let cur = GAMEPAD_BUTTONS
                            .iter()
                            .position(|&b| b == self.gamepad_bindings[action_idx])
                            .unwrap_or(0);
                        let next = (cur + 1) % GAMEPAD_BUTTONS.len();
                        self.gamepad_bindings[action_idx] = GAMEPAD_BUTTONS[next].to_string();
                    }
                    InputAction::Accept | InputAction::Cancel => {
                        self.waiting_for_button = None;
                    }
                    _ => {}
                }
            }
            return None;
        }

        // Mouse hover
        let (mx, my) = input.mouse_pos;
        for row in 0..TOTAL_ROWS {
            if !is_selectable(row) {
                continue;
            }
            let y = START_Y + row as f32 * SPACING;
            if my >= y - 16.0 && my <= y + 6.0 && mx >= 100.0 && mx <= screen_width() - 100.0 {
                self.selected = row;
                break;
            }
        }

        for action in &input.actions {
            match action {
                InputAction::Up => self.move_selection(-1),
                InputAction::Down => self.move_selection(1),
                InputAction::Accept => {
                    if self.selected >= KEYBOARD_START
                        && self.selected < KEYBOARD_START + ACTION_COUNT
                    {
                        self.waiting_for_key = Some(self.selected - KEYBOARD_START);
                    } else if self.selected >= GAMEPAD_START
                        && self.selected < GAMEPAD_START + ACTION_COUNT
                    {
                        self.waiting_for_button = Some(self.selected - GAMEPAD_START);
                    } else if self.selected == RESET_ROW {
                        let kd = bulwark_core::config::ControlsConfig::default();
                        self.key_bindings = vec![
                            kd.key_up,
                            kd.key_down,
                            kd.key_left,
                            kd.key_right,
                            kd.key_accept,
                            kd.key_cancel,
                            kd.key_rotate_cw,
                            kd.key_rotate_ccw,
                        ];
                        let gd = bulwark_core::config::GamepadConfig::default();
                        self.gamepad_bindings = vec![
                            gd.btn_up,
                            gd.btn_down,
                            gd.btn_left,
                            gd.btn_right,
                            gd.btn_accept,
                            gd.btn_cancel,
                            gd.btn_rotate_cw,
                            gd.btn_rotate_ccw,
                        ];
                    } else if self.selected == BACK_ROW {
                        self.apply(ctx);
                        return Some(AppState::Settings(super::settings::SettingsState::new()));
                    }
                }
                InputAction::Cancel => {
                    self.apply(ctx);
                    return Some(AppState::Settings(super::settings::SettingsState::new()));
                }
                _ => {}
            }
        }
        None
    }

    fn apply(&self, ctx: &mut ConfigContext) {
        let c = &mut ctx.user_config.controls;
        c.key_up = self.key_bindings[0].clone();
        c.key_down = self.key_bindings[1].clone();
        c.key_left = self.key_bindings[2].clone();
        c.key_right = self.key_bindings[3].clone();
        c.key_accept = self.key_bindings[4].clone();
        c.key_cancel = self.key_bindings[5].clone();
        c.key_rotate_cw = self.key_bindings[6].clone();
        c.key_rotate_ccw = self.key_bindings[7].clone();

        c.gamepad.btn_up = self.gamepad_bindings[0].clone();
        c.gamepad.btn_down = self.gamepad_bindings[1].clone();
        c.gamepad.btn_left = self.gamepad_bindings[2].clone();
        c.gamepad.btn_right = self.gamepad_bindings[3].clone();
        c.gamepad.btn_accept = self.gamepad_bindings[4].clone();
        c.gamepad.btn_cancel = self.gamepad_bindings[5].clone();
        c.gamepad.btn_rotate_cw = self.gamepad_bindings[6].clone();
        c.gamepad.btn_rotate_ccw = self.gamepad_bindings[7].clone();

        ctx.save_user_config();
    }

    pub fn draw(&self) {
        clear_background(ui::BG_COLOR);
        ui::draw_title("CONTROLS", 48.0, 100.0, ui::GOLD);

        let label_x = screen_width() / 2.0 - 160.0;
        let value_x = screen_width() / 2.0 + 40.0;

        for row in 0..TOTAL_ROWS {
            let y = START_Y + row as f32 * SPACING;
            let is_selected = row == self.selected;

            // Section headers
            if row == KEYBOARD_HEADER {
                draw_text("-- Keyboard --", label_x, y, SIZE, ui::GOLD);
                continue;
            }
            if row == GAMEPAD_HEADER {
                draw_text("-- Gamepad --", label_x, y, SIZE, ui::GOLD);
                continue;
            }
            // Spacers
            if !is_selectable(row) {
                continue;
            }

            let color = if is_selected {
                WHITE
            } else {
                Color::new(0.6, 0.6, 0.6, 1.0)
            };

            if is_selected {
                draw_rectangle(
                    label_x - 16.0,
                    y - 18.0,
                    screen_width() - 2.0 * (label_x - 16.0),
                    28.0,
                    Color::new(0.3, 0.3, 0.5, 0.6),
                );
                draw_text(">", label_x - 22.0, y, SIZE, ui::GOLD);
            }

            if (KEYBOARD_START..KEYBOARD_START + ACTION_COUNT).contains(&row) {
                let idx = row - KEYBOARD_START;
                draw_text(ACTIONS[idx].name(), label_x, y, SIZE, color);
                if self.waiting_for_key == Some(idx) {
                    draw_text("Press a key...", value_x, y, SIZE, ui::GOLD);
                } else {
                    draw_text(&self.key_bindings[idx], value_x, y, SIZE, color);
                }
            } else if (GAMEPAD_START..GAMEPAD_START + ACTION_COUNT).contains(&row) {
                let idx = row - GAMEPAD_START;
                draw_text(ACTIONS[idx].name(), label_x, y, SIZE, color);
                if self.waiting_for_button == Some(idx) {
                    draw_text(
                        &format!("< {} >", self.gamepad_bindings[idx]),
                        value_x,
                        y,
                        SIZE,
                        ui::GOLD,
                    );
                } else {
                    draw_text(&self.gamepad_bindings[idx], value_x, y, SIZE, color);
                }
            } else if row == RESET_ROW {
                draw_text("Reset Defaults", label_x, y, SIZE, color);
            } else if row == BACK_ROW {
                draw_text("Back", label_x, y, SIZE, color);
            }
        }
    }
}
