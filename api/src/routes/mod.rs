mod message;
mod user;

use std::{borrow::Cow, ops::ControlFlow};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    headers,
    routing::{get, post},
    Router, TypedHeader,
};
use futures::{sink::SinkExt, stream::StreamExt};
pub use message::*;
use tokio::net::unix::SocketAddr;
pub use user::*;

use crate::{api::login, AppState};

pub fn all_routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(get_all_users).post(create))
        .route("/:id", get(get_user))
        .route("/login", post(login))
        .route("/me", get(get_current_user))
        .route("/ws", ws_handler)
        .with_state(state)
}

fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}` at {:?} connected.", addr);

    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pint {:?}...", who);
    } else {
        println!("Could not pint {:?}", who);
        return;
    }

    for i in 1..5 {
        if socket
            .send(Message::Text(format!("Hi {i} times")))
            .await
            .is_err()
        {
            println!("Client {:?} is abruptly disconnected", who);
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        let n_msg = 20;

        for i in 0..n_msg {
            if sender
                .send(Message::Text(format!("Sever message {i}...")))
                .await
                .is_err()
            {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        println!("Sending close to {:?}...", who);

        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            println!("Could not send close due to {e}, probably it is okay?")
        }
        n_msg
    });

    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;

            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{a} message sent to {who:?}"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {b} message"),
                Err(b) => println!("Error receiving message {b:?}")
            }
            send_task.abort();
        }
    }

    println!("websocket context {who:?} destroyed");
}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    //match msg {
    //	Message::Text(t) => {
    //		println!(">> {} sent str: {t:?}", who);
    //	}
    //	Message::Binary(d) => {
    //		println!()
    //	}
    //}

    ControlFlow::Continue(())
}
