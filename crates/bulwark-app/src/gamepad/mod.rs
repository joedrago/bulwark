use bulwark_core::input::InputAction;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(not(target_os = "macos"))]
mod gilrs_backend;
#[cfg(not(target_os = "macos"))]
use gilrs_backend as platform;

/// Platform-agnostic gamepad state.
pub struct GamepadState {
    inner: platform::PlatformGamepad,
    pub connected: bool,
    pub gamepad_name: String,
    pub last_button: String,
    pub last_axis: String,
    actions_pressed: Vec<InputAction>,
}

impl GamepadState {
    pub fn new() -> Self {
        let mut inner = platform::PlatformGamepad::new();
        let (connected, name) = inner.detect_initial();
        if connected {
            println!("  gamepad: detected \"{}\"", name);
        } else {
            println!("  gamepad: no gamepads detected");
        }
        Self {
            inner,
            connected,
            gamepad_name: name,
            last_button: String::new(),
            last_axis: String::new(),
            actions_pressed: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.actions_pressed.clear();

        let events = self.inner.poll();
        for event in events {
            match event {
                GamepadEvent::Connected { name } => {
                    self.connected = true;
                    self.gamepad_name = name;
                }
                GamepadEvent::Disconnected => {
                    self.connected = false;
                    self.gamepad_name.clear();
                }
                GamepadEvent::ButtonPressed { button, action } => {
                    self.connected = true;
                    self.last_button = button;
                    if let Some(a) = action {
                        self.actions_pressed.push(a);
                    }
                }
                GamepadEvent::AxisChanged { axis } => {
                    self.connected = true;
                    self.last_axis = axis;
                }
            }
        }
    }

    pub fn action_pressed(&self, action: InputAction) -> bool {
        self.actions_pressed.contains(&action)
    }
}

/// Intermediate event type shared between backends.
pub enum GamepadEvent {
    Connected {
        name: String,
    },
    Disconnected,
    ButtonPressed {
        button: String,
        action: Option<InputAction>,
    },
    AxisChanged {
        axis: String,
    },
}
