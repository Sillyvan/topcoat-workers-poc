pub mod cookies;
pub mod home;
pub mod htmx;
pub mod preferences;
pub mod reactivity;

use topcoat::{
    view::{component, view, View},
    Result,
};

use crate::components::badge::{badge, BadgeVariant};

#[component]
pub async fn page_header(
    kicker: &'static str,
    title: &'static str,
    description: &'static str,
    execution: &'static str,
) -> Result {
    let variant = match execution {
        "Browser" => BadgeVariant::Secondary,
        "Worker" => BadgeVariant::Outline,
        _ => BadgeVariant::Primary,
    };

    view! {
        <header class="mb-8 max-w-3xl">
            <div class="mb-4 flex flex-wrap items-center gap-2">
                <span
                    class="font-mono text-xs font-semibold tracking-[0.18em] text-muted-foreground uppercase"
                >
                    (kicker)
                </span>
                badge(variant: variant, (execution))
            </div>
            <h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">(title)</h1>
            <p class="mt-3 max-w-2xl text-base leading-7 text-muted-foreground">
                (description)
            </p>
        </header>
    }
}

#[component]
pub async fn demo_note(title: &'static str, child: View) -> Result {
    view! {
        <aside
            class="rounded-lg border border-dashed border-border bg-foreground/[0.025] p-4 text-sm"
        >
            <p class="font-medium">(title)</p>
            <div class="mt-1 leading-6 text-muted-foreground">(child)</div>
        </aside>
    }
}
