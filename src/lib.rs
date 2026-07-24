mod components;
mod layout;
mod pages;
mod state;

use topcoat::{
    asset::{AssetConfig, Manifest, RouterBuilderAssetExt},
    cookie::RouterBuilderCookieExt,
    font::RouterBuilderFontExt,
    router::{route, Body, Router},
    runtime::RouterBuilderShardExt,
    Result as TopcoatResult,
};
use worker::{Context, Env, HttpRequest};

fn router() -> Router {
    let manifest = Manifest::parse(include_str!("../static/_topcoat/assets/manifest.toml"))
        .expect("invalid Topcoat asset manifest");

    let builder = Router::builder()
        .layout(layout::shell)
        .page(pages::home::home)
        .page(pages::reactivity::reactivity)
        .page(pages::htmx::htmx)
        .page(pages::cookies::cookies_page);
    let builder = pages::htmx::routes(builder);
    let builder = pages::cookies::routes(builder);
    let builder = pages::preferences::routes(builder);

    builder
        .route(health)
        .shard(pages::reactivity::search_results)
        .font(layout::GEIST)
        .cookies()
        .assets(AssetConfig::hosted_at("/_topcoat/assets", manifest))
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

#[route(GET "/api/health")]
async fn health() -> TopcoatResult<&'static str> {
    Ok("ok")
}
