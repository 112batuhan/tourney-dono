use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{headers, Router, TypedHeader};
use tokio::sync::broadcast::Sender;
use tower_http::services::ServeDir;

use crate::db::DB;
use crate::templates::Templates;
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
    State(state): State<Arc<SharedState<'static>>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state.donation_sender.subscribe()))
}

async fn send_page(
    State(state): State<Arc<SharedState<'static>>>,
) -> Result<Html<String>, AppError> {
    let donations = state.db.get_donations().await?;
    let html_string = state.templates.get_html(donations)?;
    Ok(Html(html_string))
}

async fn set_celebration(
    Path(id): Path<i64>,
    State(state): State<Arc<SharedState<'static>>>,
) -> Result<StatusCode, AppError> {
    state.db.set_celebration(id, true).await?;
    Ok(StatusCode::OK)
}

pub struct SharedState<'a> {
    db: Arc<DB>,
    templates: Arc<Templates<'a>>,
    donation_sender: Sender<String>,
}

pub async fn initiate_webserver(
    db: Arc<DB>,
    templates: Arc<Templates<'static>>,
    donation_sender: Sender<String>,
) {
    let state = Arc::new(SharedState {
        db,
        templates,
        donation_sender,
    });
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/ws", get(ws_handler))
        .route("/celebrated/:id", get(set_celebration))
        .route("/", get(send_page))
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
