use leptos::prelude::*;

#[component]
pub fn Details() -> impl IntoView {
    view! {
        <section class="w-full max-w-4xl">
            <div class="max-w-2xl space-y-3 text-[1.05rem] leading-8 text-zinc-700">
                <p>"📍 Based in NYC"</p>
                <p>"⚡ Obsessed with performance and data"</p>
            </div>
        </section>
    }
}
