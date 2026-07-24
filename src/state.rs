use serde::{Deserialize, Serialize};
use topcoat::{
    context::Cx,
    cookie::{cookie_store, cookies, CookieStore, Cookies, SameSite},
    router::uri,
    Result,
};

const COOKIE_NAME: &str = "topcoat_showcase";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Preferences {
    pub visits: u32,
    pub dark: bool,
    pub density: String,
    pub banner_dismissed: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            visits: 0,
            dark: false,
            density: "comfortable".to_owned(),
            banner_dismissed: false,
        }
    }
}

fn app_cookies(cx: &Cx) -> impl Cookies + '_ {
    cookies(cx)
        .default_path("/")
        .default_http_only(true)
        .default_same_site(SameSite::Lax)
        .default_secure(uri(cx).scheme_str() == Some("https"))
}

fn store(cx: &Cx) -> CookieStore<Preferences, impl Cookies + '_> {
    cookie_store(app_cookies(cx), COOKIE_NAME).parse_or_default()
}

pub fn read(cx: &Cx) -> Preferences {
    store(cx).get()
}

pub fn record_visit(cx: &Cx) -> Result<Preferences> {
    Ok(store(cx)
        .update(|preferences| preferences.visits = preferences.visits.saturating_add(1))
        .commit()?)
}

pub fn save_density(cx: &Cx, density: &str) -> Result<Preferences> {
    let density = match density {
        "compact" => "compact",
        _ => "comfortable",
    };

    Ok(store(cx)
        .update(|preferences| preferences.density = density.to_owned())
        .commit()?)
}

pub fn set_dark(cx: &Cx, dark: bool) -> Result<Preferences> {
    Ok(store(cx)
        .update(|preferences| preferences.dark = dark)
        .commit()?)
}

pub fn dismiss_banner(cx: &Cx) -> Result<Preferences> {
    Ok(store(cx)
        .update(|preferences| preferences.banner_dismissed = true)
        .commit()?)
}

pub fn reset(cx: &Cx) {
    store(cx).remove();
}
