//! Theme — Nord palette resolution + ishou-tokens integration.
//!
//! Every moldura app gets the canonical Nord theme by default. The
//! palette comes from `ishou-tokens` so a fleet-wide colorway change
//! is one ishou edit. Apps that want a custom theme implement
//! [`MolduraTheme::custom`] or compose their own resolver.

use crate::config::MolduraConfig;

/// Resolved theme — the result of asking ishou-tokens for the named
/// palette + caching it. The runtime hands this to each [`crate::Frame`]
/// so apps can read colors without re-resolving per frame.
#[derive(Clone, Debug)]
pub struct MolduraTheme {
    name: String,
    // Concrete u8 RGB values for each semantic role. Cached at
    // construction so the per-frame draw path is a struct read, not
    // a fresh ishou lookup.
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
    accent: (u8, u8, u8),
    warning: (u8, u8, u8),
    error: (u8, u8, u8),
    success: (u8, u8, u8),
    muted: (u8, u8, u8),
}

impl MolduraTheme {
    /// Construct a theme from the operator's MolduraConfig. Looks
    /// up the named palette in ishou-tokens; falls back to Nord on
    /// unknown names.
    #[must_use]
    pub fn from_config(cfg: &MolduraConfig) -> Self {
        Self::by_name(&cfg.theme)
    }

    /// Look up a named theme. Built-in: `"nord"`. Unknown names log a
    /// warning and resolve to Nord — the safe default for the fleet.
    #[must_use]
    pub fn by_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "nord" => Self::nord(),
            other => {
                tracing::warn!(theme = other, "moldura: unknown theme — falling back to nord");
                Self::nord()
            }
        }
    }

    /// The canonical Nord theme. Mirrors `ishou-tokens` Nord palette
    /// — when ishou bumps its Nord defs, these constants update via
    /// the next moldura release.
    #[must_use]
    pub fn nord() -> Self {
        Self {
            name: "nord".into(),
            // Snow Storm 3 (foreground)
            fg: (0xec, 0xef, 0xf4),
            // Polar Night 0 (background)
            bg: (0x2e, 0x34, 0x40),
            // Frost 2 (accent)
            accent: (0x88, 0xc0, 0xd0),
            // Aurora yellow (warning)
            warning: (0xeb, 0xcb, 0x8b),
            // Aurora red (error)
            error: (0xbf, 0x61, 0x6a),
            // Aurora green (success)
            success: (0xa3, 0xbe, 0x8c),
            // Polar Night 3 (muted)
            muted: (0x4c, 0x56, 0x6a),
        }
    }

    /// Author a custom palette inline — used by apps that want a
    /// non-Nord theme without touching ishou-tokens upstream.
    #[must_use]
    pub fn custom(name: impl Into<String>, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Self {
        let mut t = Self::nord();
        t.name = name.into();
        t.fg = fg;
        t.bg = bg;
        t
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn fg(&self) -> (u8, u8, u8) {
        self.fg
    }
    #[must_use]
    pub fn bg(&self) -> (u8, u8, u8) {
        self.bg
    }
    #[must_use]
    pub fn accent(&self) -> (u8, u8, u8) {
        self.accent
    }
    #[must_use]
    pub fn warning(&self) -> (u8, u8, u8) {
        self.warning
    }
    #[must_use]
    pub fn error(&self) -> (u8, u8, u8) {
        self.error
    }
    #[must_use]
    pub fn success(&self) -> (u8, u8, u8) {
        self.success
    }
    #[must_use]
    pub fn muted(&self) -> (u8, u8, u8) {
        self.muted
    }
}

impl Default for MolduraTheme {
    fn default() -> Self {
        Self::nord()
    }
}
