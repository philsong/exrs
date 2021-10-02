use serde_json::from_str;

use super::client::*;
use super::errors::*;
use super::rest_model::*;
use super::util::bool_to_string;

#[derive(Clone)]
pub struct Market {
    pub client: Client,
}
