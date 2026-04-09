pub mod main_menu;
pub mod settings;
pub mod splash;

use crate::FrameInput;

/// Top-level application state.
pub enum AppState {
    Splash(splash::SplashState),
    MainMenu(main_menu::MainMenuState),
    Settings(settings::SettingsState),
}

impl AppState {
    /// Update the current state. Returns Some(new_state) on transition.
    pub fn update(&mut self, dt: f32, input: &FrameInput) -> Option<AppState> {
        match self {
            AppState::Splash(s) => s.update(dt, input),
            AppState::MainMenu(s) => s.update(dt, input),
            AppState::Settings(s) => s.update(dt, input),
        }
    }

    /// Draw the current state.
    pub fn draw(&self) {
        match self {
            AppState::Splash(s) => s.draw(),
            AppState::MainMenu(s) => s.draw(),
            AppState::Settings(s) => s.draw(),
        }
    }
}
