use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod db;
pub mod discord;
pub mod templates;
pub mod webserver;

use once_cell::sync::Lazy;

static SERVER_URL: Lazy<String> =
    Lazy::new(|| std::env::var("SERVER_URL").expect("SERVER_URL environment variable is not set."));

#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct Donation {
    pub id: i64,
    pub donor: String,
    pub amount: f32,
    pub donated_at: DateTime<Utc>,
    pub celebrated: bool,
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
pub struct TemplateData {
    top: Vec<Donation>,
    latest: Vec<Donation>,
    total: f32,
    server_url: String,
}

impl TemplateData {
    pub fn new(raw_donations: &[Donation]) -> Self {
        let total = total_amount(raw_donations);
        let top = aggregate_donations(raw_donations);
        let latest = sort_by_date(raw_donations);

        Self {
            top,
            latest,
            total,
            server_url: SERVER_URL.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct CelebrationData {
    donation: Donation,
    total: f32,
    server_url: String,
}

impl CelebrationData {
    pub fn new(raw_donations: &[Donation]) -> Option<Self> {
        let celebrated_donation = get_celebrateable(&raw_donations);
        if let Some(donation) = celebrated_donation {
            let total = total_amount(raw_donations);
            Some(Self {
                total,
                donation,
                server_url: SERVER_URL.clone(),
            })
        } else {
            None
        }
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

pub fn get_celebrateable(donations: &[Donation]) -> Option<Donation> {
    donations
        .iter()
        .find(|donation| !donation.celebrated)
        .cloned()
}
