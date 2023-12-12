use crate::dto::AppResultResponse;
use crate::error::AppResult;
use anyhow::anyhow;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type WsClientSender =
  SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;

pub type WsClientReceiver = SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

pub async fn connect(url: &str) -> AppResult<(WsClientSender, WsClientReceiver)> {
  let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
  Ok(ws_stream.split())
}

pub async fn send_message<T>(sender: &mut WsClientSender, msg: &T) -> AppResult
where
  T: Serialize,
{
  sender
    .send(Message::Text(serde_json::to_string(msg)?))
    .await?;
  Ok(())
}

pub async fn receive_message<T>(receiver: &mut WsClientReceiver) -> AppResult<AppResultResponse<T>>
where
  T: DeserializeOwned,
{
  match receiver.next().await.transpose()? {
    Some(Message::Text(msg)) => Ok(serde_json::from_str(&msg)?),
    None => Err(anyhow!("WebSocket client received empty message.").into()),
    _ => Err(anyhow!("WebSocket client received invalid format message.").into()),
  }
}
