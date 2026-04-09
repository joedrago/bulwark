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

/// Maps a config key name to an InputAction.
/// Returns all (key_name, action) pairs from the controls config.
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
