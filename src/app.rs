use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{
        Route,
        Router,
        Routes,
    },
    path,
};

use crate::pages::dashboard::DashboardPage;
use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Pablo Mendoza Portfolio"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Stylesheet id="leptos" href="/pkg/leptos-portfolio-v2.css"/>
        
        <Router>

            <Routes fallback =|| view! { <HomePage /> }>

                <Route path=path!("/") view=HomePage />
                <Route path=path!("/dashboard") view=DashboardPage/>

            </Routes>

        </Router>

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
                <Stylesheet id="leptos" href="/pkg/leptos-portfolio-v2.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
