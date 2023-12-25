use axum::extract::ws::WebSocketUpgrade;
use axum::extract::State;

use crate::AppState;

pub fn _new_message(_state: State<AppState>, _ws: WebSocketUpgrade) {}
