#!/usr/bin/env bash

set -euo pipefail

readonly TOPCOAT_REV="6bd15a595e0d1cdb4af0b5fbdcd9be5e9873f6c9"

# Cloudflare Workers Builds does not include Rust in its build image. Install a
# minimal toolchain there, while continuing to use the existing toolchain
# during local development.
if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 --fail --silent --show-error \
        https://sh.rustup.rs | sh -s -- \
        -y \
        --profile minimal \
        --default-toolchain stable \
        --target wasm32-unknown-unknown

    export PATH="${CARGO_HOME:-${HOME}/.cargo}/bin:${PATH}"
fi

rustup target add wasm32-unknown-unknown

# Wrangler's build environment needs both Rust build tools installed explicitly.
cargo install -q "worker-build@^0.8"
cargo install -q \
    --git https://github.com/tokio-rs/topcoat \
    --rev "${TOPCOAT_REV}" \
    topcoat-cli \
    --locked

# Generate the manifest first so the final Worker build embeds the current asset catalog.
topcoat asset bundle --out static/_topcoat/assets
worker-build --release --panic-unwind
