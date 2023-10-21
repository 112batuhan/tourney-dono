use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{headers, Router, TypedHeader};
use tokio::sync::broadcast::Sender;
use tower_http::services::ServeDir;
use tracing::info;

use crate::db::DB;
use crate::websocket::handle_socket;

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<SharedState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            addr,
            state.donation_sender.subscribe(),
            state.db.clone(),
        )
    })
}

pub struct SharedState {
    db: Arc<DB>,
    donation_sender: Sender<Option<i64>>,
}

pub async fn initiate_webserver(db: Arc<DB>, donation_sender: Sender<Option<i64>>) {
    let state = Arc::new(SharedState {
        db,
        donation_sender,
    });
    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        std::env::var("PORT").unwrap().parse::<u16>().unwrap(),
    ));

    tracing::info!("Starting serving on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start axum server.")
}
