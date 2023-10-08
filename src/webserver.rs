use axum::{extract::State, http::StatusCode, response::Html, routing::get, Router};
use minijinja::render;
use std::{net::SocketAddr, sync::Arc};

use crate::db::DB;

const TEMPLATE: &'static str = r#"
    <!doctype html>

    <html lang="en">
    <head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="30">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <title>A Basic HTML5 Template</title>
    <meta name="description" content="A simple HTML5 Template for new projects.">
    <meta name="author" content="Woile">
    </head>

    <body>
        
        {% for donation in donations %}
        <li>{{ donation.donor }} {{ donation.amount }}₺</li>
        {% endfor %}
            
    </body>
    </html>
    "#;

async fn hello(State(state): State<Arc<SharedState>>) -> Result<Html<String>, StatusCode> {
    let donations = state
        .db
        .get_donations()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let r = render!(TEMPLATE, donations);
    Ok(Html(r))
}

pub struct SharedState {
    db: Arc<DB>,
}

pub async fn initiate_webserver(db: Arc<DB>) {
    let state = Arc::new(SharedState { db });
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
