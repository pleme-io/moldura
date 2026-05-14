//! Shared base config for moldura apps.
//!
//! Apps that need MORE knobs (kura wants Ghostty detection toggles,
//! arnes wants probe defaults, etc.) compose this struct via serde
//! `#[serde(flatten)]`. The CANONICAL knobs every TUI cares about
//! live here so operators learn them once.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Fleet-shared base config. Every moldura app's TOP-LEVEL config
/// struct flattens or composes this.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MolduraConfig {
    /// Active theme. By default `"nord"` — pulls from ishou-tokens'
    /// `PaletteV2::nord_*`. Other valid values map to other ishou
    /// palettes if/when the operator authors them.
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Override the default tracing-subscriber filter. `info` is the
    /// fleet default; operators can set `"debug"` or
    /// `"<crate>=debug"` patterns.
    #[serde(default)]
    pub log_level: Option<String>,

    /// Hot-reload watcher debounce in ms. Same default as tear /
    /// mado (250 ms). Operators rarely tune this.
    #[serde(default = "default_debounce")]
    pub reload_debounce_ms: u64,

    /// Status-bar refresh interval in seconds. Long-running TUIs
    /// with periodic status indicators (clock, build-status, etc.)
    /// drive their refresh from this.
    #[serde(default = "default_refresh")]
    pub refresh_interval_seconds: u32,

    /// Whether to enable mouse capture. Most fleet TUIs want this
    /// on (atuin / lazygit-style); some power-user TUIs want it off
    /// so terminal-native copy/select works.
    #[serde(default = "default_mouse")]
    pub mouse: bool,

    /// Optional path override for the app's config file. Useful for
    /// tests and for sandboxed environments where
    /// `$XDG_CONFIG_HOME` isn't reachable.
    #[serde(default)]
    pub config_path_override: Option<PathBuf>,
}

impl Default for MolduraConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            log_level: None,
            reload_debounce_ms: default_debounce(),
            refresh_interval_seconds: default_refresh(),
            mouse: default_mouse(),
            config_path_override: None,
        }
    }
}

fn default_theme() -> String {
    "nord".into()
}
fn default_debounce() -> u64 {
    250
}
fn default_refresh() -> u32 {
    5
}
fn default_mouse() -> bool {
    true
}

/// Resolve a canonical config path for an app.
/// `~/.config/<app>/<app>.yaml`, honouring `$XDG_CONFIG_HOME` and
/// `$<APP>_CONFIG_FILE`.
#[must_use]
pub fn default_config_path(app: &str) -> PathBuf {
    let env_var = format!("{}_CONFIG_FILE", app.to_ascii_uppercase());
    if let Ok(explicit) = std::env::var(&env_var) {
        return PathBuf::from(explicit);
    }
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .ok()
        .or_else(|| {
            std::env::var("HOME").ok().map(|h| {
                let mut p = PathBuf::from(h);
                p.push(".config");
                p
            })
        })
        .unwrap_or_else(|| PathBuf::from("."));
    xdg.join(app).join(format!("{app}.yaml"))
}
