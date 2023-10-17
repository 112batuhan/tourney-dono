use std::net::SocketAddr;

use axum::extract::ws::WebSocket;
use tokio::sync::broadcast::Receiver;

pub async fn handle_socket(
    mut socket: WebSocket,
    socket_addr: SocketAddr,
    donation_receiver: Receiver<String>,
) {
}
