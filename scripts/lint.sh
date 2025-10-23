#!/usr/bin/env bash

cd src-tauri && cargo clippy && cargo fmt -- --check
cd .. || exit 1
deno check
deno fmt --unstable-components --check
deno lint
deno task check

