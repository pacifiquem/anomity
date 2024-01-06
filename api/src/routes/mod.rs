mod message;
mod user;

use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};

use futures::{sink::SinkExt, stream::StreamExt};
pub use message::*;
pub use user::*;

use crate::{api::login, AppState};

pub fn all_routes<S>(state: Arc<AppState>) -> Router<S> {
    Router::new()
        .route("/users", get(get_all_users).post(create))
        .route("/users/:id", get(get_user))
        .route("/users/login", post(login))
        .route("/users/me", get(get_current_user))
        .route("/rooms", get(get_all_rooms).post(create_room))
        .route("/rooms/:id", delete(delete_room))
        //.route("/rooms/messages/:id", get(get_messages_by_room_id))
        .route("/ws/:room", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| web_socket(socket, state))
}

async fn web_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    let mut username = String::new();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            check_username(&state, &mut username, &name).await;

            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();

    let msg = format!("{username} joined.");

    tracing::debug!("{msg}");

    let _ = state.tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    // if one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    };

    let msg = format!("{username} left.");
    tracing::debug!("{msg}");

    let _ = state.tx.send(msg);
    state.user_set.lock().await.remove(&username);
}

async fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().await;

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}
