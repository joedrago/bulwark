use bulwark_core::config;

fn main() {
    println!("Bulwark App v{}", bulwark_core::VERSION);

    let exe_dir = config::exe_dir();
    let user_dir = config::user_config_dir();

    let app_config: config::AppConfig = config::load_config(&exe_dir.join("app.toml"));
    let user_config = config::load_user_config(&user_dir.join("user.toml"));

    println!("App config ({}/):", exe_dir.display());
    println!("{app_config}");
    println!("User config ({}/):", user_dir.display());
    println!("{user_config}");
}
