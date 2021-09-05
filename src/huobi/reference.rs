use std::cell::Ref;

use crate::huobi::client::*;
use crate::huobi::errors::*;
use crate::huobi::rest_model::*;

use serde_json::{from_str, Value};

#[derive(Clone)]
pub struct Reference {
    pub client: Client,
}

impl Reference {
    pub async fn summary(&self) -> Result<Summary> {
        todo!()
    }
}
