use chrono::{DateTime, Duration, Utc};
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
    new_dono: bool,
    server_url: String,
}

impl TemplateData {
    pub fn new(raw_donations: &[Donation]) -> Self {
        let total = total_amount(raw_donations);
        let top = aggregate_donations(raw_donations);
        let latest = sort_by_date(raw_donations);
        let new_dono = is_celebrateable(raw_donations);

        Self {
            top,
            latest,
            total,
            new_dono,
            server_url: SERVER_URL.clone(),
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
    let mut donations = donations.clone().to_vec();
    donations.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap());
    donations
}

pub fn sort_by_amount(donations: &[Donation]) -> Vec<Donation> {
    let mut donations = donations.clone().to_vec();
    donations.sort_by(|a, b| b.donated_at.partial_cmp(&a.donated_at).unwrap());
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

pub fn is_celebrateable(donations: &[Donation]) -> bool {
    donations.iter().any(|donation| !donation.celebrated)
}

#[allow(unused)]
fn get_dummy_donos() -> Vec<Donation> {
    let mut vec = vec![
        Donation::new("a".to_string(), 1.),
        Donation::new("a".to_string(), 1.),
        Donation::new("a".to_string(), 1.),
        Donation::new("a".to_string(), 1.),
        Donation::new("a".to_string(), 1.),
        Donation::new("b".to_string(), 1.),
        Donation::new("b".to_string(), 1.),
        Donation::new("c".to_string(), 1.),
    ];
    vec.iter_mut().enumerate().for_each(|(index, donation)| {
        let day = Duration::days(index as i64);
        donation.donated_at += day;
    });
    dbg!(vec)
}
