use serde::Deserialize;
use topcoat::{
    context::Cx,
    router::{
        error::{see_other, SeeOther},
        page, route, Form, RouterBuilder,
    },
    view::{attributes, view},
    Result,
};

use crate::{
    components::{
        badge::{badge, BadgeVariant},
        button::{button, ButtonVariant},
        card::{card, card_content, card_description, card_footer, card_header, card_title},
        label::label,
        select::select,
    },
    pages::page_header,
    state,
};

#[derive(Deserialize)]
pub struct PreferencesForm {
    density: String,
}

#[page("/cookies")]
pub async fn cookies_page(cx: &Cx) -> Result {
    let preferences = state::record_visit(cx)?;

    view! {
        page_header(
            kicker: "Request-scoped state",
            title: "Useful persistence in a browser cookie.",
            description: "This page reads a typed cookie, increments its visit count, and queues a Set-Cookie header on the response. Density persists across the whole showcase; dark mode is available in the header.",
            execution: "Worker",
        )

        if !preferences.banner_dismissed {
            <section
                class="mb-6 flex flex-col gap-4 rounded-xl border border-primary/20 bg-primary/[0.04] p-5 sm:flex-row sm:items-center sm:justify-between"
            >
                <div>
                    <p class="text-sm font-medium">"This banner is cookie-backed."</p>
                    <p class="mt-1 text-xs leading-5 text-muted-foreground">
                        "Dismiss it, refresh the page, and it stays gone."
                    </p>
                </div>
                <form method="POST" action="/cookies/dismiss">
                    button(
                        size: crate::components::button::ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        attrs: attributes! { type="submit" },
                        "Dismiss"
                    )
                </form>
            </section>
        }

        <section class="grid gap-6 xl:grid-cols-[0.9fr_1.1fr]">
            card(
                attrs: attributes! { data-demo-card="" class="h-fit" },
                card_header(
                    <div class="mb-2 flex items-center justify-between gap-3">
                        badge(variant: BadgeVariant::Secondary, "CookieStore<T>")
                        <code>"SameSite=Lax"</code>
                    </div>
                    card_title("Browser preferences")
                    card_description("The cookie is HttpOnly and only carries non-sensitive demo settings.")
                    )
                    card_content(
                        <form method="POST" action="/cookies/preferences" class="space-y-5">
                        <div class="space-y-2">
                            label(attrs: attributes! { for="density" }, "Interface density")
                            select(
                                attrs: attributes! {
                                    id="density"
                                    name="density"
                                },
                                <option
                                    value="comfortable"
                                    if preferences.density == "comfortable" { selected="" }
                                >
                                    "Comfortable"
                                </option>
                                <option
                                    value="compact"
                                    if preferences.density == "compact" { selected="" }
                                >
                                    "Compact"
                                </option>
                            )
                        </div>
                        button(attrs: attributes! { type="submit" }, "Save preferences")
                    </form>
                )
            )

            <div class="space-y-6">
                card(
                    attrs: attributes! { data-demo-card="" },
                    card_header(
                        <div class="mb-2 flex items-center justify-between gap-3">
                            badge(variant: BadgeVariant::Outline, "Current payload")
                            <span class="text-xs text-muted-foreground">"Typed Rust struct"</span>
                        </div>
                        card_title("What the Worker decoded")
                        card_description("Malformed or missing data safely falls back to defaults.")
                    )
                    card_content(
                        <dl class="divide-y divide-border rounded-lg border border-border text-sm">
                            <div class="flex items-center justify-between gap-4 p-3">
                                <dt class="font-mono text-xs text-muted-foreground">"visits"</dt>
                                <dd class="font-semibold tabular-nums">(preferences.visits)</dd>
                            </div>
                            <div class="flex items-center justify-between gap-4 p-3">
                                <dt class="font-mono text-xs text-muted-foreground">"dark"</dt>
                                <dd>(preferences.dark)</dd>
                            </div>
                            <div class="flex items-center justify-between gap-4 p-3">
                                <dt class="font-mono text-xs text-muted-foreground">"density"</dt>
                                <dd>(&preferences.density)</dd>
                            </div>
                            <div class="flex items-center justify-between gap-4 p-3">
                                <dt class="font-mono text-xs text-muted-foreground">"banner_dismissed"</dt>
                                <dd>(preferences.banner_dismissed)</dd>
                            </div>
                        </dl>
                    )
                    card_footer(
                        attrs: attributes! { class="justify-between border-t border-border pt-5" },
                        <p class="text-xs text-muted-foreground">
                            "Visit count increments on each GET."
                        </p>
                        <form method="POST" action="/cookies/reset">
                            button(
                                size: crate::components::button::ButtonSize::Sm,
                                variant: ButtonVariant::Ghost,
                                attrs: attributes! { type="submit" },
                                "Reset cookie"
                            )
                        </form>
                    )
                )
            </div>
        </section>
    }
}

#[route(POST "/cookies/preferences")]
async fn save_preferences(cx: &Cx, Form(form): Form<PreferencesForm>) -> Result<SeeOther> {
    state::save_density(cx, &form.density)?;
    Ok(see_other("/cookies"))
}

#[route(POST "/cookies/dismiss")]
async fn dismiss_banner(cx: &Cx) -> Result<SeeOther> {
    state::dismiss_banner(cx)?;
    Ok(see_other("/cookies"))
}

#[route(POST "/cookies/reset")]
async fn reset_preferences(cx: &Cx) -> Result<SeeOther> {
    state::reset(cx);
    Ok(see_other("/cookies"))
}

pub fn routes(builder: RouterBuilder) -> RouterBuilder {
    builder
        .route(save_preferences)
        .route(dismiss_banner)
        .route(reset_preferences)
}
