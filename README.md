# Topcoat UI on Cloudflare Workers

This vibe-coded POC uses Topcoat UI to show off Topcoat's full-stack features, with the app and its
server-side functionality running entirely on Cloudflare Workers as Rust/Wasm. It is a small,
interactive showcase of what works cleanly at the edge:

- Topcoat UI and build-time Tailwind styling
- server rendering, layouts, components, and typed request data
- browser-side signals and bind attributes
- server-rendered shards
- HTMX fragment responses and response headers
- typed, request-scoped cookie state

## Development

Start the local Worker. Wrangler runs `build.sh`, which bundles Topcoat's browser assets and then
builds the Rust Worker:

```sh
npx wrangler dev
```

Then open <http://localhost:8787>.

The routes are `/`, `/reactivity`, `/htmx`, `/cookies`, and `/api/health`.

## Assets

Topcoat generates its content-hashed browser runtime and manifest in `static/_topcoat/assets`.
Wrangler serves that directory as static assets, while the manifest is embedded into the Worker so
`topcoat::runtime::script()` resolves the generated runtime URL without filesystem access.

`build.rs` also compiles `styles.css`; the asset pipeline self-hosts the Geist font and pinned HTMX
script. Tailwind's native build tooling stays a build dependency and is not linked into the Worker
Wasm module.
