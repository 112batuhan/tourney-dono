use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

use crate::{db::DB, templates::Templates};

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
}

pub async fn initiate_webserver(db: Arc<DB>, templates: Arc<Templates<'static>>) {
    let state = Arc::new(SharedState { db, templates });
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/celebrated/:id", get(set_celebration))
        .route("/", get(send_page))
        .with_state(state);

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        std::env::var("PORT").unwrap().parse::<u16>().unwrap(),
    ));

    tracing::info!("Starting serving on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start axum server.")
}
