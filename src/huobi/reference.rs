use std::cell::Ref;

use crate::huobi::client::*;
use crate::huobi::errors::*;
use crate::huobi::rest_model::*;

use serde_json::from_str;

#[derive(Clone)]
pub struct Reference {
    pub client: Client,
}

impl Reference {
    pub async fn summary(&self) -> Result<String> {
        let data: String = self
            .client
            .get("https://status.huobigroup.com/api/v2/summary.json", "")
            .await?;

        Ok(data)
    }
}
