use topcoat::{
    router::{page, route, Body, Router},
    runtime::{shard, Event, RouterBuilderShardExt},
    view::view,
    Result as TopcoatResult,
};
use worker::{Context, Env, HttpRequest};

fn router() -> Router {
    Router::builder()
        .page(home)
        .route(health)
        .shard(search_results)
        .build()
}

#[worker::event(fetch)]
async fn fetch(
    request: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> worker::Result<http::Response<Body>> {
    Ok(router().handle(request.map(Body::new)).await)
}

#[page("/")]
async fn home() -> TopcoatResult {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Topcoat on Cloudflare"</title>
                <script type="module" src="/topcoat-runtime.js"></script>
            </head>
            <body>
                <main>
                    <h1>"Topcoat is running on Cloudflare Workers"</h1>
                    <p>"This HTML was rendered by Topcoat at the edge."</p>

                    signal query = String::new();

                    <label for="search">"Search frameworks"</label>
                    <input
                        id="search"
                        type="search"
                        placeholder="Try rust, cloud, or react"
                        autocomplete="off"
                        :value=$(query.get())
                        @input=$(|event: Event| query.set(event.target.value))
                    >

                    search_results(query: $(query.get()))
                </main>
            </body>
        </html>
    }
}

#[shard]
async fn search_results(query: String) -> TopcoatResult {
    const FRAMEWORKS: &[&str] = &[
        "Axum",
        "Cloudflare Workers",
        "Django",
        "Leptos",
        "Next.js",
        "Phoenix",
        "React",
        "Svelte",
        "Topcoat",
    ];

    let query = query.trim().to_ascii_lowercase();
    let matches = FRAMEWORKS
        .iter()
        .copied()
        .filter(|framework| framework.to_ascii_lowercase().contains(&query))
        .collect::<Vec<_>>();

    view! {
        <section aria-live="polite">
            if matches.is_empty() {
                <p>"No frameworks matched."</p>
            } else {
                <p>"Matches: " (matches.len())</p>
                <ul>
                    for framework in matches {
                        <li>(framework)</li>
                    }
                </ul>
            }
        </section>
    }
}

#[route(GET "/api/health")]
async fn health() -> TopcoatResult<&'static str> {
    Ok("ok")
}
