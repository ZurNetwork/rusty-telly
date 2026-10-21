#!/usr/bin/env bash
# The architecture's guards, for `just gate` and CI.
#   1. Every member inherits the workspace lints, so every crate we write
#      forbids unsafe (DD 2026101202 D7).
#   2. The dependency rule: `domain` depends on no workspace crate, and
#      `application` on no adapter (normal dependencies; dev-dependencies may).
set -euo pipefail
cd "$(dirname "$0")/.."

failed=0

for manifest in crates/*/Cargo.toml apps/*/Cargo.toml; do
    if ! grep -Pzq '\[lints\]\nworkspace = true' "$manifest"; then
        echo "boundaries: $manifest does not inherit the workspace lints ([lints] workspace = true)"
        failed=1
    fi
done

workspace_crates=$(cargo metadata --no-deps --format-version 1 --locked | grep -o '"name":"[^"]*","version"' | cut -d'"' -f4)

domain_deps=$(cargo tree -p domain -e normal --prefix none --locked | tail -n +2 | cut -d' ' -f1 | sort -u)
for name in $workspace_crates; do
    if [ "$name" != domain ] && grep -qx "$name" <<<"$domain_deps"; then
        echo "boundaries: domain depends on the workspace crate $name"
        failed=1
    fi
done

application_deps=$(cargo tree -p application -e normal --prefix none --locked | cut -d' ' -f1 | sort -u)
if grep -q '^adapter-' <<<"$application_deps"; then
    echo "boundaries: application depends on an adapter: $(grep '^adapter-' <<<"$application_deps" | tr '\n' ' ')"
    failed=1
fi

exit "$failed"
