#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairQuery {
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PositionSide {
    Net,
    Long,
    Short,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
/// market: market order
/// limit: limit order
/// post_only: Post-only order
/// fok: Fill-or-kill order
/// ioc: Immediate-or-cancel order
/// optimal_limit_ioc :Market order with immediate-or-cancel order
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
    PostOnly,
    FOK,
    IOC,
    OptimalLimitIoc,
    #[serde(other)]
    Other,
}

/// By default, buy
impl Default for OrderSide {
    fn default() -> Self {
        Self::Buy
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TradeMode {
    Isolated,
    Cross,
    Cash,
}

/// By default, Cross
impl Default for TradeMode {
    fn default() -> Self {
        Self::Cross
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    #[serde(rename = "inst_id")]
    pub symbol: String,
    #[serde(rename = "td_mode")]
    pub trade_mode: TradeMode,
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub side: OrderSide,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    #[serde(rename = "ord_type")]
    pub order_type: OrderType,
    #[serde(rename = "sz", with = "string_or_float")]
    pub qty: f64,
    #[serde(
        rename = "px",
        with = "string_or_float_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    pub target_currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub cl_ord_id: String,
    pub ord_id: String,
    pub tag: String,
    pub s_code: String,
    pub s_msg: String,
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => super::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(super::string_or_float::deserialize(deserializer)?))
    }
}

pub(crate) mod string_or_uint {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            UInt(u64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::UInt(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}

fn default_stop_price() -> f64 {
    0.0
}
fn default_activation_price() -> f64 {
    0.0
}
fn default_price_rate() -> f64 {
    0.0
}
