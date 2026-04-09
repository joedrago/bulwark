use super::GamepadEvent;
use bulwark_core::input::InputAction;

pub struct PlatformGamepad {
    gilrs: gilrs::Gilrs,
}

impl PlatformGamepad {
    pub fn new() -> Self {
        let gilrs = gilrs::Gilrs::new().expect("failed to init gilrs");
        Self { gilrs }
    }

    pub fn detect_initial(&mut self) -> (bool, String) {
        if let Some((_id, gp)) = self.gilrs.gamepads().next() {
            if gp.is_connected() {
                return (true, gp.name().to_string());
            }
        }
        (false, String::new())
    }

    pub fn poll(&mut self) -> Vec<GamepadEvent> {
        let mut events = Vec::new();

        while let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() {
            match event {
                gilrs::EventType::ButtonPressed(btn, _) => {
                    let name = self.gilrs.gamepad(id).name().to_string();
                    let action = match btn {
                        gilrs::Button::South => Some(InputAction::Accept),
                        gilrs::Button::East => Some(InputAction::Cancel),
                        gilrs::Button::RightTrigger | gilrs::Button::RightTrigger2 => {
                            Some(InputAction::RotateCW)
                        }
                        gilrs::Button::LeftTrigger | gilrs::Button::LeftTrigger2 => {
                            Some(InputAction::RotateCCW)
                        }
                        gilrs::Button::DPadUp => Some(InputAction::Up),
                        gilrs::Button::DPadDown => Some(InputAction::Down),
                        gilrs::Button::DPadLeft => Some(InputAction::Left),
                        gilrs::Button::DPadRight => Some(InputAction::Right),
                        _ => None,
                    };
                    // Ensure we report connected with the name
                    events.push(GamepadEvent::Connected { name });
                    events.push(GamepadEvent::ButtonPressed {
                        button: format!("{:?}", btn),
                        action,
                    });
                }
                gilrs::EventType::AxisChanged(axis, val, _) => {
                    events.push(GamepadEvent::AxisChanged {
                        axis: format!("{:?}={:.2}", axis, val),
                    });
                }
                gilrs::EventType::Disconnected => {
                    events.push(GamepadEvent::Disconnected);
                }
                gilrs::EventType::Connected => {
                    let name = self.gilrs.gamepad(id).name().to_string();
                    events.push(GamepadEvent::Connected { name });
                }
                _ => {}
            }
        }

        events
    }
}
