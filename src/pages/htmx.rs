use serde::Deserialize;
use topcoat::{
    context::Cx,
    htmx::{HxEvent, HxResponseTrigger},
    router::{page, route, Form, RouterBuilder},
    view::{attributes, component, view, View},
    Result,
};

use crate::{
    components::{
        badge::{badge, BadgeVariant},
        button::button,
        card::{card, card_content, card_description, card_header, card_title},
        input::input,
        label::label,
        select::select,
        textarea::textarea,
    },
    pages::{demo_note, page_header},
};

#[derive(Deserialize)]
pub struct PreviewForm {
    title: String,
    summary: String,
    tone: String,
}

#[page("/htmx")]
pub async fn htmx() -> Result {
    view! {
        page_header(
            kicker: "HTML over the wire",
            title: "Explicit requests, focused fragments.",
            description: "HTMX posts this form to a Topcoat route. The Worker validates it, renders a replacement card, and sends an HX-Trigger response header.",
            execution: "Browser → Worker",
        )

        <section class="grid gap-6 xl:grid-cols-[0.9fr_1.1fr]">
            card(
                attrs: attributes! { data-demo-card="" class="h-fit" },
                card_header(
                    <div class="mb-2 flex items-center justify-between gap-3">
                        badge(variant: BadgeVariant::Secondary, "Request")
                        <code>"hx-post=/htmx/preview"</code>
                    </div>
                    card_title("Compose a release card")
                    card_description("Submit to ask the Worker for a new HTML fragment.")
                )
                card_content(
                    <form
                        hx-post="/htmx/preview"
                        hx-target="#preview"
                        hx-swap="outerHTML"
                        hx-indicator="#preview-indicator"
                        hx-disabled-elt="find button"
                        class="space-y-4"
                    >
                        <div class="space-y-2">
                            label(attrs: attributes! { for="preview-title" }, "Title")
                            input(
                                attrs: attributes! {
                                    id="preview-title"
                                    name="title"
                                    value="Topcoat reaches the edge"
                                    minlength="3"
                                    required=""
                                }
                            )
                        </div>
                        <div class="space-y-2">
                            label(attrs: attributes! { for="preview-summary" }, "Summary")
                            textarea(
                                attrs: attributes! {
                                    id="preview-summary"
                                    name="summary"
                                    rows="4"
                                    required=""
                                },
                                "A Rust component rendered this fragment inside a Cloudflare Worker."
                            )
                        </div>
                        <div class="space-y-2">
                            label(attrs: attributes! { for="preview-tone" }, "Release tone")
                            select(
                                attrs: attributes! {
                                    id="preview-tone"
                                    name="tone"
                                },
                                <option value="stable">"Stable"</option>
                                <option value="experimental">"Experimental"</option>
                                <option value="breaking">"Breaking"</option>
                            )
                        </div>
                        <div class="flex items-center gap-3 pt-1">
                            button(attrs: attributes! { type="submit" }, "Render fragment")
                            <span
                                id="preview-indicator"
                                class="htmx-indicator inline-flex items-center gap-2 text-xs text-muted-foreground"
                            >
                                <span
                                    class="size-3 animate-spin rounded-full border-2 border-current border-r-transparent"
                                    aria-hidden="true"
                                ></span>
                                "Worker rendering…"
                            </span>
                        </div>
                    </form>
                )
            )

            <div class="space-y-4">
                preview_card(
                    title: "Topcoat reaches the edge",
                    summary: "A Rust component rendered this fragment inside a Cloudflare Worker.",
                    tone: "stable",
                    fresh: false,
                )
                demo_note(
                    title: "What crosses the boundary",
                    <p>
                        "The browser sends form-encoded values. Topcoat's "
                        <code>"Form<PreviewForm>"</code>
                        " extractor creates typed Rust data; the response contains HTML plus "
                        <code>"HX-Trigger-After-Swap"</code>
                        ". No JSON endpoint is involved."
                    </p>
                )
            </div>
        </section>
    }
}

#[route(POST "/htmx/preview")]
async fn render_preview(
    cx: &Cx,
    Form(form): Form<PreviewForm>,
) -> Result<(HxResponseTrigger, View)> {
    let title = form.title.trim();
    let summary = form.summary.trim();
    let tone = match form.tone.as_str() {
        "experimental" => "experimental",
        "breaking" => "breaking",
        _ => "stable",
    };

    let fragment = if title.chars().count() < 3 || summary.chars().count() < 8 {
        view! {
            <div
                id="preview"
                class="rounded-xl border border-destructive/40 bg-destructive/5 p-6 text-sm"
                role="alert"
            >
                <p class="font-semibold text-destructive">"The Worker rejected this preview."</p>
                <p class="mt-2 text-muted-foreground">
                    "Use a title of at least 3 characters and a summary of at least 8."
                </p>
            </div>
        }?
    } else {
        preview_fragment(cx, title, summary, tone, true).await?
    };

    let event = HxEvent::with_data("preview-rendered", "Fresh HTML from the Worker")?;
    Ok((HxResponseTrigger::after_swap([event]), fragment))
}

#[component]
async fn preview_card(
    cx: &Cx,
    title: &str,
    summary: &str,
    tone: &str,
    #[default(false)] fresh: bool,
) -> Result {
    preview_fragment(cx, title, summary, tone, fresh).await
}

async fn preview_fragment(
    cx: &Cx,
    title: &str,
    summary: &str,
    tone: &str,
    fresh: bool,
) -> Result<View> {
    let (release_label, variant) = match tone {
        "experimental" => ("Experimental", BadgeVariant::Secondary),
        "breaking" => ("Breaking", BadgeVariant::Destructive),
        _ => ("Stable", BadgeVariant::Primary),
    };

    view! { cx =>
        <article id="preview" class="overflow-hidden rounded-xl border border-border bg-background shadow-sm">
            <div class="border-b border-border bg-foreground/[0.025] px-6 py-4">
                <div class="flex flex-wrap items-center justify-between gap-3">
                    <span class="font-mono text-xs text-muted-foreground">"HTML fragment preview"</span>
                    <div class="flex items-center gap-2">
                        if fresh {
                            badge(variant: BadgeVariant::Outline, "Just rendered")
                        }
                        badge(variant: variant, (release_label))
                    </div>
                </div>
            </div>
            <div class="p-6 sm:p-8">
                <p class="text-xs font-semibold tracking-[0.16em] text-muted-foreground uppercase">
                    "Release note"
                </p>
                <h2 class="mt-3 text-2xl font-semibold tracking-tight">(title)</h2>
                <p class="mt-3 max-w-xl text-sm leading-6 text-muted-foreground">(summary)</p>
                <div class="mt-6 flex items-center gap-3 text-xs text-muted-foreground">
                    <span class="size-2 rounded-full bg-emerald-500"></span>
                    "Rendered on demand by Topcoat"
                </div>
            </div>
        </article>
    }
}

pub fn routes(builder: RouterBuilder) -> RouterBuilder {
    builder.route(render_preview)
}
