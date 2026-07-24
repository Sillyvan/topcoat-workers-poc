# Topcoat on Cloudflare Workers

This POC runs Topcoat's router and server rendering as Rust/Wasm in a Cloudflare Worker. It also
uses a browser-side signal and a `#[shard]` to filter a hardcoded list on the server.

## Development

Start the local Worker:

```sh
npx wrangler dev
```

Then open <http://localhost:8787>.

## Runtime JavaScript

`public/topcoat-runtime.js` is currently copied manually from the same Topcoat commit pinned in
`Cargo.toml`. Normally Topcoat's asset bundler generates this file and its manifest, but the current
CLI expects a Cargo executable while workers-rs builds a `cdylib` Wasm artifact. This is a temporary
POC workaround.
