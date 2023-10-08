use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod db;
pub mod discord;

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct Donation {
    pub donor: String,
    pub amount: f32,
    pub donated_at: DateTime<Utc>,
}

pub fn get_total_amount(donations: &Vec<Donation>) -> f32 {
    let sum = donations
        .iter()
        .fold(0., |acc, donation| acc + donation.amount);
    sum * 2.
}
