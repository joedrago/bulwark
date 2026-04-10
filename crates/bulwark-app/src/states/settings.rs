use super::AppState;
use crate::ui::{self, Menu, MenuAction};
use crate::FrameInput;

const SETTINGS_ITEMS: &[&str] = &["Graphics", "Audio", "Controls", "Back"];

pub struct SettingsState {
    menu: Menu,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(SETTINGS_ITEMS, 28.0, 44.0, 200.0),
        }
    }

    pub fn update(&mut self, _dt: f32, input: &FrameInput) -> Option<AppState> {
        match self.menu.update(input.mouse_pos, &input.actions) {
            MenuAction::Activated(i) => match i {
                0 => Some(AppState::SettingsGraphics(
                    super::settings_graphics::GraphicsSettingsState::new(),
                )),
                1 => Some(AppState::SettingsAudio(
                    super::settings_audio::AudioSettingsState::new(),
                )),
                2 => Some(AppState::SettingsControls(
                    super::settings_controls::ControlsSettingsState::new(),
                )),
                3 => Some(AppState::MainMenu(super::main_menu::MainMenuState::new())),
                _ => None,
            },
            MenuAction::Cancel => Some(AppState::MainMenu(super::main_menu::MainMenuState::new())),
            MenuAction::None => None,
        }
    }

    pub fn draw(&self) {
        use macroquad::prelude::*;
        clear_background(ui::BG_COLOR);
        ui::draw_title("SETTINGS", 48.0, 100.0, ui::GOLD);
        self.menu.draw();
    }
}
