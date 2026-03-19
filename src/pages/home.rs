use leptos::prelude::*;

use crate::components::footer_links::FooterLinks;
use crate::components::hero::Hero;
use crate::components::intro_card::IntroCard;
use crate::components::summary::Summary;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="page">
            <div class="container">
                <section class="section section--hero">
                    <Hero />
                </section>

                <section class="section section--compact">
                    <IntroCard />
                </section>

                <section class="section section--compact">
                    <Summary />
                </section>

            </div>

            <FooterLinks />
        </main>
    }
}