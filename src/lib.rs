use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod db;
pub mod discord;
pub mod webserver;
pub mod websocket;

#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct Donation {
    pub id: i64,
    pub donor: String,
    pub amount: f32,
    pub donated_at: DateTime<Utc>,
}
impl Donation {
    pub fn new(donor: String, amount: f32) -> Self {
        Self {
            donor,
            amount,
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
pub struct DonationData {
    id_of_celebrated: Option<i64>,
    aggregate_donations: Vec<Donation>,
    individual_donations: Vec<Donation>,
    pricepool: f32,
}

impl DonationData {
    pub fn new(raw_donations: &[Donation], id_of_celebrated: Option<i64>) -> Self {
        let pricepool = total_amount(raw_donations);
        let aggregate_donations = aggregate_donations(raw_donations);
        let individual_donations = sort_by_date(raw_donations);

        Self {
            id_of_celebrated,
            pricepool,
            aggregate_donations,
            individual_donations,
        }
    }

    pub fn get_json_string(&self) -> Result<String> {
        serde_json::to_string(&self).context("Failed to convert donation data to JSON string.")
    }
}

pub fn total_amount(donations: &[Donation]) -> f32 {
    let sum = donations
        .iter()
        .fold(0., |acc, donation| acc + donation.amount);
    sum * 2.
}

pub fn sort_by_date(donations: &[Donation]) -> Vec<Donation> {
    let mut donations = donations.to_vec();
    donations.sort_by(|a, b| b.donated_at.partial_cmp(&a.donated_at).unwrap());
    donations
}

pub fn sort_by_amount(donations: &[Donation]) -> Vec<Donation> {
    let mut donations = donations.to_vec();
    donations.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap());
    donations
}

pub fn aggregate_donations(donations: &[Donation]) -> Vec<Donation> {
    let mut return_vec: Vec<Donation> = vec![];
    donations.iter().for_each(|donation| {
        let added_donation = return_vec
            .iter_mut()
            .find(|return_donation| return_donation.donor == donation.donor);
        if let Some(return_donation) = added_donation {
            return_donation.amount += donation.amount;
        } else {
            return_vec.push(donation.clone())
        }
    });
    sort_by_amount(&return_vec)
}
