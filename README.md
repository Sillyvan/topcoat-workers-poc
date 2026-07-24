# Topcoat on Cloudflare Workers

This POC runs Topcoat's router and server rendering as Rust/Wasm in a Cloudflare Worker. It also
uses a browser-side signal and a `#[shard]` to filter a hardcoded list on the server.

## Development

Start the local Worker. Wrangler runs `build.sh`, which bundles Topcoat's browser assets and then
builds the Rust Worker:

```sh
npx wrangler dev
```

Then open <http://localhost:8787>.

## Assets

Topcoat generates its content-hashed browser runtime and manifest in `static/_topcoat/assets`.
Wrangler serves that directory as static assets, while the manifest is embedded into the Worker so
`topcoat::runtime::script()` resolves the generated runtime URL without filesystem access.
