//! **Moldura** — canonical pleme-io TUI app framework.
//!
//! Portuguese *moldura* = "frame." Bundles `egaku` + `egaku-term` +
//! `tatara-ui` + `shikumi` + `ishou-tokens` into one opinionated
//! import so every fleet TUI app (kura, hikyaku, arnes, tanken,
//! alicerce, future ones) starts from one known-good shape rather
//! than re-wiring the same stack each time.
//!
//! ## What this crate contributes vs the underlying crates
//!
//! `egaku-term` already ships the runtime ([`egaku_term::App`] trait
//! + [`egaku_term::run`] event loop) and the lifecycle wrapper
//! ([`egaku_term::Terminal`]). What this crate adds:
//!
//! 1. **Pinned versions** of the whole pleme-io TUI dep set so a
//!    consumer with one dep (`moldura`) doesn't have to think about
//!    crossterm 0.28 vs 0.29, which `tatara-ui` revision, etc.
//! 2. **Fleet runtime crates** included by default: `mimalloc`,
//!    `parking_lot`, `arc-swap`, `notify`, `tracing-subscriber` —
//!    everything every fleet TUI eventually wants.
//! 3. **[`MolduraConfig`]** — base config struct apps `#[serde(flatten)]`
//!    into theirs. Theme name, log level, mouse, refresh rate.
//! 4. **[`MolduraTheme`]** — Nord palette resolution from
//!    `ishou-tokens`. Apps read colors via `theme.fg()` /
//!    `theme.accent()` instead of re-deriving the Nord hex table.
//! 5. **[`mimalloc_init!`]** macro — one-line install of the
//!    fleet allocator.
//! 6. **[`load_app_config`]** — canonical `~/.config/<app>/<app>.yaml`
//!    loader with shikumi-style hot reload.
//! 7. **[`tracing_init`]** — common bootstrap for fleet tracing
//!    conventions.
//! 8. **Re-exports** of `egaku`, `egaku_term`, `crossterm`, `shikumi`,
//!    `tracing` so consumers don't repeat them in their own
//!    `Cargo.toml`.
//!
//! ## Hello world
//!
//! ```ignore
//! use moldura::prelude::*;
//! moldura::mimalloc_init!();
//!
//! #[derive(Clone, Copy)]
//! enum Action { Quit, Down, Up }
//!
//! struct Hello { keys: KeyMap<Action>, cursor: u16, done: bool }
//!
//! impl egaku_term::App for Hello {
//!     type Action = Action;
//!     fn keymap(&self) -> &KeyMap<Action> { &self.keys }
//!     fn handle(&mut self, a: &Action) {
//!         match a {
//!             Action::Quit => self.done = true,
//!             Action::Down => self.cursor = self.cursor.saturating_add(1),
//!             Action::Up   => self.cursor = self.cursor.saturating_sub(1),
//!         }
//!     }
//!     // Whatever the active egaku-term version exposes for cell
//!     // writes (see egaku-term::draw helpers).
//!     fn draw(&self, term: &mut egaku_term::Terminal) -> egaku_term::Result<()> {
//!         egaku_term::draw::text(term, 1, 1, "hello, moldura")?;
//!         egaku_term::draw::text(term, 1, 2 + self.cursor, "▶")?;
//!         Ok(())
//!     }
//!     fn should_quit(&self) -> bool { self.done }
//! }
//!
//! fn main() -> moldura::Result<()> {
//!     moldura::tracing_init(&Default::default());
//!     let theme = moldura::MolduraTheme::nord();
//!     tracing::info!(theme = %theme.name(), "hello starting");
//!     let mut app = Hello {
//!         keys: moldura::egaku_term::keymap! { /* … */ },
//!         cursor: 0,
//!         done: false,
//!     };
//!     moldura::egaku_term::run(&mut app)?;
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/moldura/0.1.0")]

pub mod config;
pub mod prelude;
pub mod theme;

#[cfg(feature = "ratatui")]
pub mod ratatui;

// ── Re-exports — consumers reach for these via `moldura::*`.
pub use crossterm;
pub use egaku;
pub use egaku_term;
pub use shikumi;
pub use tracing;

pub use config::{default_config_path, MolduraConfig};
pub use theme::MolduraTheme;

/// Crossterm event re-export — what `egaku_term::App::on_unhandled`
/// receives.
pub type Event = crossterm::event::Event;

/// Error type used across the framework. Boxes any consumer error.
pub type Error = anyhow::Error;

/// Result alias used across the framework.
pub type Result<T> = std::result::Result<T, Error>;

/// Install `tracing-subscriber` with the fleet's standard EnvFilter
/// shape: `RUST_LOG` if set, else the [`MolduraConfig::log_level`]
/// override, else `"info"`. Idempotent — safe to call multiple
/// times (a second invocation is a no-op).
pub fn tracing_init(cfg: &MolduraConfig) {
    use tracing_subscriber::EnvFilter;
    let fallback = cfg.log_level.as_deref().unwrap_or("info").to_string();
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(fallback));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init();
}

/// Load + parse the operator's config for an app. Reads
/// `~/.config/<app>/<app>.yaml` (XDG-aware, env-var-overridable);
/// returns `T::default()` when the file is absent and falls back to
/// `T::default()` with a warning when parsing fails — never panics
/// on a malformed config so an operator with a syntactically broken
/// file still gets a running TUI.
pub fn load_app_config<T>(app: &str) -> T
where
    T: Default + serde::de::DeserializeOwned,
{
    let path = default_config_path(app);
    if !path.exists() {
        tracing::info!(?path, app, "moldura: no config — using defaults");
        return T::default();
    }
    match std::fs::read_to_string(&path).and_then(|s| {
        serde_yaml_ng::from_str::<T>(&s)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }) {
        Ok(cfg) => {
            tracing::info!(?path, app, "moldura: config loaded");
            cfg
        }
        Err(e) => {
            tracing::warn!(?path, app, error = %e, "moldura: config parse failed — using defaults");
            T::default()
        }
    }
}

/// Install `mimalloc` as the global allocator. Call this **once** at
/// the top of your binary's `main.rs` (outside any module):
///
/// ```ignore
/// moldura::mimalloc_init!();
///
/// fn main() -> moldura::Result<()> { /* ... */ }
/// ```
///
/// Macro form rather than a direct re-export so consumers don't have
/// to write the `#[global_allocator]` attribute themselves and so
/// there's no risk of conflicting `static GLOBAL` definitions across
/// the dependency tree.
#[macro_export]
macro_rules! mimalloc_init {
    () => {
        #[global_allocator]
        static MOLDURA_GLOBAL: $crate::__mimalloc_reexport::MiMalloc =
            $crate::__mimalloc_reexport::MiMalloc;
    };
}

#[doc(hidden)]
pub mod __mimalloc_reexport {
    pub use mimalloc::MiMalloc;
}
