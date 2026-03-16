use leptos::prelude::*;

#[component]
pub fn NavbarName() -> impl IntoView {
    view! {
        <header class="pt-2">
            <p class="text-sm tracking-tight text-neutral-800">
                "Pablo Mendoza"
            </p>
        </ header>
    }
}