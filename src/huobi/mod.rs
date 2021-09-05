mod client;
pub mod errors;
mod util;

pub mod account;
pub mod api;
pub mod config;
pub mod margin;
pub mod market;
pub mod reference;
pub mod rest_model;
pub mod savings;
pub mod userstream;
pub mod websockets;
pub mod ws_model;

extern crate serde;
extern crate serde_qs as qs;
