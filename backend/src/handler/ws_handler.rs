use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, Extension, Query},
    response::IntoResponse,
};

use crate::{helper::jwt::decode_jwt, model::ws_model::AppState};
use tokio::sync::mpsc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WsAuthQuery {
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsAuthQuery>,
    Extension(state): Extension<AppState>,
) -> impl IntoResponse {
    let user = decode_jwt(&query.token);
    let username = match user {
        Ok(data) => data.claims.username,
        Err(_) => return ws.on_upgrade(|_| async {}), // jika token tidak valid, tidak perlu upgrade
    };
    ws.on_upgrade(move |socket| handle_socket(socket, username, state))
}
async fn handle_socket(
    mut socket: WebSocket,
    username: String,
    state: AppState,
) {
    use axum::extract::ws::Message;

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // simpan koneksi ke hashmap
    state.user_sockets.lock().unwrap().insert(username.clone(), tx);

    let user_sockets_clone = state.user_sockets.clone();
    let username_clone = username.clone();

    tracing::info!("User {} connected", username);

    // tugas kirim data ke client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if socket.send(msg).await.is_err() {
                break;
            }
        }
    });

    let _ = tokio::select! {
        _ = send_task => (),
    };

    // baru di sini socket dihapus
    tracing::info!("User {} disconnected", username_clone);
    user_sockets_clone.lock().unwrap().remove(&username_clone);
}