//! Optional ratatui interop — gated behind the `ratatui` feature.
//!
//! pleme-io's indigenous TUI stack (egaku + egaku-term) is the
//! canonical fleet choice: GPU and TUI apps share widget semantics
//! via egaku, which ratatui can't offer. **BUT** ratatui's widget
//! ecosystem (sparklines, charts, gauges, ratatui-image's Kitty
//! graphics renderer) is genuinely best-in-class for certain
//! visualisation needs.
//!
//! This module re-exports `ratatui` + `ratatui_image` so apps that
//! want one of those specific widgets pull them through moldura
//! (one pinned version) rather than adding an independent dep.
//!
//! ## When to reach for this
//!
//! - You want a sparkline / chart / gauge. ratatui has them; egaku
//!   doesn't (yet).
//! - You want Kitty-graphics image rendering via `ratatui-image`.
//! - You want to ship a widget today that someone wrote against
//!   ratatui and you don't want to port it.
//!
//! ## When to NOT reach for this
//!
//! - You're picking between egaku and ratatui for a NEW widget.
//!   Author it in egaku — that's where the fleet's widget gravity
//!   sits.
//!
//! ## Cell-buffer bridge
//!
//! ratatui's `Buffer` and egaku-term's renderer ultimately stage
//! the same `(char, fg, bg, attrs)` cells into crossterm's queue.
//! A future moldura M1 deliverable adds a thin
//! `render_ratatui_into_term(term, area, widget)` adapter that
//! drives ratatui's `Widget::render` against a temporary
//! `ratatui::buffer::Buffer` and then copies cells into the
//! `egaku_term::Terminal` for the matching rectangle. The
//! interface lands once egaku-term exposes the necessary cell-
//! write API publicly.

pub use ratatui;
pub use ratatui_image;
