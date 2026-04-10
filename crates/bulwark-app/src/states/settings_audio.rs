use super::AppState;
use crate::ui;
use crate::{ConfigContext, FrameInput};
use bulwark_core::input::InputAction;
use macroquad::prelude::*;

const ITEMS: &[&str] = &["Master Volume", "Music Volume", "SFX Volume", "Back"];
const VOLUME_STEP: f32 = 0.05;

const SIZE: f32 = 28.0;
const SPACING: f32 = 44.0;
const START_Y: f32 = 200.0;
const BAR_W: f32 = 200.0;
const BAR_H: f32 = 16.0;

fn label_x() -> f32 {
    screen_width() / 2.0 - 180.0
}
fn bar_x() -> f32 {
    screen_width() / 2.0 + 20.0
}

pub struct AudioSettingsState {
    selected: usize,
    master: f32,
    music: f32,
    sfx: f32,
    initialized: bool,
}

impl AudioSettingsState {
    pub fn new() -> Self {
        Self {
            selected: 0,
            master: 1.0,
            music: 0.7,
            sfx: 1.0,
            initialized: false,
        }
    }

    fn init_from_config(&mut self, ctx: &ConfigContext) {
        self.master = ctx.app_config.audio.master_volume;
        self.music = ctx.app_config.audio.music_volume;
        self.sfx = ctx.app_config.audio.sfx_volume;
        self.initialized = true;
    }

    fn volume_mut(&mut self, index: usize) -> Option<&mut f32> {
        match index {
            0 => Some(&mut self.master),
            1 => Some(&mut self.music),
            2 => Some(&mut self.sfx),
            _ => None,
        }
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

        let (mx, my) = input.mouse_pos;
        let bx = bar_x();

        // Mouse hover updates selection
        for (i, _) in ITEMS.iter().enumerate() {
            let y = START_Y + i as f32 * SPACING;
            if my >= y - 20.0 && my <= y + 8.0 && mx >= 100.0 && mx <= screen_width() - 100.0 {
                self.selected = i;
                break;
            }
        }

        // Mouse click on volume bar sets volume directly
        if is_mouse_button_down(MouseButton::Left) && self.selected < 3 {
            let y = START_Y + self.selected as f32 * SPACING;
            let bar_y = y - BAR_H / 2.0 - 2.0;
            if mx >= bx && mx <= bx + BAR_W && my >= bar_y - 4.0 && my <= bar_y + BAR_H + 4.0 {
                let pct = ((mx - bx) / BAR_W).clamp(0.0, 1.0);
                if let Some(vol) = self.volume_mut(self.selected) {
                    *vol = pct;
                }
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
                InputAction::Left => {
                    if let Some(vol) = self.volume_mut(self.selected) {
                        *vol = (*vol - VOLUME_STEP).clamp(0.0, 1.0);
                    }
                }
                InputAction::Right => {
                    if let Some(vol) = self.volume_mut(self.selected) {
                        *vol = (*vol + VOLUME_STEP).clamp(0.0, 1.0);
                    }
                }
                InputAction::Accept => {
                    if self.selected == ITEMS.len() - 1 {
                        self.apply(ctx);
                        return Some(AppState::Settings(super::settings::SettingsState::new()));
                    }
                }
                InputAction::Cancel => {
                    self.apply(ctx);
                    return Some(AppState::Settings(super::settings::SettingsState::new()));
                }
                _ => {}
            }
        }
        None
    }

    fn apply(&self, ctx: &mut ConfigContext) {
        ctx.app_config.audio.master_volume = self.master;
        ctx.app_config.audio.music_volume = self.music;
        ctx.app_config.audio.sfx_volume = self.sfx;
        ctx.save_app_config();
    }

    pub fn draw(&self) {
        clear_background(ui::BG_COLOR);
        ui::draw_title("AUDIO", 48.0, 100.0, ui::GOLD);

        let lx = label_x();
        let bx = bar_x();
        let volumes = [self.master, self.music, self.sfx];

        // Right edge: after bar + percentage label
        let right_edge = bx + BAR_W + 80.0;

        for (i, item) in ITEMS.iter().enumerate() {
            let y = START_Y + i as f32 * SPACING;
            let color = if i == self.selected {
                WHITE
            } else {
                Color::new(0.6, 0.6, 0.6, 1.0)
            };

            if i == self.selected {
                draw_rectangle(
                    lx - 16.0,
                    y - 20.0,
                    right_edge - lx + 32.0,
                    32.0,
                    Color::new(0.3, 0.3, 0.5, 0.6),
                );
                draw_text(">", lx - 26.0, y, SIZE, ui::GOLD);
            }

            draw_text(item, lx, y, SIZE, color);

            if i < 3 {
                let bar_y = y - BAR_H / 2.0 - 2.0;
                // Background
                draw_rectangle(bx, bar_y, BAR_W, BAR_H, Color::new(0.2, 0.2, 0.2, 1.0));
                // Fill
                draw_rectangle(
                    bx,
                    bar_y,
                    BAR_W * volumes[i],
                    BAR_H,
                    if i == self.selected {
                        ui::GOLD
                    } else {
                        Color::new(0.5, 0.5, 0.5, 1.0)
                    },
                );
                // Percentage label
                let pct = format!("{:.0}%", volumes[i] * 100.0);
                draw_text(&pct, bx + BAR_W + 10.0, y, SIZE, color);
            }
        }
    }
}
