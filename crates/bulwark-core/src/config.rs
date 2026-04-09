use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// App Config (app.toml)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub audio: AudioConfig,
    pub network: AppNetworkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub mode: DisplayMode,
    pub vsync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    Windowed,
    Fullscreen,
    Borderless,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisplayMode::Windowed => write!(f, "windowed"),
            DisplayMode::Fullscreen => write!(f, "fullscreen"),
            DisplayMode::Borderless => write!(f, "borderless"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNetworkConfig {
    pub default_server: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 1280,
                height: 720,
                mode: DisplayMode::Windowed,
                vsync: true,
            },
            audio: AudioConfig {
                master_volume: 1.0,
                music_volume: 0.7,
                sfx_volume: 1.0,
            },
            network: AppNetworkConfig {
                default_server: "127.0.0.1:7700".to_string(),
            },
        }
    }
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  Window: {}x{} {}",
            self.window.width, self.window.height, self.window.mode
        )?;
        writeln!(f, "  VSync: {}", self.window.vsync)?;
        writeln!(
            f,
            "  Audio: master={:.0}% music={:.0}% sfx={:.0}%",
            self.audio.master_volume * 100.0,
            self.audio.music_volume * 100.0,
            self.audio.sfx_volume * 100.0,
        )?;
        write!(f, "  Default server: {}", self.network.default_server)
    }
}

// ---------------------------------------------------------------------------
// User Config (user.toml)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub player_name: String,
    pub player_id: String,
    pub controls: ControlsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlsConfig {
    pub key_up: String,
    pub key_down: String,
    pub key_left: String,
    pub key_right: String,
    pub key_accept: String,
    pub key_cancel: String,
    pub key_rotate_cw: String,
    pub key_rotate_ccw: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            player_name: "Player".to_string(),
            player_id: uuid::Uuid::new_v4().to_string(),
            controls: ControlsConfig::default(),
        }
    }
}

impl Default for ControlsConfig {
    fn default() -> Self {
        Self {
            key_up: "Up".to_string(),
            key_down: "Down".to_string(),
            key_left: "Left".to_string(),
            key_right: "Right".to_string(),
            key_accept: "Space".to_string(),
            key_cancel: "Escape".to_string(),
            key_rotate_cw: "E".to_string(),
            key_rotate_ccw: "Q".to_string(),
        }
    }
}

impl fmt::Display for UserConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  Player: {} ({})",
            self.player_name,
            &self.player_id[..8]
        )?;
        write!(
            f,
            "  Controls: LRUD={}/{}/{}/{} accept={} cancel={} rotate={}/{}",
            self.controls.key_left,
            self.controls.key_right,
            self.controls.key_up,
            self.controls.key_down,
            self.controls.key_accept,
            self.controls.key_cancel,
            self.controls.key_rotate_cw,
            self.controls.key_rotate_ccw,
        )
    }
}

// ---------------------------------------------------------------------------
// Dedicated Config (dedicated.toml)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedicatedConfig {
    pub listen_address: String,
    pub max_rooms: u32,
    pub room_timeout_secs: u64,
    pub reconnect_timeout_secs: u64,
}

impl Default for DedicatedConfig {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0:7700".to_string(),
            max_rooms: 100,
            room_timeout_secs: 300,
            reconnect_timeout_secs: 60,
        }
    }
}

impl fmt::Display for DedicatedConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Listen: {}", self.listen_address)?;
        writeln!(f, "  Max rooms: {}", self.max_rooms)?;
        writeln!(f, "  Room timeout: {}s", self.room_timeout_secs)?;
        write!(f, "  Reconnect timeout: {}s", self.reconnect_timeout_secs)
    }
}

// ---------------------------------------------------------------------------
// Loading helpers
// ---------------------------------------------------------------------------

/// Load a config from a TOML file, returning the default if the file doesn't exist.
pub fn load_config<T>(path: &Path) -> T
where
    T: Default + serde::de::DeserializeOwned,
{
    match fs::read_to_string(path) {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Warning: failed to parse {}, using defaults: {}",
                    path.display(),
                    e
                );
                T::default()
            }
        },
        Err(_) => T::default(),
    }
}

/// Save a config to a TOML file.
pub fn save_config<T>(path: &Path, config: &T) -> std::io::Result<()>
where
    T: Serialize,
{
    let contents = toml::to_string_pretty(config).expect("failed to serialize config");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

/// Resolve the directory where config files live (next to the executable).
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Resolve the user config directory. On macOS, uses ~/Library/Application Support/com.bulwark.app/.
/// On other platforms, uses the directory next to the executable.
pub fn user_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            let path = PathBuf::from(home).join("Library/Application Support/com.bulwark.app");
            return path;
        }
    }
    exe_dir()
}

/// Load UserConfig, auto-generating a player ID and saving if the file doesn't exist.
pub fn load_user_config(path: &Path) -> UserConfig {
    if path.exists() {
        load_config(path)
    } else {
        let config = UserConfig::default();
        if let Err(e) = save_config(path, &config) {
            eprintln!(
                "Warning: could not save initial user config to {}: {}",
                path.display(),
                e
            );
        }
        config
    }
}
