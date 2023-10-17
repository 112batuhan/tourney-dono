use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::broadcast::Receiver;

use crate::db::DB;

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}

pub async fn handle_socket(
    mut socket: WebSocket,
    socket_addr: SocketAddr,
    donation_receiver: Receiver<String>,
    db: Arc<DB>,
) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, socket_addr).is_break() {
                return;
            }
        } else {
            println!("client {socket_addr} abruptly disconnected");
            return;
        }
    }
}
