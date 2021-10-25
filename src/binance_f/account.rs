use serde::Serializer;
use std::collections::BTreeMap;
use std::fmt;

use super::client::Client;
use super::errors::*;
use super::rest_model::CancelAllOpenOrdersResponse;
use super::rest_model::MultiAssetsMarginResponse;
use super::rest_model::PositionModeResponse;
use super::rest_model::{
    AccountBalance, CanceledOrder, ChangeLeverageResponse, OrderType, Position, Transaction,
};
use super::rest_model::{OrderSide, TimeInForce};
use super::rest_model::{PairAndWindowQuery, PairQuery};
use super::util::*;

static FAPI_ORDER: &str = "/fapi/v1/order";
static FAPI_OPEN_ORDERS: &str = "/fapi/v2/openOrders";
static FAPI_ALL_OPEN_ORDERS: &str = "/fapi/v1/allOpenOrders";
static FAPI_POSITION_RISK: &str = "/fapi/v2/positionRisk";
static FAPI_BALANCE: &str = "/fapi/v2/balance";
static FAPI_LEVERAGE: &str = "/fapi/v1/leverage";
static FAPI_POSITION_SIDE_DUAL: &str = "/fapi/v1/positionSide/dual";
static FAPI_MULTI_ASSETS_MARGIN: &str = "/fapi/v1/multiAssetsMargin";

#[derive(Clone)]
pub struct FuturesAccount {
    pub client: Client,
    pub recv_window: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
}

/// By default, use Perpetual
impl Default for ContractType {
    fn default() -> Self {
        Self::Perpetual
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
    #[serde(other)]
    Other,
}

/// Serialize bool as str
fn serialize_as_str<S, T>(t: &T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: fmt::Display,
{
    serializer.collect_str(t)
}

/// Serialize opt bool as str
fn serialize_opt_as_uppercase<S, T>(
    t: &Option<T>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    match *t {
        Some(ref v) => serializer.serialize_some(&v.to_string().to_uppercase()),
        None => serializer.serialize_none(),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "quantity")]
    pub qty: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub close_position: Option<bool>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    #[serde(serialize_with = "serialize_opt_as_uppercase")]
    pub price_protect: Option<bool>,
}

/// Order Cancellation Request
/// perform an order cancellation for the account
/// only works if the parameters match an active order
/// either order_id (binance side id) or orig_client_order_id (id originally given by the client) must be set
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancellation {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
    /// Cannot be greater than 60000
    pub recv_window: Option<u64>,
    pub timestamp: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ChangePositionModeRequest {
    #[serde(serialize_with = "serialize_as_str")]
    pub dual_side_position: bool,
}

/// todo: BatchOrder
struct BatchOrdersRequest {
    pub batch_orders: Vec<OrderRequest>,
}

impl FuturesAccount {
    async fn post_order(&self, order: OrderRequest) -> Result<Transaction> {
        self.client
            .post_signed_p(FAPI_ORDER, order, self.recv_window)
            .await
    }

    pub async fn limit_buy(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: impl Into<f64>,
        position_side: PositionSide,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: Some(position_side),
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price.into()),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        self.post_order(order).await
    }

    pub async fn limit_sell(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: impl Into<f64>,
        position_side: PositionSide,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: Some(position_side),
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price.into()),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        self.post_order(order).await
    }

    // Place a MARKET order - BUY
    pub async fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            qty: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        self.post_order(order).await
    }

    // Place a MARKET order - SELL
    pub async fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            qty: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        self.post_order(order).await
    }

    /// Place a cancellation order
    pub async fn cancel_order(&self, o: OrderCancellation) -> Result<CanceledOrder> {
        let recv_window = o.recv_window.unwrap_or(self.recv_window);
        self.client
            .delete_signed_p(FAPI_ORDER, &o, recv_window)
            .await
    }

    pub async fn position_information<S>(&self, symbol: S) -> Result<Vec<Position>>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                FAPI_POSITION_RISK,
                Some(PairAndWindowQuery {
                    symbol: symbol.into(),
                    recv_window: self.recv_window,
                }),
                self.recv_window,
            )
            .await
    }

    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed_d(FAPI_BALANCE, request.as_str())
            .await
    }

    pub async fn change_initial_leverage<S>(
        &self,
        symbol: S,
        leverage: u8,
    ) -> Result<ChangeLeverageResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed_d(FAPI_LEVERAGE, request.as_str())
            .await
    }

    pub async fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        self.client
            .post_signed_p(
                FAPI_POSITION_SIDE_DUAL,
                ChangePositionModeRequest { dual_side_position },
                self.recv_window,
            )
            .await?;
        Ok(())
    }

    pub async fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<CancelAllOpenOrdersResponse>
    where
        S: Into<String>,
    {
        self.client
            .delete_signed_p(
                FAPI_ALL_OPEN_ORDERS,
                PairQuery {
                    symbol: symbol.into(),
                },
                self.recv_window,
            )
            .await
    }

    pub async fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<Transaction>>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                FAPI_OPEN_ORDERS,
                Some(PairAndWindowQuery {
                    symbol: symbol.into(),
                    recv_window: self.recv_window,
                }),
                self.recv_window,
            )
            .await
    }

    pub async fn get_position_mode(&self) -> Result<PositionModeResponse> {
        let parameters = BTreeMap::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed_d(FAPI_MULTI_ASSETS_MARGIN, request.as_str())
            .await
    }

    pub async fn get_multi_assets_mode(&self) -> Result<MultiAssetsMarginResponse> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed_d(FAPI_MULTI_ASSETS_MARGIN, request.as_str())
            .await
    }

    pub async fn change_multi_assets_mode<S>(&self, mutl_assets_margin: S) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("multiAssetsMargin".into(), mutl_assets_margin.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed_d(FAPI_MULTI_ASSETS_MARGIN, request.as_str())
            .await?;
        Ok(())
    }
}
