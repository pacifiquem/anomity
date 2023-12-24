use axum::extract::ws::WebSocketUpgrade;
use axum::extract::State;

use crate::AppState;

pub fn new_message(state: State<AppState>, ws: WebSocketUpgrade) {}
