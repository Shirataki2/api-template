use sqlx_helper::{Get, Create, Update, Delete};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Get, Create, Update, Delete)]
pub struct Events {
    #[get(pk)]
    #[create(ignore)]
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub notifications: String,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
