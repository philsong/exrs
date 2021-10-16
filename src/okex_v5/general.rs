use super::client::*;
use super::errors::*;
use super::rest_model::*;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}
