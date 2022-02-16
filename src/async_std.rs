use crate::errors::HassResult;

use async_tungstenite::{stream::Stream, WebSocketStream};
use url::Url;

#[cfg(feature = "async-std-runtime")]
pub(crate) type WebSocket = WebSocketStream<
    Stream<async_std::net::TcpStream, async_tls::client::TlsStream<async_std::net::TcpStream>>,
>;

#[cfg(feature = "async-std-runtime")]
pub use async_std::task;

#[cfg(feature = "async-std-runtime")]
pub(crate) async fn connect_async(url: Url) -> HassResult<WebSocket> {
    let (client, _) = async_tungstenite::async_std::connect_async(url).await?;
    Ok(client)
}
