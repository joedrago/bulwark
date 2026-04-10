pub mod main_menu;
pub mod settings;
pub mod settings_audio;
pub mod settings_controls;
pub mod settings_graphics;
pub mod splash;

use crate::FrameInput;

/// Top-level application state.
pub enum AppState {
    Splash(splash::SplashState),
    MainMenu(main_menu::MainMenuState),
    Settings(settings::SettingsState),
    SettingsGraphics(settings_graphics::GraphicsSettingsState),
    SettingsAudio(settings_audio::AudioSettingsState),
    SettingsControls(settings_controls::ControlsSettingsState),
}

impl AppState {
    /// Update the current state. Returns Some(new_state) on transition.
    pub fn update(
        &mut self,
        dt: f32,
        input: &FrameInput,
        ctx: &mut crate::ConfigContext,
    ) -> Option<AppState> {
        match self {
            AppState::Splash(s) => s.update(dt, input),
            AppState::MainMenu(s) => s.update(dt, input),
            AppState::Settings(s) => s.update(dt, input),
            AppState::SettingsGraphics(s) => s.update(dt, input, ctx),
            AppState::SettingsAudio(s) => s.update(dt, input, ctx),
            AppState::SettingsControls(s) => s.update(dt, input, ctx),
        }
    }

    /// Draw the current state.
    pub fn draw(&self) {
        match self {
            AppState::Splash(s) => s.draw(),
            AppState::MainMenu(s) => s.draw(),
            AppState::Settings(s) => s.draw(),
            AppState::SettingsGraphics(s) => s.draw(),
            AppState::SettingsAudio(s) => s.draw(),
            AppState::SettingsControls(s) => s.draw(),
        }
    }
}
