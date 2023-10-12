use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod db;
pub mod discord;
pub mod templates;
pub mod webserver;

#[derive(Debug, Serialize, Deserialize, FromRow, Default, Clone)]
pub struct Donation {
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

pub fn total_amount(donations: &[Donation]) -> f32 {
    let sum = donations
        .iter()
        .fold(0., |acc, donation| acc + donation.amount);
    sum * 2.
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
    return_vec
}

pub fn sort_by_date(donations: &[Donation]) -> Vec<Donation> {
    let mut donations = donations.clone().to_vec();
    donations.sort_by(|a, b| b.donated_at.partial_cmp(&a.donated_at).unwrap());
    donations
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
