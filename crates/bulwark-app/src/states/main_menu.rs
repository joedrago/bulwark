use super::AppState;
use crate::ui::{self, Menu, MenuAction};
use crate::FrameInput;

const MENU_ITEMS: &[&str] = &[
    "New Game (Local)",
    "New Game (Multiplayer)",
    "Join Game",
    "Settings",
    "Quit",
];

pub struct MainMenuState {
    menu: Menu,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(MENU_ITEMS, 32.0, 48.0, 240.0),
        }
    }

    pub fn update(&mut self, _dt: f32, input: &FrameInput) -> Option<AppState> {
        match self.menu.update(input.mouse_pos, &input.actions) {
            MenuAction::Activated(i) => self.activate(i),
            _ => None,
        }
    }

    fn activate(&self, index: usize) -> Option<AppState> {
        match index {
            3 => Some(AppState::Settings(super::settings::SettingsState::new())),
            4 => std::process::exit(0),
            _ => None, // Stubs for now
        }
    }

    pub fn draw(&self) {
        use macroquad::prelude::*;
        clear_background(ui::BG_COLOR);
        ui::draw_title("BULWARK", 60.0, 120.0, ui::GOLD);
        self.menu.draw();
    }
}
