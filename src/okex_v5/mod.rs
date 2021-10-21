mod client;
pub mod errors;
pub mod util;

pub mod account;
pub mod api;
pub mod config;
pub mod general;
pub mod margin;
pub mod market;
pub mod rest_model;
pub mod userstream;
pub mod websockets;
pub mod ws_model;

extern crate serde;
extern crate serde_qs as qs;
