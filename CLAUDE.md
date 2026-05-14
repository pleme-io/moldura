# moldura — Claude Orientation

> **★★★ CSE / Knowable Construction.** This repo operates under
> **Constructive Substrate Engineering** — canonical specification at
> [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md).

One-sentence purpose: canonical pleme-io TUI app framework —
bundles `egaku` + `egaku-term` + `tatara-ui` + `shikumi` +
`ishou-tokens` into one opinionated import so every fleet TUI app
shares one lifecycle, one theme source, one config story.

## Where this sits in the fleet

| Layer | Crate | Role |
|---|---|---|
| Widget primitives | `egaku` | State-machine widget toolkit. **GPU and TUI dual-use.** |
| Terminal rendering | `egaku-term` | crossterm-backed renderer + event loop. Drop-safe lifecycle wrapper. |
| Sigils / CLI UX | `tatara-ui` | Typed palette + sigils + cache-aware renderer. |
| Live config | `shikumi` | `~/.config/<app>/<app>.yaml` parse + hot reload via `notify`. |
| Palette | `ishou-tokens` | Source of truth for Nord palette + typography. |
| **App framework** | **`moldura`** | **One import; one canonical preset. THIS crate.** |

The chain: every fleet TUI app imports `moldura`. Moldura imports
`egaku-term` (renders) + `egaku` (widgets) + `tatara-ui` (sigils) +
`shikumi` (config) + `ishou-tokens` (palette) + the fleet runtime
crates (`mimalloc`, `parking_lot`, `crossterm`, `tracing`,
`anyhow`). Consumers don't repeat any of those in their own
`Cargo.toml`.

## When to author a new widget

| Want | Authoring location |
|---|---|
| New state-machine widget reused by GPU + TUI apps | `egaku` |
| New TUI-specific renderer for an egaku widget | `egaku-term::draw` |
| New CLI sigil / cached text panel | `tatara-ui` |
| New app-level pattern (status bar idiom, modal stack, etc.) | `moldura` |
| Chart / sparkline / image — won't fit egaku | `moldura::ratatui` interop |

The compounding rule: if you're about to write a new widget
abstraction in your TUI app, check `egaku` first. If egaku doesn't
have it, **extend egaku** rather than adding a private widget to
your app.

## Why not ratatui

The Rust ecosystem's most popular TUI library is
[ratatui](https://ratatui.rs/) — and it's excellent. But pleme-io's
widget gravity is **egaku** because GPU apps (mado, hibikine,
kagibako) and TUI apps share its state machines. Adopting ratatui
fleet-wide would create a second widget model — exactly the
duplication the compounding directive prohibits.

We keep egaku canonical and bridge to ratatui *only* where its
ecosystem offers something egaku doesn't (charts, sparklines,
`ratatui-image` Kitty graphics). The bridge lives in
`moldura::ratatui` (gated behind the `ratatui` feature flag).

## Migration plan for existing TUI consumers

Apps that currently wire `egaku-term` + `crossterm` directly should
migrate to `moldura` in order of impact:

1. **kura** (kura-tui) — most active, largest. Big win on
   lifecycle + theme consolidation.
2. **hikyaku** — email client, custom shikumi config — needs the
   composability story moldura provides.
3. **arnes** — uses egaku-term + crossterm 0.28; bumps to 0.29
   along with the move.
4. **tanken** — file manager planned; ship on moldura from M0.
5. **alicerce** — uses egaku-term wiring; consolidates here.
6. **skim-tab** — stays on raw `skim` (different fuzzy-finder
   model; out of moldura's scope).

Each migration is roughly a Cargo.toml swap + a `MolduraApp` impl
on the existing top-level state struct. The widget render paths
stay on `egaku_term::draw::*`.

## Status

M0 — crate scaffold + types + sync runtime + theme + ratatui
interop placeholder. Compiles standalone. The egaku-term cell-write
bridge for `Frame::print_at` is the first M1 deliverable (sourcing
the public-API delta from egaku-term v0.3 once published).

## See also

- `pleme-io/egaku-term` — the renderer this wraps.
- `pleme-io/egaku` — the widget state machines this exposes.
- `pleme-io/tatara/tatara-ui` — sigils + CLI UX.
- `pleme-io/shikumi` — live config.
- `pleme-io/ishou` — palette + typography source of truth.
- `pleme-io/mado` — the GPU terminal emulator that shares egaku +
  ishou + shikumi conventions with this crate.
- `pleme-io/tear` — the multiplexer that shares the same fleet
  config + theme conventions; future fleet ops UIs will compose
  moldura against tear's `MultiplexerControl` for pane-aware
  status bars.
