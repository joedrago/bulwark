use bulwark_core::input::InputAction;
use macroquad::prelude::*;

/// A reusable vertical menu widget.
pub struct Menu {
    pub items: &'static [&'static str],
    pub selected: usize,
    pub item_size: f32,
    pub item_spacing: f32,
    pub start_y: f32,
}

/// Result of a menu update: which item was activated (if any).
pub enum MenuAction {
    None,
    Activated(usize),
    Cancel,
}

impl Menu {
    pub fn new(
        items: &'static [&'static str],
        item_size: f32,
        item_spacing: f32,
        start_y: f32,
    ) -> Self {
        Self {
            items,
            selected: 0,
            item_size,
            item_spacing,
            start_y,
        }
    }

    /// Update selection from mouse position and input actions. Returns what happened.
    pub fn update(&mut self, mouse_pos: (f32, f32), actions: &[InputAction]) -> MenuAction {
        // Mouse hover
        let (mx, my) = mouse_pos;
        for (i, item) in self.items.iter().enumerate() {
            let y = self.start_y + i as f32 * self.item_spacing;
            let dims = measure_text(item, None, self.item_size as u16, 1.0);
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

        for action in actions {
            match action {
                InputAction::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = self.items.len() - 1;
                    }
                }
                InputAction::Down => {
                    self.selected = (self.selected + 1) % self.items.len();
                }
                InputAction::Accept => {
                    return MenuAction::Activated(self.selected);
                }
                InputAction::Cancel => {
                    return MenuAction::Cancel;
                }
                _ => {}
            }
        }

        MenuAction::None
    }

    /// Draw the menu items centered on screen.
    pub fn draw(&self) {
        for (i, item) in self.items.iter().enumerate() {
            let y = self.start_y + i as f32 * self.item_spacing;
            let dims = measure_text(item, None, self.item_size as u16, 1.0);
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
                    x - 30.0,
                    y,
                    self.item_size,
                    Color::new(0.9, 0.8, 0.5, 1.0),
                );
                draw_text(item, x, y, self.item_size, WHITE);
            } else {
                draw_text(item, x, y, self.item_size, Color::new(0.6, 0.6, 0.6, 1.0));
            }
        }
    }
}

/// Draw a centered title.
pub fn draw_title(text: &str, size: f32, y: f32, color: Color) {
    let dims = measure_text(text, None, size as u16, 1.0);
    draw_text(text, (screen_width() - dims.width) / 2.0, y, size, color);
}

/// Standard dark background color.
pub const BG_COLOR: Color = Color::new(0.08, 0.08, 0.12, 1.0);

/// Gold accent color used for titles and selection arrows.
pub const GOLD: Color = Color::new(0.9, 0.8, 0.5, 1.0);
