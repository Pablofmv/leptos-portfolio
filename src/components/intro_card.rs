use leptos::prelude::*;

#[component]
pub fn IntroCard() -> impl IntoView {
    view! {
        <section class="intro">
            <div class="intro__card">
                <p class="intro__text">
                    <span class="intro__symbol">"> "</span>
                    "🦀 Rust and 🐍 Python developer"
                </p>
            </div>
        </section>
    }
}