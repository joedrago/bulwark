use super::AppState;
use crate::FrameInput;
use bulwark_core::input::InputAction;
use macroquad::prelude::*;

const MENU_ITEMS: &[&str] = &[
    "New Game (Local)",
    "New Game (Multiplayer)",
    "Join Game",
    "Settings",
    "Quit",
];
const MENU_ITEM_SIZE: f32 = 32.0;
const MENU_ITEM_SPACING: f32 = 48.0;
const MENU_START_Y: f32 = 240.0;

pub struct MainMenuState {
    selected: usize,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self { selected: 0 }
    }

    pub fn update(&mut self, _dt: f32, input: &FrameInput) -> Option<AppState> {
        // Mouse hover updates selection
        let (mx, my) = input.mouse_pos;
        for (i, item) in MENU_ITEMS.iter().enumerate() {
            let y = MENU_START_Y + i as f32 * MENU_ITEM_SPACING;
            let dims = measure_text(item, None, MENU_ITEM_SIZE as u16, 1.0);
            let x = (screen_width() - dims.width) / 2.0;
            if mx >= x - 16.0
                && mx <= x + dims.width + 16.0
                && my >= y - dims.height - 4.0
                && my <= y + 8.0
            {
                self.selected = i;
                break;
            }
        }

        for action in &input.actions {
            match action {
                InputAction::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = MENU_ITEMS.len() - 1;
                    }
                }
                InputAction::Down => {
                    self.selected = (self.selected + 1) % MENU_ITEMS.len();
                }
                InputAction::Accept => {
                    return self.activate_selection();
                }
                _ => {}
            }
        }
        None
    }

    fn activate_selection(&self) -> Option<AppState> {
        match self.selected {
            3 => Some(AppState::Settings(super::settings::SettingsState::new())),
            4 => std::process::exit(0),
            _ => None, // Other items are stubs for now
        }
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.08, 0.08, 0.12, 1.0));

        // Title
        let title = "BULWARK";
        let title_size = 60.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            (screen_width() - title_dims.width) / 2.0,
            120.0,
            title_size,
            Color::new(0.9, 0.8, 0.5, 1.0),
        );

        // Menu items
        for (i, item) in MENU_ITEMS.iter().enumerate() {
            let y = MENU_START_Y + i as f32 * MENU_ITEM_SPACING;
            let dims = measure_text(item, None, MENU_ITEM_SIZE as u16, 1.0);
            let x = (screen_width() - dims.width) / 2.0;

            if i == self.selected {
                // Highlight background
                draw_rectangle(
                    x - 16.0,
                    y - dims.height - 4.0,
                    dims.width + 32.0,
                    dims.height + 12.0,
                    Color::new(0.3, 0.3, 0.5, 0.6),
                );
                // Selection arrow
                draw_text(
                    ">",
                    x - 30.0,
                    y,
                    MENU_ITEM_SIZE,
                    Color::new(0.9, 0.8, 0.5, 1.0),
                );
                draw_text(item, x, y, MENU_ITEM_SIZE, WHITE);
            } else {
                draw_text(item, x, y, MENU_ITEM_SIZE, Color::new(0.6, 0.6, 0.6, 1.0));
            }
        }

        // Footer
        let footer = "Up/Down: Navigate  |  Accept: Select  |  Cancel: Back";
        let footer_size = 16.0;
        let footer_dims = measure_text(footer, None, footer_size as u16, 1.0);
        draw_text(
            footer,
            (screen_width() - footer_dims.width) / 2.0,
            screen_height() - 30.0,
            footer_size,
            Color::new(0.4, 0.4, 0.4, 1.0),
        );
    }
}
