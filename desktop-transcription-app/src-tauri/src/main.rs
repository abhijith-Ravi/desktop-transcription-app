use std::{env, sync::Arc};
use futures_util::{SinkExt, StreamExt};
use tauri::{Emitter, Listener};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    tungstenite::client::IntoClientRequest,
};

use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;

type WsWrite = Arc<Mutex<Option<
    futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
        Message
    >
>>>;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let api_key = env::var("DEEPGRAM_API_KEY")
                .expect("DEEPGRAM_API_KEY not set");

            let app_handle = app.handle().clone();
            let emit_handle = app_handle.clone();

            let ws_write: WsWrite = Arc::new(Mutex::new(None));

            // PTT START
            {
                let ws_write = ws_write.clone();
                let api_key_shared = api_key.clone();
                let emit_handle_shared = emit_handle.clone();

                app_handle.listen("ptt-start", move |_| {
                    println!("🎤 PTT START → connecting Deepgram");

                    let ws_write = ws_write.clone();
                    let api_key = api_key_shared.clone();
                    let emit_handle = emit_handle_shared.clone();

                    tauri::async_runtime::spawn(async move {
                        let url = "wss://api.deepgram.com/v1/listen?model=nova-2&encoding=linear16&sample_rate=16000&channels=1&interim_results=true";

                        let mut req = match url.into_client_request() {
                            Ok(r) => r,
                            Err(e) => {
                                let _ = emit_handle.emit(
                                    "app-error",
                                    format!("Invalid Deepgram URL: {}", e),
                                );
                                return;
                            }
                        };

                        req.headers_mut().insert(
                            "Authorization",
                            format!("Token {}", api_key).parse().unwrap(),
                        );

                        let (ws, _) = match connect_async(req).await {
                            Ok(v) => v,
                            Err(e) => {
                                let _ = emit_handle.emit(
                                    "app-error",
                                    format!("Deepgram connection failed: {}", e),
                                );
                                return;
                            }
                        };

                        let (write, mut read) = ws.split();
                        *ws_write.lock().await = Some(write);

                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    let json = match serde_json::from_str::<serde_json::Value>(&text) {
                                        Ok(v) => v,
                                        Err(_) => continue,
                                    };

                                    // Ignore metadata & keepalive
                                    if json.get("type").and_then(|v| v.as_str()) == Some("Metadata") {
                                        continue;
                                    }

                                    // Deepgram explicit error
                                    if let Some(err) = json.get("error") {
                                        let _ = emit_handle.emit(
                                            "app-error",
                                            format!("Deepgram error: {}", err),
                                        );
                                        continue;
                                    }

                                    // FINAL ONLY
                                    let is_final = json
                                        .get("is_final")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);

                                    if !is_final {
                                        continue;
                                    }

                                    if let Some(t) = json
                                        .get("channel")
                                        .and_then(|c| c.get("alternatives"))
                                        .and_then(|a| a.get(0))
                                        .and_then(|a| a.get("transcript"))
                                        .and_then(|t| t.as_str())
                                    {
                                        if !t.trim().is_empty() {
                                            println!("FINAL: {}", t);
                                            let _ = emit_handle.emit("transcript-final", t);
                                        }
                                    }
                                }

                                Ok(Message::Close(_)) => {
                                    let _ = emit_handle.emit(
                                        "app-error",
                                        "Deepgram disconnected",
                                    );
                                    break;
                                }

                                Err(e) => {
                                    let _ = emit_handle.emit(
                                        "app-error",
                                        format!("WebSocket error: {}", e),
                                    );
                                    break;
                                }

                                _ => {}
                            }
                        }

                        println!("Deepgram reader ended");
                        *ws_write.lock().await = None;
                    });
                });
            }



            // AUDIO
            {
                let ws_write = ws_write.clone();
                app_handle.listen("audio-chunk", move |event| {
                    if let Ok(bytes) = serde_json::from_str::<Vec<u8>>(event.payload()) {
                        let ws_write = ws_write.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Some(write) = ws_write.lock().await.as_mut() {
                                let _ = write.send(Message::Binary(bytes)).await;
                            }
                        });
                    }
                });
            }

            // PTT STOP
            {
                let ws_write = ws_write.clone();
                app_handle.listen("ptt-stop", move |_| {
                    println!("🛑 PTT STOP → closing Deepgram");
                    let ws_write = ws_write.clone();
                    tauri::async_runtime::spawn(async move {
                        *ws_write.lock().await = None;
                    });
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running app");
}
