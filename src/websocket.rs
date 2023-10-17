use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::select;
use tokio::sync::broadcast::Receiver;
use tokio::sync::oneshot;
use tracing::{error, info, warn};

use crate::db::DB;
use crate::DonationData;

async fn send_donations(
    socket_sender: &mut SplitSink<WebSocket, Message>,
    socket_addr: &SocketAddr,
    db: Arc<DB>,
) -> Result<()> {
    let donations = db
        .get_donations()
        .await
        .context("Database error while fetching data.")?;
    let json_string = DonationData::new(&donations)
        .get_json_string()
        .context("Failed to parse donation data.")?;
    socket_sender
        .send(Message::Text(json_string))
        .await
        .with_context(|| format!("Failed to send donation data to {}", socket_addr))?;
    Ok(())
}

pub async fn handle_socket(
    socket: WebSocket,
    socket_addr: SocketAddr,
    mut donation_receiver: Receiver<()>,
    db: Arc<DB>,
) {
    let (mut socket_sender, mut socket_receiver) = socket.split();
    // Send when a new connection is established
    if let Err(err) = send_donations(&mut socket_sender, &socket_addr, db.clone()).await {
        error!("Error while sending donation: {}", err)
    }

    let (oneshot_sender, mut oneshot_receiver) = oneshot::channel::<()>();
    let moving_socket_addr = socket_addr.clone();

    // send when a new donation is triggered
    tokio::task::spawn(async move {
        loop {
            select! {
                _msg = &mut oneshot_receiver=> {
                    break;
                }

                _new_donation = donation_receiver.recv() => {
                    if let Ok(_) = _new_donation{
                        if let Err(err) = send_donations(&mut socket_sender, &moving_socket_addr, db.clone()).await
                        {
                            error!("Error while sending donation: {}", err);
                        }
                    }
                }
            }
        }
    });

    // listen for closing message
    if let Some(msg) = socket_receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Close(close_message) => {
                    info!(
                        "Closing the connection of {}: {:?}",
                        socket_addr, close_message
                    );
                    oneshot_sender.send(()).ok();
                    return;
                }
                _ => {}
            }
        } else {
            warn!("Connection of {} got abruptly closed", socket_addr);
            return;
        }
    }
}
