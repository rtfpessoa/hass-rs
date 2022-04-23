//! # Async HomeAssistant Websocket Library
//!
//! hass-ws is a HomeAssistant Websocket API client library.
//!
//! It is based on the [official API specifications](https://developers.home-assistant.io/docs/api/websocket).
//!
//! # Configuring async runtime
//! hass_ws supports `async-std` and `tokio` runtimes, by default it uses `async-std`,
//! to use `tokio` change the feature flags in `Cargo.toml`
//!
//! ```toml
//! [dependencies.hass_ws]
//! version = "0.1.0"
//! default-features = false
//! features = ["tokio-runtime"]
//! ```
//!
//!
//! # Example usage
//! It is fetching the Home Assistant Config
//!
//! ```no_run
//! use hass_ws::client;
//! use lazy_static::lazy_static;
//! use std::env::var;
//!
//! lazy_static! {
//!     static ref TOKEN: String =
//!         var("HASS_TOKEN").expect("need HASS_TOKEN environment variable");
//! }
//!
//! #[async_std::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//! // Create the websocket client and connect to gateway
//!     let mut client = client::connect("localhost", 8123).await?;
//!
//! // Authenticate the session
//!     client.auth_with_longlivedtoken(&*TOKEN).await?;
//!     println!("WebSocket connection and authethication works");
//!
//! // Fetch the Home Assistant Config
//!     println!("Get Hass Config");
//!     match client.get_config().await {
//!         Ok(v) => println!("{:?}", v),
//!         Err(err) => println!("Oh no, an error: {}", err),
//!     }
//!     Ok(())
//! }
//! ```

pub mod errors;
pub use errors::{HassError, HassResult};

pub mod types;
pub use types::*;

pub mod client;
pub use client::{connect, HassClient};

#[cfg(feature = "async-std-runtime")]
mod async_std;
#[cfg(feature = "async-std-runtime")]
use crate::async_std::{connect_async, task, WebSocket};

#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
mod tokio;
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use crate::tokio::{connect_async, task, WebSocket};

mod wsconn;
use wsconn::WsConn;
