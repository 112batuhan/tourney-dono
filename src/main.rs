use std::sync::Arc;

use tourney_dono::db;
use tourney_dono::discord;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let db = db::DB::new().await.unwrap();
    let db = Arc::new(db);

    let allowed_users = vec![142828565359099905];
    discord::initiate_dc_bot(db.clone(), allowed_users)
        .await
        .unwrap()
        .await
        .unwrap();
}
