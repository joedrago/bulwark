use super::AppState;
use crate::FrameInput;
use bulwark_core::input::InputAction;
use macroquad::prelude::*;

const SPLASH_DURATION: f32 = 2.0;

pub struct SplashState {
    elapsed: f32,
}

impl SplashState {
    pub fn new() -> Self {
        Self { elapsed: 0.0 }
    }

    pub fn update(&mut self, dt: f32, input: &FrameInput) -> Option<AppState> {
        self.elapsed += dt;

        // Skip splash on any Accept press
        let skip = input.actions.contains(&InputAction::Accept);

        if self.elapsed >= SPLASH_DURATION || skip {
            return Some(AppState::MainMenu(super::main_menu::MainMenuState::new()));
        }
        None
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.08, 0.08, 0.12, 1.0));

        // Fade in/out
        let alpha = if self.elapsed < 0.5 {
            self.elapsed / 0.5
        } else if self.elapsed > SPLASH_DURATION - 0.5 {
            (SPLASH_DURATION - self.elapsed) / 0.5
        } else {
            1.0
        }
        .clamp(0.0, 1.0);

        let title = "BULWARK";
        let font_size = 80.0;
        let dims = measure_text(title, None, font_size as u16, 1.0);
        let x = (screen_width() - dims.width) / 2.0;
        let y = (screen_height() + dims.height) / 2.0 - 20.0;
        draw_text(title, x, y, font_size, Color::new(0.9, 0.8, 0.5, alpha));

        let subtitle = &format!("v{}", bulwark_core::VERSION);
        let sub_size = 24.0;
        let sub_dims = measure_text(subtitle, None, sub_size as u16, 1.0);
        draw_text(
            subtitle,
            (screen_width() - sub_dims.width) / 2.0,
            y + 40.0,
            sub_size,
            Color::new(0.6, 0.6, 0.6, alpha),
        );
    }
}
