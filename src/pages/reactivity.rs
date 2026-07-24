use topcoat::{
    router::page,
    runtime::{shard, Event},
    view::{attributes, view},
    Result,
};

use crate::{
    components::{
        badge::{badge, BadgeVariant},
        button::{button, ButtonSize, ButtonVariant},
        card::{card, card_content, card_description, card_header, card_title},
        input::input,
    },
    pages::{demo_note, page_header},
};

const EXAMPLES: &[(&str, &str, &str)] = &[
    (
        "Signals",
        "Browser",
        "Local state with typed get and set expressions",
    ),
    (
        "Bind attributes",
        "Browser",
        "Reactive DOM properties such as hidden and value",
    ),
    (
        "Event handlers",
        "Browser",
        "Rust closures compiled into browser JavaScript",
    ),
    (
        "Shards",
        "Worker",
        "Server-rendered components driven by reactive arguments",
    ),
    (
        "Procedures",
        "Worker",
        "Async server functions callable from the browser",
    ),
    (
        "Components",
        "Worker",
        "Async Rust functions that return composable views",
    ),
    (
        "Cookies",
        "Worker",
        "Request-scoped typed state written through response headers",
    ),
    (
        "HTMX",
        "Worker",
        "Explicit fragment requests and typed response headers",
    ),
];

#[page("/reactivity")]
pub async fn reactivity() -> Result {
    view! {
        page_header(
            kicker: "Client + server reactivity",
            title: "One syntax, two execution boundaries.",
            description: "The counter and disclosure update immediately in your browser. The search input passes its value to a shard, which asks the Worker for fresh HTML.",
            execution: "Browser → Worker",
        )

        <section class="grid gap-6 xl:grid-cols-2">
            card(
                attrs: attributes! { data-demo-card="" class="h-fit" },
                card_header(
                    <div class="mb-2 flex items-center justify-between gap-3">
                        badge(variant: BadgeVariant::Secondary, "Browser")
                        <code>"signal count"</code>
                    </div>
                    card_title("Instant client expressions")
                    card_description("These controls do not make a network request.")
                )
                card_content(
                    signal count = 2.0;
                    signal details = false;

                    <div class="rounded-xl border border-border bg-foreground/[0.025] p-5">
                        <div class="flex items-center justify-between gap-4">
                            <div>
                                <p class="text-xs font-medium tracking-wide text-muted-foreground uppercase">
                                    "Release confidence"
                                </p>
                                <p class="mt-1 text-3xl font-semibold tabular-nums">
                                    $(count.get())
                                    <span class="text-sm font-normal text-muted-foreground">" / 10"</span>
                                </p>
                            </div>
                            <div class="flex gap-2">
                                button(
                                    size: ButtonSize::Icon,
                                    variant: ButtonVariant::Outline,
                                    attrs: attributes! {
                                        type="button"
                                        aria-label="Decrease confidence"
                                        @click=$(|_event: Event| count.set(count.get() - 1.0))
                                    },
                                    "−"
                                )
                                button(
                                    size: ButtonSize::Icon,
                                    attrs: attributes! {
                                        type="button"
                                        aria-label="Increase confidence"
                                        @click=$(|_event: Event| count.set(count.get() + 1.0))
                                    },
                                    "+"
                                )
                            </div>
                        </div>
                        <progress
                            class="mt-5 h-2 w-full overflow-hidden rounded-full accent-current"
                            max="10"
                            :value=$(count.get())
                        ></progress>
                    </div>

                    <div class="mt-4 rounded-xl border border-border p-5">
                        <div class="flex items-center justify-between gap-4">
                            <div>
                                <p class="text-sm font-medium">"How did that update?"</p>
                                <p class="text-xs text-muted-foreground">"Toggle a reactive attribute."</p>
                            </div>
                            button(
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Ghost,
                                attrs: attributes! {
                                    type="button"
                                    @click=$(|_event: Event| details.set(!details.get()))
                                },
                                $(if details.get() { "Hide" } else { "Reveal" })
                            )
                        </div>
                        <p
                            class="mt-4 border-l-2 border-primary pl-4 text-sm leading-6 text-muted-foreground"
                            :hidden=$(!details.get())
                        >
                            "Topcoat evaluated the Rust expression for the initial HTML, translated the same expression to JavaScript, and bound it to this DOM region."
                        </p>
                    </div>
                )
            )

            card(
                attrs: attributes! { data-demo-card="" },
                card_header(
                    <div class="mb-2 flex items-center justify-between gap-3">
                        badge("Browser → Worker")
                        <code>"#[shard]"</code>
                    </div>
                    card_title("Server-filtered component catalog")
                    card_description("Every changed query re-renders only the results fragment at the edge.")
                )
                card_content(
                    signal query = String::new();

                    <div class="relative">
                        input(
                            attrs: attributes! {
                                id="feature-search"
                                type="search"
                                placeholder="Try browser, worker, shard..."
                                autocomplete="off"
                                :value=$(query.get())
                                @input=$(|event: Event| query.set(event.target.value))
                            }
                        )
                    </div>
                    search_results(query: $(query.get()))
                )
            )
        </section>

        <div class="mt-6">
            demo_note(
                title: "Watch the Network panel",
                <p>
                    "The left card stays completely local. Typing on the right calls Topcoat's generated shard endpoint and swaps a server-rendered fragment—there is no hand-written JSON API."
                </p>
            )
        </div>
    }
}

#[shard]
pub async fn search_results(query: String) -> Result {
    let needle = query.trim().to_ascii_lowercase();
    let matches = EXAMPLES
        .iter()
        .filter(|(name, execution, description)| {
            needle.is_empty()
                || name.to_ascii_lowercase().contains(&needle)
                || execution.to_ascii_lowercase().contains(&needle)
                || description.to_ascii_lowercase().contains(&needle)
        })
        .collect::<Vec<_>>();

    view! {
        <div class="mt-4" aria-live="polite">
            <div class="mb-2 flex items-center justify-between text-xs text-muted-foreground">
                <span>(matches.len()) " matches"</span>
                <span class="font-mono">"HTML from Worker"</span>
            </div>
            if matches.is_empty() {
                <div class="rounded-lg border border-dashed border-border px-4 py-8 text-center text-sm text-muted-foreground">
                    "No Topcoat feature matched that query."
                </div>
            } else {
                <ul class="divide-y divide-border rounded-lg border border-border">
                    for (name, execution, description) in matches {
                        <li class="p-3 first:rounded-t-lg last:rounded-b-lg">
                            <div class="flex items-center justify-between gap-3">
                                <span class="text-sm font-medium">(name)</span>
                                badge(
                                    variant: if *execution == "Browser" { BadgeVariant::Secondary } else { BadgeVariant::Outline },
                                    (execution)
                                )
                            </div>
                            <p class="mt-1 text-xs leading-5 text-muted-foreground">(description)</p>
                        </li>
                    }
                </ul>
            }
        </div>
    }
}
