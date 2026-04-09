use bulwark_core::config::{self, AppConfig, DisplayMode, UserConfig};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    let exe_dir = config::exe_dir();
    let app_config: AppConfig = config::load_config(&exe_dir.join("app.toml"));

    Conf {
        window_title: "Bulwark".to_string(),
        window_width: app_config.window.width as i32,
        window_height: app_config.window.height as i32,
        fullscreen: matches!(app_config.window.mode, DisplayMode::Fullscreen),
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let exe_dir = config::exe_dir();
    let user_dir = config::user_config_dir();

    let app_config: AppConfig = config::load_config(&exe_dir.join("app.toml"));
    let user_config: UserConfig = config::load_user_config(&user_dir.join("user.toml"));

    println!("Bulwark App v{}", bulwark_core::VERSION);
    println!("App config:\n{app_config}");
    println!("User config:\n{user_config}");

    // Ocean blue background color
    let ocean_blue = Color::new(0.15, 0.35, 0.65, 1.0);

    // Test grid settings
    let grid_cols = 40;
    let grid_rows = 30;
    let grid_color_ground = Color::new(0.35, 0.55, 0.25, 1.0);
    let grid_color_water = Color::new(0.18, 0.40, 0.70, 1.0);
    let grid_line_color = Color::new(0.0, 0.0, 0.0, 0.15);

    loop {
        clear_background(ocean_blue);

        // Calculate grid cell size to fit the window with some padding
        let padding = 40.0;
        let available_w = screen_width() - padding * 2.0;
        let available_h = screen_height() - padding * 2.0;
        let cell_size = (available_w / grid_cols as f32).min(available_h / grid_rows as f32);

        // Center the grid
        let grid_w = cell_size * grid_cols as f32;
        let grid_h = cell_size * grid_rows as f32;
        let offset_x = (screen_width() - grid_w) / 2.0;
        let offset_y = (screen_height() - grid_h) / 2.0;

        // Draw grid cells
        for row in 0..grid_rows {
            for col in 0..grid_cols {
                let x = offset_x + col as f32 * cell_size;
                let y = offset_y + row as f32 * cell_size;

                // Simple test pattern: water border, ground interior
                let is_water = row == 0
                    || row == grid_rows - 1
                    || col == 0
                    || col == grid_cols - 1
                    || (row < 3 && col < 5)
                    || (row > grid_rows - 4 && col > grid_cols - 6);

                let color = if is_water {
                    grid_color_water
                } else {
                    grid_color_ground
                };

                draw_rectangle(x, y, cell_size, cell_size, color);
            }
        }

        // Draw grid lines
        for row in 0..=grid_rows {
            let y = offset_y + row as f32 * cell_size;
            draw_line(offset_x, y, offset_x + grid_w, y, 1.0, grid_line_color);
        }
        for col in 0..=grid_cols {
            let x = offset_x + col as f32 * cell_size;
            draw_line(x, offset_y, x, offset_y + grid_h, 1.0, grid_line_color);
        }

        // Draw title text
        let title = format!("Bulwark v{}", bulwark_core::VERSION);
        draw_text(&title, 10.0, 24.0, 24.0, WHITE);

        // Draw window info
        let info = format!(
            "{}x{} | Grid {}x{} | Cell {:.0}px",
            screen_width() as u32,
            screen_height() as u32,
            grid_cols,
            grid_rows,
            cell_size,
        );
        draw_text(&info, 10.0, screen_height() - 10.0, 18.0, WHITE);

        next_frame().await
    }
}
