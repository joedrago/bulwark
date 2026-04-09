use super::GamepadEvent;
use bulwark_core::input::InputAction;
use objc2_game_controller::{GCController, GCControllerButtonInput, GCDevice, GCExtendedGamepad};

pub struct PlatformGamepad {
    /// Track which buttons were pressed last frame to detect edges.
    prev_buttons: ButtonSnapshot,
}

#[derive(Default, Clone)]
struct ButtonSnapshot {
    dpad_up: bool,
    dpad_down: bool,
    dpad_left: bool,
    dpad_right: bool,
    button_a: bool,
    button_b: bool,
    button_x: bool,
    button_y: bool,
    left_shoulder: bool,
    right_shoulder: bool,
    left_trigger: bool,
    right_trigger: bool,
    left_stick_button: bool,
    right_stick_button: bool,
    button_menu: bool,
    button_options: bool,
}

fn get_first_controller() -> Option<objc2::rc::Retained<GCController>> {
    let controllers = unsafe { GCController::controllers() };
    if !controllers.is_empty() {
        Some(controllers.objectAtIndex(0))
    } else {
        None
    }
}

fn controller_name(controller: &GCController) -> String {
    unsafe { controller.vendorName() }
        .map(|n: objc2::rc::Retained<objc2_foundation::NSString>| n.to_string())
        .unwrap_or_else(|| "Unknown Controller".to_string())
}

impl PlatformGamepad {
    pub fn new() -> Self {
        Self {
            prev_buttons: ButtonSnapshot::default(),
        }
    }

    pub fn detect_initial(&mut self) -> (bool, String) {
        if let Some(controller) = get_first_controller() {
            (true, controller_name(&controller))
        } else {
            (false, String::new())
        }
    }

    pub fn poll(&mut self) -> Vec<GamepadEvent> {
        let mut events = Vec::new();

        let Some(controller) = get_first_controller() else {
            if self.prev_buttons.any_pressed() {
                self.prev_buttons = ButtonSnapshot::default();
                events.push(GamepadEvent::Disconnected);
            }
            return events;
        };

        let name = controller_name(&controller);

        let Some(gamepad) = (unsafe { controller.extendedGamepad() }) else {
            return events;
        };

        let current = read_buttons(&gamepad);

        // Detect newly pressed buttons (edge detection)
        type ButtonCheck = (
            &'static str,
            Option<InputAction>,
            fn(&ButtonSnapshot) -> bool,
        );
        let button_checks: &[ButtonCheck] = &[
            ("DPadUp", Some(InputAction::Up), |s| s.dpad_up),
            ("DPadDown", Some(InputAction::Down), |s| s.dpad_down),
            ("DPadLeft", Some(InputAction::Left), |s| s.dpad_left),
            ("DPadRight", Some(InputAction::Right), |s| s.dpad_right),
            ("A", Some(InputAction::Accept), |s| s.button_a),
            ("B", Some(InputAction::Cancel), |s| s.button_b),
            ("X", None, |s| s.button_x),
            ("Y", None, |s| s.button_y),
            ("LB", Some(InputAction::RotateCCW), |s| s.left_shoulder),
            ("RB", Some(InputAction::RotateCW), |s| s.right_shoulder),
            ("LT", Some(InputAction::RotateCCW), |s| s.left_trigger),
            ("RT", Some(InputAction::RotateCW), |s| s.right_trigger),
            ("L3", None, |s| s.left_stick_button),
            ("R3", None, |s| s.right_stick_button),
            ("Menu", None, |s| s.button_menu),
            ("Options", None, |s| s.button_options),
        ];

        for (btn_name, action, getter) in button_checks {
            let is_now = getter(&current);
            let was_before = getter(&self.prev_buttons);
            if is_now && !was_before {
                events.push(GamepadEvent::ButtonPressed {
                    button: btn_name.to_string(),
                    action: *action,
                });
            }
        }

        // Report axis state (left stick) for debug display
        let (lx, ly) = read_left_stick(&gamepad);
        if lx.abs() > 0.2 || ly.abs() > 0.2 {
            events.push(GamepadEvent::AxisChanged {
                axis: format!("LStick=({:.2},{:.2})", lx, ly),
            });
        }

        // Check if we just connected (first time seeing any input)
        if !self.prev_buttons.any_pressed() && current.any_pressed() {
            events.insert(0, GamepadEvent::Connected { name });
        }

        self.prev_buttons = current;
        events
    }
}

fn is_pressed(button: &GCControllerButtonInput) -> bool {
    unsafe { button.isPressed() }
}

fn read_buttons(gamepad: &GCExtendedGamepad) -> ButtonSnapshot {
    unsafe {
        let dpad = gamepad.dpad();
        ButtonSnapshot {
            dpad_up: is_pressed(&dpad.up()),
            dpad_down: is_pressed(&dpad.down()),
            dpad_left: is_pressed(&dpad.left()),
            dpad_right: is_pressed(&dpad.right()),
            button_a: is_pressed(&gamepad.buttonA()),
            button_b: is_pressed(&gamepad.buttonB()),
            button_x: is_pressed(&gamepad.buttonX()),
            button_y: is_pressed(&gamepad.buttonY()),
            left_shoulder: is_pressed(&gamepad.leftShoulder()),
            right_shoulder: is_pressed(&gamepad.rightShoulder()),
            left_trigger: is_pressed(&gamepad.leftTrigger()),
            right_trigger: is_pressed(&gamepad.rightTrigger()),
            left_stick_button: gamepad
                .leftThumbstickButton()
                .is_some_and(|b| is_pressed(&b)),
            right_stick_button: gamepad
                .rightThumbstickButton()
                .is_some_and(|b| is_pressed(&b)),
            button_menu: is_pressed(&gamepad.buttonMenu()),
            button_options: gamepad.buttonOptions().is_some_and(|b| is_pressed(&b)),
        }
    }
}

fn read_left_stick(gamepad: &GCExtendedGamepad) -> (f32, f32) {
    unsafe {
        let stick = gamepad.leftThumbstick();
        (stick.xAxis().value(), stick.yAxis().value())
    }
}

impl ButtonSnapshot {
    fn any_pressed(&self) -> bool {
        self.dpad_up
            || self.dpad_down
            || self.dpad_left
            || self.dpad_right
            || self.button_a
            || self.button_b
            || self.button_x
            || self.button_y
            || self.left_shoulder
            || self.right_shoulder
            || self.left_trigger
            || self.right_trigger
            || self.left_stick_button
            || self.right_stick_button
            || self.button_menu
            || self.button_options
    }
}
