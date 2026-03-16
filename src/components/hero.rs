use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="flex flex-col items-center gap-6 text-center">
            <div class="flex h-28 w-28 items-center justify-center rounded-[28px] bg-black text-6xl font-bold text-white shadow-sm">
                "n"
            </div>

            <h1 class="text-5xl font-extrabold tracking-tight text-zinc-800 sm:text-6xl">
                "Hi 👋 I’m Pablo!"
            </h1>

        </section>
    }
}