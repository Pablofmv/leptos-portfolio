use gloo_net::http::Request;
use leptos::prelude::*;

use crate::models::{
    AnalyticsCountResponse,
    ClickCountResponse,
    UniqueVisitorCountResponse,
};


const API_BASE_URL: &str = "https://me.pablomendoza.site";

async fn fetch_json<T>(
    url: &str,
) -> Result<T, String>
where T: serde::de::DeserializeOwned, {

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


    let total_clicks = LocalResource::new(|| async move {

        fetch_json::<ClickCountResponse>(
            &format!("{API_BASE_URL}/analytics/clicks/total"),
        )
        .await

    });


    let unique_visitors = LocalResource::new(|| async move {
        
        fetch_json::<UniqueVisitorCountResponse>(
            &format!("{API_BASE_URL}/analytics/visitors/unique"),
        )
        .await
        
    });

    let device_analytics = LocalResource::new(|| async move {

        fetch_json::<Vec<AnalyticsCountResponse>>(
            &format!("{API_BASE_URL}/analytics/devices"),
        )
        .await

    });

    let referrer_analytics = LocalResource::new(|| async move {
        
        fetch_json::<Vec<AnalyticsCountResponse>>(
            &format!("{API_BASE_URL}/analytics/referrers"),
        )
        .await
        
    });



    view! {

        <main>

            <h1> "Analytics Dashboard" </h1>

                <section>

                    <h2> "Total Clicks" </h2>

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
                    <h2> "Unique Vistors" </h2>

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
                <h2> "Device Analytics" </h2>

                {move || {
                    match device_analytics.get() {
                        None => view! {
                            <p>"Loading devie analytics ..."</p>
                        }.into_any(),

                        Some(Ok(items)) => view!{
                            <ul> 
                            {items.into_iter().map(|item| {
                                view!{
                                    <li>{item.name}":"{item.count}</li>
                                }
                            })
                            .collect_view()}
                            </ul>
                        }.into_any(),

                        Some(Err(_)) => view! {
                            <p> "Unable to load device analytics." </p>
                        }.into_any(),
                    }}
                }

                </section>

            <section>
                <h2> "Referrer Analytics" </h2>

                {move || {
                    match referrer_analytics.get() {
                        None => view! {
                            <p>"Loading referrer analytics ..."</p>
                        }.into_any(),

                        Some(Ok(items)) => view!{
                            <ul> 
                            {items.into_iter().map(|item| {
                                view!{
                                    <li>{item.name}":"{item.count}</li>
                                }
                            })
                            .collect_view()}
                            </ul>
                        }.into_any(),

                        Some(Err(_)) => view! {
                            <p> "Unable to load referrer analytics." </p>
                        }.into_any(),
                    }}
                }
            </section>

        </main>


    }

}