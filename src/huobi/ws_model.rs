#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ping {
    ping: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pong {
    pong: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubRequest {
    sub: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollRequest {
    req: String,
    id: String,
    from: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnSubRequest {
    sub: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubResponseEvent {
    id: String,
    status: String,
    subbed: String,
    #[serde(rename = "ts")]
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnSubResponseEvent {
    id: String,
    status: String,
    unsubbed: String,
    #[serde(rename = "ts")]
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineEvent {
    #[serde(rename = "ch")]
    pub channel: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub tick: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub vol: f64,
    #[serde(with = "string_or_float")]
    pub count: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerEvent {
    #[serde(rename = "ch")]
    pub ch: String,
    #[serde(rename = "ts")]
    pub ts: u64,
    pub tick: Ticker,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub vol: f64,
    #[serde(with = "string_or_float")]
    pub count: f64,
    #[serde(with = "string_or_float")]
    pub bid: f64,
    #[serde(with = "string_or_float")]
    pub bid_size: f64,
    #[serde(with = "string_or_float")]
    pub ask: f64,
    #[serde(with = "string_or_float")]
    pub ask_size: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub last_size: f64,
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
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
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
