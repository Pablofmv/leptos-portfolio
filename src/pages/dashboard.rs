use gloo_net::http::Request;
use leptos::{logging, prelude::*, task::spawn_local};

use crate::models::{AnalyticsCountResponse, ClickCountResponse, UniqueVisitorCountResponse};

const API_BASE_URL: &str = "https://me.pablomendoza.site";

async fn fetch_json<T>(url: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = Request::get(url)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    response
        .json::<T>()
        .await
        .map_err(|error| error.to_string())
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let (total_clicks, set_total_clicks) =
        signal::<Option<Result<ClickCountResponse, String>>>(None);

    let (unique_visitors, set_unique_visitors) =
        signal::<Option<Result<UniqueVisitorCountResponse, String>>>(None);

    let (device_analytics, set_device_analytics) =
        signal::<Option<Result<Vec<AnalyticsCountResponse>, String>>>(None);

    let (referrer_analytics, set_referrer_analytics) =
        signal::<Option<Result<Vec<AnalyticsCountResponse>, String>>>(None);

    Effect::new(move |_| {
        logging::log!("Dashboard effect started");

        spawn_local(async move {
            logging::log!("Fetching analytics...");

            set_total_clicks.set(Some(
                fetch_json::<ClickCountResponse>(&format!("{API_BASE_URL}/analytics/clicks/total"))
                    .await,
            ));

            set_unique_visitors.set(Some(
                fetch_json::<UniqueVisitorCountResponse>(&format!(
                    "{API_BASE_URL}/analytics/visitors/unique"
                ))
                .await,
            ));

            set_device_analytics.set(Some(
                fetch_json::<Vec<AnalyticsCountResponse>>(&format!(
                    "{API_BASE_URL}/analytics/devices"
                ))
                .await,
            ));

            set_referrer_analytics.set(Some(
                fetch_json::<Vec<AnalyticsCountResponse>>(&format!(
                    "{API_BASE_URL}/analytics/referrers"
                ))
                .await,
            ));
        });
    });

    view! {
        <main class = "page">
            <div class ="container">
                <section class ="hero">
                    <section class ="hero">
                        <h1 class ="title-xl">"Analytics Dashboard"</h1>
                    </section>
                </section>

            <section class ="Section section--compact">
                <section class="intro">
                    <div class = "intro__card">
                        <p class = "intro__text">
                            <span class="intro__symbol"> " >"</span>
                            "Total Clicks"
                        </p>

                        <p class ="intro__text">
                        {move || {
                            match total_clicks.get() {
                                None => "Loading total clicks...".to_string(),

                                Some(Ok(data)) => data.total_clicks.to_string(),

                                Some(Err(_)) => "unable to load total clicks.".to_string(),
                            }
                        }}
                        </p>
                    </div>
                </section>
            </section>

            <section class="section section--compact">
                <section class="intro">
                    <div class="intro__card">
                        <p class="intro__text">
                            <span class="intro__symbol">"> "</span>
                            "Unique Visitors"
                        </p>

                        <p class="intro__text">
                            {move || {
                                match unique_visitors.get() {
                                    None => "Loading unique visitors...".to_string(),
                                    Some(Ok(data)) => data.unique_visitors.to_string(),
                                    Some(Err(_)) => "Unable to load unique visitors.".to_string(),
                                }
                            }}
                        </p>
                    </div>
                </section>
            </section>

            <section class="section section--compact">
                <section class="intro">
                    <div class="intro__card">
                        <p class="intro__text">
                            <span class="intro__symbol">"> "</span>
                            "Device Analytics"
                        </p>

                        {move || {
                            match device_analytics.get() {
                                None => view! {
                                    <p class="intro__text">"Loading device analytics..."</p>
                                }.into_any(),

                                Some(Ok(items)) => view! {
                                    <>
                                        {items
                                            .into_iter()
                                            .map(|item| {
                                                view! {
                                                    <p class="intro__text">
                                                        {item.name}": "{item.count}
                                                    </p>
                                                }
                                            })
                                            .collect_view()}
                                    </>
                                }.into_any(),

                                Some(Err(_)) => view! {
                                    <p class="intro__text">"Unable to load device analytics."</p>
                                }.into_any(),
                            }
                        }}
                    </div>
                </section>
            </section>
            </div>
        </main>
    }
}
