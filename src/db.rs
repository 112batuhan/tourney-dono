use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::Donation;

pub struct DB {
    pub con: PgPool,
}

struct DiscordID {
    discord_id: i64,
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
            "INSERT INTO donations (donor, amount) VALUES ($1, $2) Returning id, donor, amount, donated_at, celebrated",
            donor,
            amount,
        )
        .fetch_one(&self.con)
        .await?;

        Ok(())
    }

    pub async fn delete_donation(&self, donor: i64) -> Result<()> {
        sqlx::query_as!(
            Donation,
            "DELETE FROM donations WHERE id = $1 Returning id, donor, amount, donated_at, celebrated",
            donor
        )
        .fetch_one(&self.con)
        .await?;

        Ok(())
    }

    pub async fn get_donations(&self) -> Result<Vec<Donation>> {
        let donations = sqlx::query_as!(
            Donation,
            "SELECT id, donor, amount, donated_at, celebrated FROM donations"
        )
        .fetch_all(&self.con)
        .await?;

        Ok(donations)
    }
    pub async fn set_all_celebration(&self) -> Result<()> {
        sqlx::query_as!(Donation,"UPDATE donations SET celebrated = TRUE  Returning id, donor, amount, donated_at, celebrated")
        .fetch_one(&self.con)
        .await?;
        Ok(())
    }

    pub async fn get_admins(&self) -> Result<Vec<u64>> {
        let admins = sqlx::query_as!(DiscordID, "SELECT discord_id FROM admins")
            .fetch_all(&self.con)
            .await?;

        let id_vec = admins
            .into_iter()
            .map(|id_struct| id_struct.discord_id as u64)
            .collect();
        Ok(id_vec)
    }
}
