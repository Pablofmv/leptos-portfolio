use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <h1 class="title-xl">
                "Hello I’m Pablo!"
            </h1>
        </section>
    }
}