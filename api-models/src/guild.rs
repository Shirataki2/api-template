use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx_helper::{Create, Delete, Get, Update};

#[derive(Clone, Debug, Serialize, Deserialize, Builder, Get, Create, Update, Delete)]
pub struct Guild {
    #[get(pk)]
    pub id: i64,
    pub name: String,
    pub icon_url: String,
    pub locale: String,
    pub voice_model: String,
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::PgPool;

    use super::*;

    #[actix_rt::test]
    async fn guild_crud_test() {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let pool = PgPool::builder()
            .max_size(5)
            .build(&database_url)
            .await
            .expect("Failed to connect to database");

        Guild::create(
            &pool,
            330,
            "Test".to_string(),
            "http://example.com".to_string(),
            "en_US".to_string(),
            "a".to_string(),
        )
        .await
        .unwrap();

        let guild = Guild::get(&pool, 330).await.unwrap();

        assert_eq!(guild.name, "Test".to_string());

        Guild::update(
            &pool,
            guild.id,
            "Piyo".to_string(),
            guild.icon_url,
            guild.locale,
            "a".to_string(),
        )
        .await
        .unwrap();

        let guild = Guild::get(&pool, 330).await.unwrap();

        assert_eq!(guild.name, "Piyo".to_string());

        Guild::delete(&pool, 330).await.unwrap();
    }
}
