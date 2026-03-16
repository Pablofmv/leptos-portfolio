use leptos::prelude::*;
use leptos_meta::*;

use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Pablo Mendoza Portfolio"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Stylesheet id="leptos" href="/pkg/leptos-portfolio.css"/>
        <HomePage/>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
