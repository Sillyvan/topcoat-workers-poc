use topcoat::{
    router::{page, route, Body, Router},
    view::view,
    Result as TopcoatResult,
};
use worker::{Context, Env, HttpRequest};

fn router() -> Router {
    Router::builder().page(home).route(health).build()
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
            </head>
            <body>
                <main>
                    <h1>"Topcoat is running on Cloudflare Workers"</h1>
                    <p>"This HTML was rendered by Topcoat at the edge."</p>
                </main>
            </body>
        </html>
    }
}

#[route(GET "/api/health")]
async fn health() -> TopcoatResult<&'static str> {
    Ok("ok")
}
