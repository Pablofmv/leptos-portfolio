use leptos::prelude::*;

#[component]
pub fn IntroCard() -> impl IntoView {
    view! {
        <section class="w-full max-w-2xl rounded-md bg-zinc-200 p-4 shadow-sm">
            <div class="rounded-md bg-zinc-300 px-6 py-5 text-left font-mono text-lg leading-8 text-zinc-700">
                <p><span class="text-fuchsia-600">"> "</span>"Rust Developer in NYC"</p>
                <p><span class="text-fuchsia-600">"> "</span>"Building a minimalist Leptos portfolio"</p>
                <p class="pl-8 text-zinc-600">
                    <span class="text-fuchsia-600">">> "</span>
                    "<Learning full-stack Rust one component at a time>"
                </p>
            </div>
        </section>
    }
}
