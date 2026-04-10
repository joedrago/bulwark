use super::GamepadEvent;

pub struct PlatformGamepad {
    gilrs: gilrs::Gilrs,
}

/// Map gilrs button enum to our canonical button names (matching macOS names).
fn gilrs_button_name(btn: gilrs::Button) -> &'static str {
    match btn {
        gilrs::Button::South => "A",
        gilrs::Button::East => "B",
        gilrs::Button::West => "X",
        gilrs::Button::North => "Y",
        gilrs::Button::LeftTrigger => "LB",
        gilrs::Button::RightTrigger => "RB",
        gilrs::Button::LeftTrigger2 => "LT",
        gilrs::Button::RightTrigger2 => "RT",
        gilrs::Button::DPadUp => "DPadUp",
        gilrs::Button::DPadDown => "DPadDown",
        gilrs::Button::DPadLeft => "DPadLeft",
        gilrs::Button::DPadRight => "DPadRight",
        gilrs::Button::Select => "Options",
        gilrs::Button::Start => "Menu",
        gilrs::Button::LeftThumb => "L3",
        gilrs::Button::RightThumb => "R3",
        _ => "Unknown",
    }
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
                    events.push(GamepadEvent::Connected { name });
                    events.push(GamepadEvent::ButtonPressed {
                        button: gilrs_button_name(btn).to_string(),
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
