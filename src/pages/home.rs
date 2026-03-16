use leptos::prelude::*;

use crate::components::navbar_name::NavbarName;
use crate::components::hero::Hero;
use crate::components::intro_card::IntroCard;
use crate::components::summary::Summary;
use crate::components::details::Details;
use crate::components::footer_links::FooterLinks;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="min-h-screen bg-zinc-100 px-6 py-4 text-zinc-900">
            <div class="mx-auto flex w-full max-w-5xl flex-col">
                <NavbarName />

                <div class="mt-8 flex flex-col items-center gap-10">
                    <Hero />
                    <IntroCard />
                    <Summary />
                    <Details />
                    <FooterLinks />
                </div>
            </div>
        </main>
    }
}
