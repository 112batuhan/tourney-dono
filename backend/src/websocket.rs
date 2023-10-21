use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::select;
use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::db::DB;
use crate::DonationData;

async fn send_donations(
    socket_sender: &mut SplitSink<WebSocket, Message>,
    socket_addr: &SocketAddr,
    db: Arc<DB>,
    celebration_id: Option<i64>,
) -> Result<()> {
    let donations = db
        .get_donations()
        .await
        .context("Database error while fetching data.")?;
    let json_string = DonationData::new(&donations, celebration_id)
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
    mut donation_receiver: Receiver<Option<i64>>,
    db: Arc<DB>,
) {
    let (mut socket_sender, mut socket_receiver) = socket.split();
    // Send data when a new connection is established
    if let Err(err) = send_donations(&mut socket_sender, &socket_addr, db.clone(), None).await {
        error!("Error while sending donation: {}", err)
    }

    let (mpsc_sender, mut mpsc_receiver) = mpsc::channel::<()>(3);
    let moving_socket_addr = socket_addr;

    // send data when a new donation is triggered and respond to ping.
    tokio::task::spawn(async move {
        loop {
            select! {

                client_message = mpsc_receiver.recv() =>{
                    if let Some(_) = client_message{
                        if let Err(err) = socket_sender.send(Message::Text("pong".to_string())).await{
                            error!("Error while sending Pong: {}", err);
                            break;
                        }
                    } else {
                        break;
                    }
                }

                new_donation = donation_receiver.recv() => {
                    if let Ok(celebration_id) = new_donation{
                        if let Err(err) = send_donations(&mut socket_sender, &moving_socket_addr, db.clone(), celebration_id).await
                        {
                            error!("Error while sending donation: {}", err);
                        }
                    }
                }
            }
        }
    });

    loop {
        if let Some(msg) = socket_receiver.next().await {
            if let Ok(msg) = msg {
                debug!("incoming message {:?} from {}", msg, socket_addr);
                if let Message::Close(close_message) = msg {
                    info!(
                        "Closing the connection of {}: {:?}",
                        socket_addr, close_message
                    );
                    return;
                } else if let Message::Text(text_content) = msg {
                    if text_content == "ping" {
                        mpsc_sender.send(()).await.ok();
                    }
                }
            } else {
                warn!("Connection of {} got abruptly closed", socket_addr);
                return;
            }
        }
    }
}
