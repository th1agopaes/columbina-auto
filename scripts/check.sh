#!/usr/bin/env bash
# The green button: the same checks the CI runs, before every commit.
# Usage: ./scripts/check.sh
set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> fmt"
cargo fmt --all --check
echo "==> clippy"
cargo clippy --workspace --all-targets --locked --quiet -- -D warnings
echo "==> test"
cargo test --workspace --locked --quiet
echo "==> doc"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --quiet
echo "All green."
