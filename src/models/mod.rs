use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct ClickCountResponse {
    pub total_clicks: usize,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct UniqueVisitorCountResponse {
    pub unique_visitors: usize,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct AnalyticsCountResponse {
    pub name: String,
    pub count: i64,
}