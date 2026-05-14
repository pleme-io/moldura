//! `use moldura::prelude::*;` brings the canonical fleet TUI types
//! into scope in one line.

pub use crate::{
    config::{default_config_path, MolduraConfig},
    theme::MolduraTheme,
    Event, Result,
};

// egaku-term's runtime contract — every fleet TUI app implements
// `egaku_term::App` and dispatches keymap actions through it.
pub use egaku_term::{App as EgakuApp, Terminal};

// egaku widget vocabulary — Rect, KeyCombo, KeyMap, etc.
pub use egaku::{KeyCombo, KeyMap, Rect};
