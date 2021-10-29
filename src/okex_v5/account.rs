use serde::de::DeserializeOwned;
use serde::Serializer;
use std::collections::BTreeMap;
use std::fmt;

use super::client::Client;
use super::errors::*;
use super::rest_model::*;
// use super::rest_model::CancelAllOpenOrdersResponse;
// use super::rest_model::MultiAssetsMarginResponse;
// use super::rest_model::PositionModeResponse;
// use super::rest_model::{
//     AccountBalance, CanceledOrder, ChangeLeverageResponse, OrderType, Position, Transaction,
// };
// use super::rest_model::{OrderSide, TimeInForce};
// use super::rest_model::{PairAndWindowQuery, PairQuery};
use super::util::*;

// trade
static API_V5_ORDER: &str = "/api/v5/trade/order";
static API_V5_BATCH_ORDERS: &str = "/api/v5/trade/batch-orders";
static API_V5_CANCEL_ORDER: &str = "/api/v5/trade/cancel-order";
static API_V5_CANCEL_BATCH_ORDERS: &str = "/api/v5/trade/cancel-batch-orders";
static API_V5_AMEND_ORDER: &str = "/api/v5/trade/amend-order";
static API_V5_AMEND_BATCH_ORDERS: &str = "/api/v5/trade/amend-batch-orders";
static API_V5_CLOSE_POSITION: &str = "/api/v5/trade/close-position";
static API_V5_ORDERS_PENDING: &str = "/api/v5/trade/orders-pending";
static API_v5_ORDERS_HISTORY: &str = "/api/v5/trade/orders-history";
static API_V5_ORDERS_HISOTRY_ARCHIVE: &str = "/api/v5/trade/orders-history-archive";
static API_V5_FILLS: &str = "/api/v5/trade/fills";
static API_V5_FILLS_HISTORY: &str = "/api/v5/trade/fills-history";
static API_V5_ORDER_ALGO: &str = "/api/v5/trade/order-algo";
static API_V5_CANCEL_ALGOS: &str = "/api/v5/trade/cancel-algos";
static API_V5_CANCEL_ADVANCE_ALGOS: &str = "/api/v5/trade/cancel-advance-algos";
static API_V5_ORDERS_ALGO_PENDING: &str = "/api/v5/trade/orders-algo-pending";
static API_V5_ORDERS_ALGO_HISTORY: &str = "/api/v5/trade/orders-algo-history";

// account
static API_V5_POSITIONS: &str = "/api/v5/account/positions";
static API_V5_ACCOUNT_POSITION_RISK: &str = "/api/v5/account/account-position-risk";
static API_V5_BILLS: &str = "/api/v5/account/bills";
static API_V5_BILLS_ARCHIVE: &str = "/api/v5/account/bills-archive";
static API_V5_ACCOUNT_CONFIG: &str = "/api/v5/account/config";
static API_V5_SET_POSITION_MODE: &str = "/api/v5/account/set-position-mode";
static API_V5_SET_LEVERAGE: &str = "/api/v5/account/set-leverage";
static API_V5_MAX_SIZE: &str = "/api/v5/account/max-size";
static API_V5_MAX_AVAIL_SIZE: &str = "/api/v5/account/max-avail-size";
static API_V5_MARGIN_BALANCE: &str = "/api/v5/account/position/margin-balance";
static API_V5_LEVERAGE_INFO: &str = "/api/v5/account/leverage-info";
static API_V5_MAX_LOAN: &str = "/api/v5/account/max-loan";
static API_V5_TRADE_FEE: &str = "/api/v5/account/trade-fee";
static API_V5_INTEREST_ACCRUED: &str = "/api/v5/account/interest-accrued";
static API_V5_INTEREST_RATE: &str = "/api/v5/account/interest-rate";
static API_V5_SET_GREEKS: &str = "/api/v5/account/set-greeks";
static API_V5_MAX_WITHDRAWAL: &str = "/api/v5/account/max-withdrawal";

// todo
// sub account

#[derive(Clone)]
pub struct Account {
    pub client: Client,
}

impl Account {
    async fn post_order<O>(&self, order: O) -> Result<TransactionResponse>
    where
        O: serde::Serialize,
    {
        self.client.post_signed_p(API_V5_ORDER, order).await
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        position_side: PositionSide,
        client_order_id: S,
    ) -> Result<TransactionResponse>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            trade_mode: TradeMode::Cross,
            currency: None,
            client_order_id: Some(client_order_id.into()),
            tag: None,
            side: OrderSide::Buy,
            position_side: Some(position_side),
            order_type: OrderType::Limit,
            qty: qty.into(),
            price: price.into(),
            reduce_only: None,
            target_currency: None,
        };
        self.post_order(order).await
    }

    pub async fn limit_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        position_side: PositionSide,
        client_order_id: S,
    ) -> Result<TransactionResponse>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            trade_mode: TradeMode::Cross,
            currency: None,
            client_order_id: Some(client_order_id.into()),
            tag: None,
            side: OrderSide::Sell,
            position_side: Some(position_side),
            order_type: OrderType::Limit,
            qty: qty.into(),
            price: price.into(),
            reduce_only: None,
            target_currency: None,
        };
        self.post_order(order).await
    }

    pub async fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<TransactionResponse>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order = OrderRequest {
            symbol: symbol.into(),
            trade_mode: TradeMode::Cross,
            currency: None,
            client_order_id: None,
            tag: None,
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            order_type: OrderType::Market,
            qty: qty.into(),
            price: None,
            reduce_only: None,
            target_currency: None,
        };
        self.post_order(order).await
    }

    pub async fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<TransactionResponse>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order = OrderRequest {
            symbol: symbol.into(),
            trade_mode: TradeMode::Cross,
            currency: None,
            client_order_id: None,
            tag: None,
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Short),
            order_type: OrderType::Market,
            qty: qty.into(),
            price: None,
            reduce_only: None,
            target_currency: None,
        };
        self.post_order(order).await
    }

    pub async fn close_position<S>(
        &self,
        symbol: S,
        pos_side: Option<PositionSide>,
    ) -> Result<TransactionResponse>
    where
        S: Into<String>,
    {
        let order = ClosePositionRequest {
            symbol: symbol.into(),
            pos_side: pos_side,
            margin_mode: MarginMode::Cross,
            currency: None,
        };
        self.client
            .post_signed_p(API_V5_CLOSE_POSITION, &order)
            .await
    }

    /// Place a cancellation order
    pub async fn cancel_order(&self, order: OrderCancellation) -> Result<TransactionResponse> {
        self.client.post_signed_p(API_V5_CANCEL_ORDER, &order).await
    }

    pub async fn cancel_all_open_orders(
        &self,
        order: Vec<OrderCancellation>,
    ) -> Result<TransactionResponse> {
        self.client
            .post_signed_p(API_V5_CANCEL_BATCH_ORDERS, &order)
            .await
    }
}
