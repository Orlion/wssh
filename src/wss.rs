use serde::Serialize;
use tungstenite::http;

use crate::error;

pub async fn connect(
    ssh_token: &str,
) -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    error::WsshError,
> {
    let uri = format!(
        "wss://jumpserver.domain.com/ssh?ssh_token={}",
        urlencoding::encode(ssh_token),
    );
    let (socket, response) = tokio_tungstenite::connect_async(uri)
        .await
        .map_err(|e| error::from_string(format!("websocket连接失败: {}", e.to_string())))?;
    if response.status() != http::StatusCode::SWITCHING_PROTOCOLS {
        Err(error::from_string(format!(
            "websocket握手失败, status: {}",
            response.status()
        )))
    } else {
        Ok(socket)
    }
}

#[derive(Serialize)]
pub struct Message {
    pub r#type: String,
    pub input: String,
    pub rows: u16,
    pub cols: u16,
}

pub const MESSAGE_TYPE_INPUT: &str = "input";
pub const MESSAGE_TYPE_RESIZE: &str = "resize";
