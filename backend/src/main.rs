#![warn(unused_extern_crates)]

use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task;
use tourney_dono::db::DB;
use tourney_dono::{discord, webserver};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = DB::new().await.unwrap();
    let db = Arc::new(db);

    let (donation_sender, _receiver) = broadcast::channel::<Option<i64>>(1);

    let dc_donation_sender = donation_sender.clone();
    let dc_db = db.clone();
    let allowed_users = db.get_admins().await.unwrap();

    task::spawn(async move {
        discord::initiate_dc_bot(dc_db, allowed_users, dc_donation_sender)
            .await
            .unwrap();
    });

    webserver::initiate_webserver(db, donation_sender).await;
}
