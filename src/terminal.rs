use crate::{error, wss};
use futures_util::{SinkExt, StreamExt};
use std::io::Write;
use tokio::{io::AsyncReadExt, sync::mpsc};

use termion::{raw::IntoRawMode, screen::IntoAlternateScreen};

pub async fn login(ssh_token: &str) -> Result<(), error::WsshError> {
    let socket = wss::connect(ssh_token).await?;
    let (mut wss_write, mut wss_read) = socket.split();

    // 将终端设置为原始模式
    let mut stdin = tokio::io::stdin();
    let mut stdout = std::io::stdout()
        .into_raw_mode()
        .map_err(|e| error::from_string(format!("设置标准输出为原始模式失败: {}", e.to_string())))?
        .into_alternate_screen()
        .map_err(|e| {
            error::from_string(format!("设置标准输出为备用屏幕失败: {}", e.to_string()))
        })?;

    let (close_tx, mut close_rx) = mpsc::channel(1);
    // 读取wss输出写入到标准输出
    tokio::spawn(async move {
        loop {
            if let Some(message) = wss_read.next().await {
                match message {
                    Ok(message) => {
                        stdout.write_all(&message.into_data()).unwrap();
                        stdout.flush().unwrap();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        // 发送关闭消息
        let _ = close_tx.send(1).await;
    });

    // 初始化窗口大小
    if let Ok(resize_message) = build_resize_message() {
        let _ = wss_write.send(resize_message).await;
    }

    // 读取标准输入写入到wss
    let mut buf = [0; 1024];
    loop {
        tokio::select! {
            _ = close_rx.recv() => {
                break;
            }
            read_res = stdin.read(&mut buf) => {
                match read_res {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }
                        let message = build_input_message(&buf[..n])?;

                        if let Err(_) = wss_write.send(message).await {
                            break;
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn build_resize_message() -> Result<tungstenite::Message, error::WsshError> {
    let (cols, rows) = termion::terminal_size()
        .map_err(|e| error::from_string(format!("终端大小获取失败: {}", e.to_string())))?;
    let raw_message = serde_json::to_string(&wss::Message {
        r#type: wss::MESSAGE_TYPE_RESIZE.to_string(),
        input: "".to_string(),
        rows,
        cols,
    })
    .map_err(|e| error::from_string(format!("resize消息序列化失败: {}", e.to_string())))?;

    Ok(tungstenite::Message::text(raw_message))
}

fn build_input_message(input: &[u8]) -> Result<tungstenite::Message, error::WsshError> {
    let input = String::from_utf8(input.to_vec())
        .map_err(|e| error::from_string(format!("input消息转字符串失败: {}", e.to_string())))?;
    let raw_message = serde_json::to_string(&wss::Message {
        r#type: wss::MESSAGE_TYPE_INPUT.to_string(),
        input,
        rows: 0,
        cols: 0,
    })
    .map_err(|e| error::from_string(format!("input消息序列化失败: {}", e)))?;

    Ok(tungstenite::Message::text(raw_message))
}
