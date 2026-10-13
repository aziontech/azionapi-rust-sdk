#!/usr/bin/env bash
# Clippy baseline for the generated crates (OpenAPI Generator output).
#
# Only the `clippy::correctness` group (code that is outright wrong) is
# denied; the style, complexity and perf groups are allowed because the
# generated code is not hand-edited. Tighten the baseline in the generator
# templates, not here. `storage` is skipped: its generated Cargo.toml has an
# invalid version ("1.0.0 (1)") and Cargo cannot load it.
set -euo pipefail

cd "$(dirname "$0")/.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}"
SKIP=" storage "

status=0
for manifest in */Cargo.toml; do
  crate="${manifest%%/*}"
  if [[ "$SKIP" == *" $crate "* ]]; then
    echo "::warning::skipping $crate (invalid generated Cargo.toml)"
    continue
  fi
  echo "::group::clippy $crate"
  if ! cargo clippy --quiet --manifest-path "$manifest" -- -A clippy::all -D clippy::correctness; then
    echo "::error::clippy found correctness issues in $crate"
    status=1
  fi
  echo "::endgroup::"
done
exit "$status"
