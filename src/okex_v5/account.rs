use serde::Serializer;
use std::collections::BTreeMap;
use std::fmt;

use super::client::Client;
use super::errors::*;
// use super::rest_model::CancelAllOpenOrdersResponse;
// use super::rest_model::MultiAssetsMarginResponse;
// use super::rest_model::PositionModeResponse;
// use super::rest_model::{
//     AccountBalance, CanceledOrder, ChangeLeverageResponse, OrderType, Position, Transaction,
// };
// use super::rest_model::{OrderSide, TimeInForce};
// use super::rest_model::{PairAndWindowQuery, PairQuery};
use super::util::*;

#[derive(Clone)]
pub struct Account {
    pub client: Client,
}

