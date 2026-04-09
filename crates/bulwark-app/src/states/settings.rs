use super::AppState;
use crate::FrameInput;
use bulwark_core::input::InputAction;
use macroquad::prelude::*;

const SETTINGS_ITEMS: &[&str] = &["Graphics", "Audio", "Controls", "Back"];
const SETTINGS_ITEM_SIZE: f32 = 28.0;
const SETTINGS_ITEM_SPACING: f32 = 44.0;
const SETTINGS_START_Y: f32 = 200.0;

pub struct SettingsState {
    selected: usize,
}

impl SettingsState {
    pub fn new() -> Self {
        Self { selected: 0 }
    }

    pub fn update(&mut self, _dt: f32, input: &FrameInput) -> Option<AppState> {
        // Mouse hover updates selection
        let (mx, my) = input.mouse_pos;
        for (i, item) in SETTINGS_ITEMS.iter().enumerate() {
            let y = SETTINGS_START_Y + i as f32 * SETTINGS_ITEM_SPACING;
            let dims = measure_text(item, None, SETTINGS_ITEM_SIZE as u16, 1.0);
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
                        self.selected = SETTINGS_ITEMS.len() - 1;
                    }
                }
                InputAction::Down => {
                    self.selected = (self.selected + 1) % SETTINGS_ITEMS.len();
                }
                InputAction::Accept => {
                    if self.selected == SETTINGS_ITEMS.len() - 1 {
                        return Some(AppState::MainMenu(super::main_menu::MainMenuState::new()));
                    }
                    // Other items are stubs for now
                }
                InputAction::Cancel => {
                    return Some(AppState::MainMenu(super::main_menu::MainMenuState::new()));
                }
                _ => {}
            }
        }
        None
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.08, 0.08, 0.12, 1.0));

        // Title
        let title = "SETTINGS";
        let title_size = 48.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            (screen_width() - title_dims.width) / 2.0,
            100.0,
            title_size,
            Color::new(0.9, 0.8, 0.5, 1.0),
        );

        // Menu items
        for (i, item) in SETTINGS_ITEMS.iter().enumerate() {
            let y = SETTINGS_START_Y + i as f32 * SETTINGS_ITEM_SPACING;
            let dims = measure_text(item, None, SETTINGS_ITEM_SIZE as u16, 1.0);
            let x = (screen_width() - dims.width) / 2.0;

            if i == self.selected {
                draw_rectangle(
                    x - 16.0,
                    y - dims.height - 4.0,
                    dims.width + 32.0,
                    dims.height + 12.0,
                    Color::new(0.3, 0.3, 0.5, 0.6),
                );
                draw_text(
                    ">",
                    x - 26.0,
                    y,
                    SETTINGS_ITEM_SIZE,
                    Color::new(0.9, 0.8, 0.5, 1.0),
                );
                draw_text(item, x, y, SETTINGS_ITEM_SIZE, WHITE);
            } else {
                draw_text(
                    item,
                    x,
                    y,
                    SETTINGS_ITEM_SIZE,
                    Color::new(0.6, 0.6, 0.6, 1.0),
                );
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
