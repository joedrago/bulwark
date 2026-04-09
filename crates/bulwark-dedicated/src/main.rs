use bulwark_core::config;

fn main() {
    println!("Bulwark Dedicated Server v{}", bulwark_core::VERSION);

    let exe_dir = config::exe_dir();

    let dedicated_config: config::DedicatedConfig =
        config::load_config(&exe_dir.join("dedicated.toml"));

    println!("Dedicated config ({}/):", exe_dir.display());
    println!("{dedicated_config}");
}
