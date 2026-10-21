default:
    @just --list

# Everything CI runs: format, lint (native and wasm), tests, the dependency
# policy and the architecture's guards.
gate: fmt lint wasm test deny boundaries

fmt:
    cargo fmt --all --check

lint:
    cargo clippy --workspace --all-targets --locked -- -D warnings

# The core and Mini Telegram build for the browser.
wasm:
    cargo clippy -p domain -p application -p mini --target wasm32-unknown-unknown --locked -- -D warnings

test:
    cargo test --workspace --locked

deny:
    cargo deny check advisories licenses bans sources

# No unsafe in our crates; domain <- application <- adapters, never the reverse.
boundaries:
    scripts/boundaries.sh

# Drive the command engine over the in-memory demo account, e.g.
# `printf 'chat.open {"chat":100}\nmessage.recent\n' | just cli`.
cli *args:
    cargo run -q -p cli -- {{args}}
