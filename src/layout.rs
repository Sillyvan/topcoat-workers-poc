use topcoat::{
    asset::{asset, Asset},
    context::Cx,
    font::{fontsource::fontsource_font, Font},
    router::{layout, uri},
    view::{attributes, class, view},
    Result,
};

use crate::{
    components::badge::{badge_variants, BadgeVariant},
    components::switch::switch,
    state,
};

pub(crate) const GEIST: Font = fontsource_font!(
    GEIST,
    weight: [400, 500, 600, 700],
    style: Normal,
    host: Asset,
);
const HTMX: Asset = asset!("https://cdn.jsdelivr.net/npm/htmx.org@2.0.10/dist/htmx.min.js");
const STYLESHEET: Asset = asset!("styles.generated.css");

const NAV_ITEMS: &[(&str, &str, &str)] = &[
    ("/", "Overview", "01"),
    ("/reactivity", "Reactivity", "02"),
    ("/htmx", "HTMX", "03"),
    ("/cookies", "Cookies", "04"),
];

#[layout("/")]
pub async fn shell(cx: &Cx, slot: Result) -> Result {
    let path = uri(cx).path();
    let preferences = state::read(cx);

    view! {
        <!DOCTYPE html>
        <html
            lang="en"
            class=(class!("dark" if preferences.dark))
            data-density=(&preferences.density)
        >
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta
                    name="description"
                    content="An interactive Topcoat framework showcase running as Rust/Wasm on Cloudflare Workers."
                >
                <title>"Topcoat on Cloudflare"</title>
                topcoat::font::link(font: GEIST)
                <link rel="stylesheet" href=(STYLESHEET)>
                topcoat::runtime::script()
                <script src=(HTMX) defer=""></script>
            </head>
            <body>
                <div class="min-h-screen">
                    <header
                        class="sticky top-0 z-20 border-b border-border/80 bg-background/90 backdrop-blur"
                    >
                        <div
                            class="mx-auto flex h-16 max-w-7xl items-center justify-between gap-4 px-4 sm:px-6"
                        >
                            <a href="/" class="group flex items-center gap-3">
                                <span
                                    class="grid size-9 place-items-center rounded-lg bg-primary text-sm font-bold text-primary-foreground shadow-xs transition-transform group-hover:-rotate-3"
                                >
                                    "T"
                                </span>
                                <span class="leading-tight">
                                    <span class="block text-sm font-semibold tracking-tight">
                                        "Topcoat"
                                    </span>
                                    <span class="block text-xs text-muted-foreground">
                                        "on Cloudflare"
                                    </span>
                                </span>
                            </a>
                            <div class="flex items-center gap-3">
                                <form method="POST" action="/preferences/dark">
                                    <input type="hidden" name="redirect" value=(path)>
                                    <label
                                        for="header-dark-mode"
                                        class="flex cursor-pointer items-center gap-2 text-xs font-medium text-muted-foreground"
                                    >
                                        <span class="hidden md:inline">"Dark mode"</span>
                                        switch(
                                            attrs: attributes! {
                                                id="header-dark-mode"
                                                name="dark"
                                                value="on"
                                                aria-label="Dark mode"
                                                title="Toggle dark mode"
                                                onchange="this.form.submit()"
                                                if preferences.dark { checked="" }
                                            }
                                        )
                                    </label>
                                </form>
                                <div
                                    class="flex items-center gap-2 text-xs text-muted-foreground"
                                >
                                    <span class="relative flex size-2">
                                        <span
                                            class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-500 opacity-60"
                                        ></span>
                                        <span
                                            class="relative inline-flex size-2 rounded-full bg-emerald-500"
                                        ></span>
                                    </span>
                                    <span class="hidden sm:inline">
                                        "Rust/Wasm at the edge"
                                    </span>
                                    <a
                                        href="/api/health"
                                        class=(badge_variants(BadgeVariant::Outline))
                                    >
                                        "GET /api/health"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </header>

                    <div
                        class="mx-auto grid max-w-7xl gap-8 px-4 py-6 sm:px-6 lg:grid-cols-[13rem_minmax(0,1fr)] lg:py-10"
                    >
                        <aside class="min-w-0 lg:sticky lg:top-24 lg:h-fit">
                            <nav
                                aria-label="Showcase"
                                class="flex gap-1 overflow-x-auto pb-2 lg:flex-col lg:overflow-visible"
                            >
                                for (href, label, number) in NAV_ITEMS {
                                    <a
                                        href=(href)
                                        aria-current=(if *href == path {
                                            Some("page")
                                        } else {
                                            None
                                        })
                                        class=(class!(
                                            "group flex shrink-0 items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors",
                                            "bg-primary text-primary-foreground shadow-xs" if *href == path else "text-muted-foreground hover:bg-foreground/5 hover:text-foreground",
                                        ))
                                    >
                                        <span
                                            class=(class!(
                                                "font-mono text-[10px]",
                                                "text-primary-foreground/60" if *href == path else "text-muted-foreground/70",
                                            ))
                                        >
                                            (number)
                                        </span>
                                        <span class="font-medium">(label)</span>
                                    </a>
                                }
                            </nav>
                            <div
                                class="mt-6 hidden rounded-lg border border-dashed border-border p-3 text-xs leading-relaxed text-muted-foreground lg:block"
                            >
                                "Demo data lives in Rust constants, requests, or your browser cookie."
                            </div>
                        </aside>

                        <main class="min-w-0 pb-16">(slot?)</main>
                    </div>

                    <footer class="border-t border-border">
                        <div
                            class="mx-auto flex max-w-7xl flex-col gap-2 px-4 py-6 text-xs text-muted-foreground sm:flex-row sm:items-center sm:justify-between sm:px-6"
                        >
                            <p>"Rendered with Topcoat inside a Cloudflare Worker."</p>
                            <p>"Static data · no mutable global state"</p>
                        </div>
                    </footer>
                </div>
            </body>
        </html>
    }
}
