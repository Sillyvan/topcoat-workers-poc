use topcoat::{
    router::page,
    view::{attributes, view},
    Result,
};

use crate::{
    components::{
        badge::{badge, BadgeVariant},
        button::{button_variants, ButtonSize, ButtonVariant},
        card::{card, card_content, card_description, card_footer, card_header, card_title},
    },
    pages::page_header,
};

const FEATURES: &[(&str, &str, &str, &str, &str)] = &[
    (
        "Client reactivity",
        "Browser",
        "Type-checked Rust expressions become tiny browser-side updates with no Wasm client bundle.",
        "/reactivity",
        "$(...)",
    ),
    (
        "Server shards",
        "Browser → Worker",
        "Reactive arguments trigger a server component render and swap only the returned fragment.",
        "/reactivity",
        "#[shard]",
    ),
    (
        "HTMX fragments",
        "Browser → Worker",
        "Explicit form requests return focused HTML fragments plus Topcoat's typed HTMX headers.",
        "/htmx",
        "hx-post",
    ),
    (
        "Cookie state",
        "Worker",
        "A typed cookie store persists harmless preferences and counters across requests.",
        "/cookies",
        "CookieStore<T>",
    ),
];

#[page("/")]
pub async fn home() -> Result {
    view! {
        page_header(
            kicker: "Cloudflare compatibility lab",
            title: "A small full-stack framework, running at the edge.",
            description: "This project exercises the parts of Topcoat that compile cleanly to Rust/Wasm and behave honestly inside Cloudflare Workers.",
            execution: "Worker",
        )

        <section class="mb-8 grid gap-4 lg:grid-cols-[1.35fr_0.65fr]">
            <div
                class="relative overflow-hidden rounded-xl border border-border bg-primary p-7 text-primary-foreground shadow-sm sm:p-9"
            >
                <div
                    class="absolute -top-20 -right-20 size-64 rounded-full border border-primary-foreground/10"
                ></div>
                <div
                    class="absolute -right-5 -bottom-28 size-56 rounded-full border border-primary-foreground/10"
                ></div>
                <div class="relative max-w-xl">
                    <p class="font-mono text-xs tracking-[0.16em] text-primary-foreground/60 uppercase">
                        "One request, one HTML response"
                    </p>
                    <h2 class="mt-4 text-2xl font-semibold tracking-tight sm:text-3xl">
                        "Server rendering when you want it. Browser reactivity when you need it."
                    </h2>
                    <p class="mt-4 text-sm leading-6 text-primary-foreground/70">
                        "Pages and components stay in Rust. Topcoat selectively turns reactive expressions into JavaScript and sends shard or HTMX work back to the Worker."
                    </p>
                    <div class="mt-6 flex flex-wrap gap-3">
                        <a
                            href="/reactivity"
                            class=(button_variants(ButtonVariant::Inverse, ButtonSize::Lg))
                        >
                            "Try the interactions"
                        </a>
                    </div>
                </div>
            </div>

            card(
                attrs: attributes! { data-demo-card="" class="justify-between" },
                card_header(
                    card_description("Current runtime")
                    card_title(
                        attrs: attributes! { class="text-xl" },
                        "Cloudflare Worker"
                    )
                )
                card_content(
                    <dl class="space-y-4 text-sm">
                        <div class="flex items-center justify-between gap-4">
                            <dt class="text-muted-foreground">"Server"</dt>
                            <dd class="font-mono">"Rust/Wasm"</dd>
                        </div>
                        <div class="flex items-center justify-between gap-4">
                            <dt class="text-muted-foreground">"Persistence"</dt>
                            <dd>"Cookie only"</dd>
                        </div>
                        <div class="flex items-center justify-between gap-4">
                            <dt class="text-muted-foreground">"Health"</dt>
                            <dd class="flex items-center gap-2">
                                <span class="size-1.5 rounded-full bg-emerald-500"></span>
                                "Ready"
                            </dd>
                        </div>
                    </dl>
                )
                card_footer(
                    attrs: attributes! { class="border-t border-border pt-5" },
                    <a href="/api/health" class="font-mono text-xs text-muted-foreground hover:text-foreground">
                        "/api/health → ok"
                    </a>
                )
            )
        </section>

        <section>
            <div class="mb-4 flex items-end justify-between gap-4">
                <div>
                    <h2 class="text-lg font-semibold">"Explore the feature path"</h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Each page isolates one boundary so the network behavior stays visible."
                    </p>
                </div>
                badge(variant: BadgeVariant::Outline, (FEATURES.len()) " demos")
            </div>
            <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
                for (name, execution, description, href, code) in FEATURES {
                    card(
                        attrs: attributes! { data-demo-card="" class="group h-full transition-transform hover:-translate-y-0.5" },
                        card_header(
                            <div class="mb-2 flex items-center justify-between gap-3">
                                <code>(code)</code>
                                badge(
                                    variant: if *execution == "Worker" { BadgeVariant::Outline } else { BadgeVariant::Secondary },
                                    (execution)
                                )
                            </div>
                            card_title((name))
                            card_description((description))
                        )
                        card_footer(
                            attrs: attributes! { class="mt-auto" },
                            <a
                                href=(href)
                                class="text-sm font-medium underline-offset-4 group-hover:underline"
                            >
                                "Open demo →"
                            </a>
                        )
                    )
                }
            </div>
        </section>
    }
}
