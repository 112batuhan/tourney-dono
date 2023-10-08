use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::Donation;

pub struct DB {
    pub con: PgPool,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
        let con = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;
        Ok(DB { con })
    }

    pub async fn add_donation(&self, donor: &str, amount: &f32) -> Result<()> {
        sqlx::query_as!(
            Donation,
            "INSERT INTO donations (donor, amount) VALUES ($1, $2) Returning donor, amount, donated_at",
            donor,
            amount,
        )
        .fetch_one(&self.con)
        .await?;

        Ok(())
    }

    pub async fn delete_donation(&self, donor: &str) -> Result<()> {
        sqlx::query_as!(
            Donation,
            "DELETE FROM donations WHERE donor = $1 Returning donor, amount, donated_at",
            donor
        )
        .fetch_one(&self.con)
        .await?;

        Ok(())
    }

    pub async fn get_donations(&self) -> Result<Vec<Donation>> {
        let donations =
            sqlx::query_as!(Donation, "SELECT donor, amount, donated_at FROM donations")
                .fetch_all(&self.con)
                .await?;

        Ok(donations)
    }
}
