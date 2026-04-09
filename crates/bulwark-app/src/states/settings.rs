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
            MenuAction::Activated(i) => {
                if i == SETTINGS_ITEMS.len() - 1 {
                    return Some(AppState::MainMenu(super::main_menu::MainMenuState::new()));
                }
                None // Other items are stubs for now
            }
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
