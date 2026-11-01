#!/bin/bash
# Runs upstream's cargo xtask ci steps one by one; tests with --no-fail-fast.
cd "$1"; export CARGO_TARGET_DIR="$2"; out="$3"; : > "$out"
step(){ name=$1; shift; "$@" > "$out.$name.log" 2>&1; echo "$name EXIT $?" >> "$out"; }
step fmt cargo fmt --all -- --check
step pdf-deps cargo xtask pdf-deps
step clippy cargo clippy --workspace --all-targets -- -D warnings
step test cargo test --workspace --lib --bins --tests --no-fail-fast
grep -E "^test .* FAILED" "$out.test.log" | sort >> "$out"
step i18n cargo xtask i18n-report
step doctest cargo test --workspace --doc
step assets cargo xtask assets
step layers cargo xtask layers
step wasm cargo xtask wasm
echo DONE >> "$out"
