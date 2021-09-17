pub use util::{bool_to_string, bool_to_string_some};

mod client;
pub mod errors;
pub mod util;

pub mod rest_model;
pub mod ws_model;

pub mod account;
pub mod api;
pub mod config;
pub mod general;
pub mod market;
pub mod userstream;
pub mod websockets;
