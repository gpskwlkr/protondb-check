use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GamesList {
    #[serde(rename = "steamID64")]
    pub steam_id64: String,
    #[serde(rename = "steamID")]
    pub steam_id: String,
    pub games: Games,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Games {
    pub game: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    #[serde(rename = "appID")]
    pub app_id: u32,
    pub name: String,
    pub logo: String,
    #[serde(rename = "storeLink")]
    pub store_link: String,

    #[serde(rename = "hoursOnRecord")]
    pub hours_on_record: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtonAPIResponse {
    #[serde(rename = "bestReportedTier")]
    pub best_reported_tier: String,
    pub confidence: String,
    pub score: f32,
    pub tier: String,
    pub total: u32,
    #[serde(rename = "trendingTier")]
    pub trending_tier: String,
}
