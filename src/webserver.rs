use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};

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

async fn hello(State(state): State<Arc<SharedState<'static>>>) -> Result<Html<String>, AppError> {
    let donations = state.db.get_donations().await?;

    let html_string = state.templates.get_html(donations)?;
    Ok(Html(html_string))
}

pub struct SharedState<'a> {
    db: Arc<DB>,
    templates: Arc<Templates<'a>>,
}

pub async fn initiate_webserver(db: Arc<DB>, templates: Arc<Templates<'static>>) {
    let state = Arc::new(SharedState { db, templates });
    let app = Router::new().route("/", get(hello)).with_state(state);

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
