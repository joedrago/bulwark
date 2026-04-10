use super::AppState;
use crate::ui;
use crate::{ConfigContext, FrameInput};
use bulwark_core::config::DisplayMode;
use bulwark_core::input::InputAction;
use macroquad::prelude::*;

const RESOLUTIONS: &[(u32, u32)] = &[
    (1280, 720),
    (1366, 768),
    (1600, 900),
    (1920, 1080),
    (2560, 1440),
    (3840, 2160),
];

fn available_display_modes() -> &'static [DisplayMode] {
    if cfg!(target_os = "macos") {
        &[DisplayMode::Windowed, DisplayMode::Fullscreen]
    } else {
        &[
            DisplayMode::Windowed,
            DisplayMode::Fullscreen,
            DisplayMode::Borderless,
        ]
    }
}

fn display_mode_index(mode: &DisplayMode) -> usize {
    let modes = available_display_modes();
    modes
        .iter()
        .position(|m| std::mem::discriminant(m) == std::mem::discriminant(mode))
        .unwrap_or(0)
}

/// Which picker is currently expanded (if any).
#[derive(PartialEq)]
enum ExpandedPicker {
    None,
    Resolution,
    DisplayMode,
}

const ITEMS: &[&str] = &["Resolution", "Display Mode", "Apply", "Back"];

pub struct GraphicsSettingsState {
    selected: usize,
    res_index: usize,
    mode_index: usize,
    expanded: ExpandedPicker,
    picker_cursor: usize,
    initialized: bool,
}

impl GraphicsSettingsState {
    pub fn new() -> Self {
        Self {
            selected: 0,
            res_index: 0,
            mode_index: 0,
            expanded: ExpandedPicker::None,
            picker_cursor: 0,
            initialized: false,
        }
    }

    fn init_from_config(&mut self, ctx: &ConfigContext) {
        let w = ctx.app_config.window.width;
        let h = ctx.app_config.window.height;
        self.res_index = RESOLUTIONS
            .iter()
            .position(|&(rw, rh)| rw == w && rh == h)
            .unwrap_or(0);
        self.mode_index = display_mode_index(&ctx.app_config.window.mode);
        self.initialized = true;
    }

    pub fn update(
        &mut self,
        _dt: f32,
        input: &FrameInput,
        ctx: &mut ConfigContext,
    ) -> Option<AppState> {
        if !self.initialized {
            self.init_from_config(ctx);
        }

        // If a picker is expanded, handle it separately
        if self.expanded != ExpandedPicker::None {
            return self.update_picker(input);
        }

        // Mouse hover on main items
        let (mx, my) = input.mouse_pos;
        for (i, _) in ITEMS.iter().enumerate() {
            let y = 200.0 + i as f32 * 44.0;
            if my >= y - 20.0 && my <= y + 8.0 && mx >= 100.0 && mx <= screen_width() - 100.0 {
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
                        self.selected = ITEMS.len() - 1;
                    }
                }
                InputAction::Down => {
                    self.selected = (self.selected + 1) % ITEMS.len();
                }
                InputAction::Accept => match self.selected {
                    0 => {
                        self.expanded = ExpandedPicker::Resolution;
                        self.picker_cursor = self.res_index;
                    }
                    1 => {
                        self.expanded = ExpandedPicker::DisplayMode;
                        self.picker_cursor = self.mode_index;
                    }
                    2 => self.apply(ctx),
                    3 => {
                        return Some(AppState::Settings(super::settings::SettingsState::new()));
                    }
                    _ => {}
                },
                InputAction::Cancel => {
                    return Some(AppState::Settings(super::settings::SettingsState::new()));
                }
                _ => {}
            }
        }
        None
    }

    fn update_picker(&mut self, input: &FrameInput) -> Option<AppState> {
        let count = match self.expanded {
            ExpandedPicker::Resolution => RESOLUTIONS.len(),
            ExpandedPicker::DisplayMode => available_display_modes().len(),
            ExpandedPicker::None => return None,
        };

        // Mouse hover on picker items
        let (mx, my) = input.mouse_pos;
        let base_y = self.picker_base_y();
        for i in 0..count {
            let y = base_y + i as f32 * 32.0;
            if my >= y - 16.0 && my <= y + 8.0 && mx >= 100.0 && mx <= screen_width() - 100.0 {
                self.picker_cursor = i;
                break;
            }
        }

        for action in &input.actions {
            match action {
                InputAction::Up => {
                    if self.picker_cursor > 0 {
                        self.picker_cursor -= 1;
                    } else {
                        self.picker_cursor = count - 1;
                    }
                }
                InputAction::Down => {
                    self.picker_cursor = (self.picker_cursor + 1) % count;
                }
                InputAction::Accept => {
                    match self.expanded {
                        ExpandedPicker::Resolution => self.res_index = self.picker_cursor,
                        ExpandedPicker::DisplayMode => self.mode_index = self.picker_cursor,
                        ExpandedPicker::None => {}
                    }
                    self.expanded = ExpandedPicker::None;
                }
                InputAction::Cancel => {
                    self.expanded = ExpandedPicker::None;
                }
                _ => {}
            }
        }
        None
    }

    fn picker_base_y(&self) -> f32 {
        let row_y = 200.0 + self.selected as f32 * 44.0;
        row_y + 28.0
    }

    fn apply(&self, ctx: &mut ConfigContext) {
        let (w, h) = RESOLUTIONS[self.res_index];
        let modes = available_display_modes();
        ctx.app_config.window.width = w;
        ctx.app_config.window.height = h;
        ctx.app_config.window.mode = modes[self.mode_index].clone();
        ctx.save_app_config();

        let fullscreen = !matches!(modes[self.mode_index], DisplayMode::Windowed);
        set_fullscreen(fullscreen);
        if !fullscreen {
            request_new_screen_size(w as f32, h as f32);
        }
    }

    pub fn draw(&self) {
        clear_background(ui::BG_COLOR);
        ui::draw_title("GRAPHICS", 48.0, 100.0, ui::GOLD);

        let (rw, rh) = RESOLUTIONS[self.res_index];
        let modes = available_display_modes();
        let values: [String; 4] = [
            format!("{}x{}", rw, rh),
            format!("{}", modes[self.mode_index]),
            String::new(),
            String::new(),
        ];

        let size = 28.0;
        let spacing = 44.0;
        let start_y = 200.0;
        let label_x = screen_width() / 2.0 - 180.0;
        let value_x = screen_width() / 2.0 + 20.0;

        for (i, item) in ITEMS.iter().enumerate() {
            let y = start_y + i as f32 * spacing;
            let color = if i == self.selected {
                WHITE
            } else {
                Color::new(0.6, 0.6, 0.6, 1.0)
            };

            if i == self.selected && self.expanded == ExpandedPicker::None {
                draw_rectangle(
                    label_x - 16.0,
                    y - 20.0,
                    screen_width() - 2.0 * (label_x - 16.0),
                    32.0,
                    Color::new(0.3, 0.3, 0.5, 0.6),
                );
                draw_text(">", label_x - 26.0, y, size, ui::GOLD);
            }

            draw_text(item, label_x, y, size, color);
            if !values[i].is_empty() {
                draw_text(&values[i], value_x, y, size, color);
            }
        }

        // Draw expanded picker overlay
        if self.expanded != ExpandedPicker::None {
            self.draw_picker(value_x);
        }
    }

    fn draw_picker(&self, x: f32) {
        let base_y = self.picker_base_y();
        let items: Vec<String> = match self.expanded {
            ExpandedPicker::Resolution => RESOLUTIONS
                .iter()
                .map(|(w, h)| format!("{}x{}", w, h))
                .collect(),
            ExpandedPicker::DisplayMode => available_display_modes()
                .iter()
                .map(|m| format!("{}", m))
                .collect(),
            ExpandedPicker::None => return,
        };

        let current = match self.expanded {
            ExpandedPicker::Resolution => self.res_index,
            ExpandedPicker::DisplayMode => self.mode_index,
            ExpandedPicker::None => 0,
        };

        let item_h = 32.0;
        let pad = 8.0;
        let panel_h = items.len() as f32 * item_h + pad * 2.0;
        let panel_w = 240.0;

        // Background panel
        draw_rectangle(
            x - pad,
            base_y - 18.0,
            panel_w,
            panel_h,
            Color::new(0.12, 0.12, 0.18, 0.95),
        );
        draw_rectangle_lines(
            x - pad,
            base_y - 18.0,
            panel_w,
            panel_h,
            2.0,
            Color::new(0.4, 0.4, 0.6, 1.0),
        );

        let small_size = 24.0;
        for (i, label) in items.iter().enumerate() {
            let y = base_y + i as f32 * item_h;

            if i == self.picker_cursor {
                draw_rectangle(
                    x - pad + 2.0,
                    y - 16.0,
                    panel_w - 4.0,
                    item_h - 2.0,
                    Color::new(0.3, 0.3, 0.5, 0.8),
                );
                draw_text(label, x, y, small_size, WHITE);
            } else if i == current {
                draw_text(label, x, y, small_size, ui::GOLD);
            } else {
                draw_text(label, x, y, small_size, Color::new(0.6, 0.6, 0.6, 1.0));
            }
        }
    }
}
