use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStat {
    pub date: String,
    pub focus_min: i64,
    pub rounds: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResult {
    pub focus_min: i64,
    pub rounds: i64,
    pub completion_rate: Option<f64>,
    pub break_min: i64,
    pub daily: Vec<DailyStat>,
}
