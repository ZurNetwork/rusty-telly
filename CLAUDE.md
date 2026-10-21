# CLAUDE.md

Unofficial Mini Telegram (Limited, browser) and Unofficial Rusty Telegram (Full, desktop) on one Rust core. The repository is GPL-3.0-or-later and separate from Zurfur.

## Design

This repository is governed by design page `2026101202`, "Unofficial Telegram Clients — Mini in the Browser, Rusty on the Desktop, One Rust Core". It lives in the Zurfur design corpus at `~/code/zurfur-design/decisions/2026101202-*.md`. Read that page before guessing at scope, increments (A1–A4) or open items. Rust style follows page `37519361`, "Code Style — Semantic Rulings (Rust)". This repository keeps no local design documents.

tdesktop, the reference for behaviour and features, is checked out at `../tdesktop`.

## Architecture

- `domain`: `elements` holds plain values, and `ports` holds `?Send` async traits named by role. `elements` never imports `ports`, and `domain/tests/elements_do_not_import_ports.rs` guards that.
- `application`: use cases are plain `async fn`s with their ports in their signature. `Session` holds the ports, the `Edition` and the open chat. `commands` is the PhotoCraft-style registry (`noun.verb` ids, JSON params), filtered by edition. `dispatch` is the door every UI uses.
- Adapters implement the ports. Apps are composition roots.
- `scripts/boundaries.sh` enforces the dependency rule and the workspace lints.

## Invariants (DD 2026101202)

- No `unsafe` in our crates (`unsafe_code = "forbid"`, inherited by every member). Dependencies may use it.
- The Telegram login never touches Zurfur: there is no relay, and the key stays on Mini Telegram's own origin.
- Limited has exactly one chat, fixed at spawn. It has no command that opens another, and spawning does no I/O.
- Broadcast channels are never opened until sponsored messages are shown (`ChatKind::may_open`).
- A `ChatRef` never carries an access hash, and it resolves only among the user's own dialogs.
- The `api_id` and `api_hash` are injected at build time and never committed.

## Commands

`just gate` runs everything CI runs (fmt, clippy for native and wasm32, test, cargo-deny, boundaries). `just cli` drives the engine over the in-memory demo account. Dependencies are exact-pinned in `[workspace.dependencies]`, and `Cargo.lock` is committed.
