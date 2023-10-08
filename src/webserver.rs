use anyhow::Result;
use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::task::JoinHandle;

use crate::db::DB;

async fn hello() {}

pub struct State {
    db: Arc<DB>,
}

async fn initiate_webserver(db: Arc<DB>) -> Result<JoinHandle<()>> {
    let state = Arc::new(State { db });
    let app = Router::new().route("/", get(hello)).with_state(state);

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        std::env::var("PORT").unwrap().parse::<u16>().unwrap(),
    ));

    tracing::info!("Starting serving on: {}", addr);

    let axum_handle = tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .expect("Failed to start server")
    });
    Ok(axum_handle)
}
