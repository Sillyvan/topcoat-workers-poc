use serde::Deserialize;
use topcoat::{
    context::Cx,
    router::{
        error::{see_other, SeeOther},
        route, Form, RouterBuilder,
    },
    Result,
};

use crate::state;

#[derive(Deserialize)]
struct DarkModeForm {
    dark: Option<String>,
    redirect: String,
}

#[route(POST "/preferences/dark")]
async fn save_dark_mode(cx: &Cx, Form(form): Form<DarkModeForm>) -> Result<SeeOther> {
    state::set_dark(cx, form.dark.is_some())?;

    let redirect = match form.redirect.as_str() {
        "/" | "/reactivity" | "/htmx" | "/cookies" => form.redirect.as_str(),
        _ => "/",
    };

    Ok(see_other(redirect))
}

pub fn routes(builder: RouterBuilder) -> RouterBuilder {
    builder.route(save_dark_mode)
}
