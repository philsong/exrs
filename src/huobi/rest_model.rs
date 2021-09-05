use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub page: Page,
    pub components: Vec<Component>,
    pub incidents: Vec<::serde_json::Value>,
    pub scheduled_maintenances: Vec<::serde_json::Value>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub name: String,
    pub url: String,
    pub time_zone: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub position: i64,
    pub description: ::serde_json::Value,
    pub showcase: bool,
    pub start_date: ::serde_json::Value,
    pub group_id: Option<String>,
    pub page_id: String,
    pub group: bool,
    pub only_show_if_degraded: bool,
    pub components: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub indicator: String,
    pub description: String,
}
