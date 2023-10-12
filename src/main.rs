use std::sync::Arc;
use tokio::task;

use tourney_dono::db;
use tourney_dono::discord;
use tourney_dono::webserver;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let db = db::DB::new().await.unwrap();
    let db = Arc::new(db);

    let dc_db = db.clone();

    let allowed_users = db.get_admins().await.unwrap();
    task::spawn(async move {
        discord::initiate_dc_bot(dc_db, allowed_users)
            .await
            .unwrap();
    });

    task::spawn(async move {
        webserver::initiate_webserver(db).await;
    })
    .await
    .unwrap();
}
