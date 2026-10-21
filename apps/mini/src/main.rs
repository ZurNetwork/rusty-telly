//! Unofficial Mini Telegram: the Limited UI, in the browser (DD 2026101202 D1, D9).
//!
//! It opens exactly one chat, fixed when it is spawned, over an
//! [`application::Session::limited`] session, and connects straight to Telegram
//! over WebSocket from its own origin. Its UI toolkit is not chosen yet: it
//! renders from page elements, not a canvas, and our crates hold no
//! macro-generated unsafe (D7). Until then this binary proves that the core
//! builds for `wasm32-unknown-unknown`.

fn main() {}
