#!/usr/bin/env bash

set -euo pipefail

readonly TOPCOAT_REV="6bd15a595e0d1cdb4af0b5fbdcd9be5e9873f6c9"

# Wrangler's build environment needs both Rust build tools installed explicitly.
cargo install -q "worker-build@^0.8"
cargo install -q \
    --git https://github.com/tokio-rs/topcoat \
    --rev "${TOPCOAT_REV}" \
    topcoat-cli \
    --locked

# Generate the manifest first so the final Worker build embeds the current asset catalog.
topcoat asset bundle --out static/_topcoat/assets
worker-build --release
