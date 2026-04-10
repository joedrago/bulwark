use crate::config::ControlsConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
    RotateCW,
    RotateCCW,
}

impl InputAction {
    pub const ALL: &[InputAction] = &[
        InputAction::Up,
        InputAction::Down,
        InputAction::Left,
        InputAction::Right,
        InputAction::Accept,
        InputAction::Cancel,
        InputAction::RotateCW,
        InputAction::RotateCCW,
    ];

    pub fn name(self) -> &'static str {
        match self {
            InputAction::Up => "Up",
            InputAction::Down => "Down",
            InputAction::Left => "Left",
            InputAction::Right => "Right",
            InputAction::Accept => "Accept",
            InputAction::Cancel => "Cancel",
            InputAction::RotateCW => "RotateCW",
            InputAction::RotateCCW => "RotateCCW",
        }
    }
}

/// Returns all (key_name, action) pairs from the keyboard controls config.
pub fn bindings_from_config(config: &ControlsConfig) -> Vec<(String, InputAction)> {
    vec![
        (config.key_up.clone(), InputAction::Up),
        (config.key_down.clone(), InputAction::Down),
        (config.key_left.clone(), InputAction::Left),
        (config.key_right.clone(), InputAction::Right),
        (config.key_accept.clone(), InputAction::Accept),
        (config.key_cancel.clone(), InputAction::Cancel),
        (config.key_rotate_cw.clone(), InputAction::RotateCW),
        (config.key_rotate_ccw.clone(), InputAction::RotateCCW),
    ]
}

/// Returns all (button_name, action) pairs from the gamepad config.
pub fn gamepad_bindings_from_config(
    config: &crate::config::GamepadConfig,
) -> Vec<(String, InputAction)> {
    vec![
        (config.btn_up.clone(), InputAction::Up),
        (config.btn_down.clone(), InputAction::Down),
        (config.btn_left.clone(), InputAction::Left),
        (config.btn_right.clone(), InputAction::Right),
        (config.btn_accept.clone(), InputAction::Accept),
        (config.btn_cancel.clone(), InputAction::Cancel),
        (config.btn_rotate_cw.clone(), InputAction::RotateCW),
        (config.btn_rotate_ccw.clone(), InputAction::RotateCCW),
    ]
}
