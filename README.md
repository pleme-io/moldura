# moldura

Portuguese: *moldura* = "frame." The canonical pleme-io TUI app framework.

`moldura` is the **fleet-wide preset** that bundles the existing
pleme-io TUI stack (`egaku` widgets, `egaku-term` renderer,
`tatara-ui` sigils, `shikumi` live config, `ishou-tokens` palette)
into one opinionated import. Every TUI app in the fleet — kura,
hikyaku, arnes, tanken, alicerce, the future tear/mado TUI
components — should depend on `moldura` rather than wiring those
crates independently.

## Why a meta-crate?

`egaku-term` already provides the lifecycle + event loop + widget
drawers + keymap macros. But every consumer was still independently
re-deciding:

1. Crossterm version (the fleet had a 0.28 / 0.29 split before this
   crate; moldura pins 0.29).
2. Whether to use shikumi for live config (every TUI in the fleet
   should — operators learn one config story).
3. How to translate ishou-tokens palette into egaku Styles (every
   TUI re-derived the Nord palette table).
4. Whether to install `mimalloc` (yes — same fleet allocator as
   mado / tear).
5. Whether to use parking_lot (yes).
6. How to bootstrap `tracing-subscriber` with EnvFilter (every TUI
   had a slight variation).
7. Sync vs tokio.

Moldura makes those seven decisions once. Consumers add one dep
(`moldura`), implement `MolduraApp`, and call `moldura::run()`.

## Quick start

```toml
[dependencies]
moldura = "0.1"
```

```rust
use moldura::prelude::*;

moldura::mimalloc_init!();

struct Hello {
    cursor: u16,
}

impl MolduraApp for Hello {
    fn name(&self) -> &str { "hello" }

    fn handle_event(&mut self, ev: &moldura::Event) -> moldura::Flow {
        use moldura::crossterm::event::{Event, KeyCode};
        let Event::Key(k) = ev else { return Flow::NoRedraw };
        match k.code {
            KeyCode::Char('q') => Flow::Quit,
            KeyCode::Down => { self.cursor = self.cursor.saturating_add(1); Flow::Continue }
            KeyCode::Up   => { self.cursor = self.cursor.saturating_sub(1); Flow::Continue }
            _ => Flow::NoRedraw,
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>) -> moldura::Result<()> {
        f.print_at(1, 1, "hello, moldura");
        f.print_at(1, 2 + self.cursor, "▶");
        Ok(())
    }
}

fn main() -> moldura::Result<()> {
    moldura::run(Hello { cursor: 0 })
}
```

## What's in the crate

| Module | What it does |
|---|---|
| `app::MolduraApp` | The app trait — three required methods (`name`, `handle_event`, `draw`). |
| `app::Flow` | Control-flow signal (`Continue`/`NoRedraw`/`Quit`). |
| `config::MolduraConfig` | Shared base config (theme, log level, mouse, refresh interval). App-specific configs flatten this. |
| `config::default_config_path()` | `~/.config/<app>/<app>.yaml` resolver (XDG-aware, env-var-overridable). |
| `theme::MolduraTheme` | Theme resolved from ishou-tokens (Nord by default; unknown names fall back with a warning). |
| `frame::Frame` | Per-redraw drawing surface: `print_at`, `fill_rect`, `theme()`, `size()`. |
| `mimalloc_init!()` | One-line macro to install `#[global_allocator] mimalloc`. |
| `run()` / `run_with()` | Synchronous runtime entry. |
| `run_async()` / `run_async_with()` | Tokio runtime (behind `tokio` feature). |
| `ratatui::*` (optional) | ratatui interop — re-exports `ratatui` + `ratatui-image` + a `render_ratatui_widget` adapter for apps that want charts/sparklines/Kitty graphics on top of the egaku-term frame. |

## Re-exports

`moldura::{egaku, egaku_term, crossterm, shikumi, tracing}` —
consumers don't need to repeat these in their own `Cargo.toml`.

## Feature flags

- `sync` (default) — synchronous event loop only.
- `tokio` — async runtime with `crossterm/event-stream` +
  `tokio::select!`. Required if your app wants to interleave
  crossterm events with custom futures.
- `ratatui` — pulls in ratatui + ratatui-image for the interop
  module. Off by default because most fleet TUIs don't need it.

## Why not ratatui directly?

The Rust ecosystem's most popular TUI library is
[ratatui](https://ratatui.rs/) — and it's genuinely excellent. But
pleme-io's gravity is in **egaku**, the state-machine widget toolkit
that GPU apps (mado, hibikine, kagibako) and TUI apps share. Switching
to ratatui would split the fleet's widget model in two — a tax that
compounds with every new widget. We keep egaku as the canonical
widget primitive and bridge to ratatui only where its ecosystem
offers something egaku doesn't (charts / sparklines / Kitty
graphics) via the optional `ratatui` feature.

## Status

M0 — workspace compiles, hello-world example runs end-to-end. The
ratatui bridge is a typed placeholder pending an upstream
`egaku-term::Buffer::as_ratatui_buffer` addition (see
`src/ratatui.rs` for the M1 plan).

## License

MIT.
