# Unofficial Telegram clients

Two unofficial Telegram clients on one Rust core:

- **Unofficial Mini Telegram**, the Limited client. It runs in the browser and opens exactly one chat, fixed when it is spawned.
- **Unofficial Rusty Telegram**, the Full client, on the desktop (PC only). It is a beta item.

These apps use the Telegram API and are part of the Telegram ecosystem. They are not official Telegram apps, and they do not use the Telegram logo.

## Layout

```text
crates/
  domain/       the pure-data model (elements) and the ports it is served through
  application/  the command engine: use cases, the Session, the command registry
  adapter-mem/  an in-memory Telegram behind every port (tests, and the CLI for now)
apps/
  mini/         Unofficial Mini Telegram (Limited, wasm32)
  rusty/        Unofficial Rusty Telegram (Full, desktop)
  cli/          a command-line UI over the same commands
```

Dependencies point inward only: `domain` ← `application` ← adapters and apps. An app is the composition root: it picks the adapters and hands them to a `Session`. `scripts/boundaries.sh` enforces this, and it also checks that every crate forbids `unsafe`.

## Commands

```bash
just gate            # everything CI runs
just cli commands    # list the commands
printf 'chat.open {"chat":100}\nmessage.recent\nmessage.send {"text":"Hi"}\n' | just cli
```

The `api_id` and `api_hash` are injected at build time and never committed.

## Licence

GPL-3.0-or-later. See [LICENSE](LICENSE).
