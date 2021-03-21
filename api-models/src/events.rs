use super::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Events {
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub notifications: Vec<String>,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventCreate {
    pub guild_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub notifications: Vec<String>,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[async_trait]
impl GetModel for Events {
    type Pk = i32;

    async fn get(pool: &sqlx::PgPool, key: &Self::Pk) -> Result<Self, sqlx::Error> {
        let data = sqlx::query_as!(Self, "SELECT * FROM events WHERE id = $1", *key,)
            .fetch_one(pool)
            .await?;
        Ok(data)
    }
}

#[async_trait]
impl CreateModel for Events {
    type 
}