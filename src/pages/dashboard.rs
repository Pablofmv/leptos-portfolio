use gloo_net::http::Request;
use leptos::{
    logging,
    prelude::*,
    task::spawn_local,
};

use crate::models::{
    AnalyticsCountResponse,
    ClickCountResponse,
    UniqueVisitorCountResponse,
};

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
                fetch_json::<ClickCountResponse>(
                    &format!("{API_BASE_URL}/analytics/clicks/total"),
                )
                .await,
            ));

            set_unique_visitors.set(Some(
                fetch_json::<UniqueVisitorCountResponse>(
                    &format!("{API_BASE_URL}/analytics/visitors/unique"),
                )
                .await,
            ));

            set_device_analytics.set(Some(
                fetch_json::<Vec<AnalyticsCountResponse>>(
                    &format!("{API_BASE_URL}/analytics/devices"),
                )
                .await,
            ));

            set_referrer_analytics.set(Some(
                fetch_json::<Vec<AnalyticsCountResponse>>(
                    &format!("{API_BASE_URL}/analytics/referrers"),
                )
                .await,
            ));
        });
    });

    view! {
        <main>
            <h1>"Analytics Dashboard"</h1>

            <section>
                <h2>"Total Clicks"</h2>

                {move || {
                    match total_clicks.get() {
                        None => view! {
                            <p>"Loading total clicks..."</p>
                        }.into_any(),

                        Some(Ok(data)) => view! {
                            <p>{data.total_clicks}</p>
                        }.into_any(),

                        Some(Err(_)) => view! {
                            <p>"Unable to load total clicks."</p>
                        }.into_any(),
                    }
                }}
            </section>

            <section>
                <h2>"Unique Visitors"</h2>

                {move || {
                    match unique_visitors.get() {
                        None => view! {
                            <p>"Loading unique visitors..."</p>
                        }.into_any(),

                        Some(Ok(data)) => view! {
                            <p>{data.unique_visitors}</p>
                        }.into_any(),

                        Some(Err(_)) => view! {
                            <p>"Unable to load unique visitors."</p>
                        }.into_any(),
                    }
                }}
            </section>

            <section>
                <h2>"Device Analytics"</h2>

                {move || {
                    match device_analytics.get() {
                        None => view! {
                            <p>"Loading device analytics..."</p>
                        }.into_any(),

                        Some(Ok(items)) => view! {
                            <ul>
                                {items
                                    .into_iter()
                                    .map(|item| {
                                        view! {
                                            <li>{item.name}": "{item.count}</li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        }.into_any(),

                        Some(Err(_)) => view! {
                            <p>"Unable to load device analytics."</p>
                        }.into_any(),
                    }
                }}
            </section>

            <section>
                <h2>"Referrer Analytics"</h2>

                {move || {
                    match referrer_analytics.get() {
                        None => view! {
                            <p>"Loading referrer analytics..."</p>
                        }.into_any(),

                        Some(Ok(items)) => view! {
                            <ul>
                                {items
                                    .into_iter()
                                    .map(|item| {
                                        view! {
                                            <li>{item.name}": "{item.count}</li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        }.into_any(),

                        Some(Err(_)) => view! {
                            <p>"Unable to load referrer analytics."</p>
                        }.into_any(),
                    }
                }}
            </section>
        </main>
    }
}