use super::*;
use async_trait::async_trait;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct Guild {
    pub id: i64,
    pub name: String,
    pub icon_url: String,
    pub locale: String,
    pub voice_model: String,
}

impl Guild {
    pub async fn set_locale(
        pool: &sqlx::PgPool,
        id: &i64,
        locale: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE guild SET locale = $2 WHERE id = $1", *id, locale)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_voice(
        pool: &sqlx::PgPool,
        id: &i64,
        voice: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE guild SET voice_model = $2 WHERE id = $1", *id, voice)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct GuildUpdate {
    name: String,
    icon_url: String,
    locale: String,
    voice_model: String,
}

#[async_trait]
impl GetModel for Guild {
    type Pk = i64;
    async fn get(pool: &sqlx::PgPool, key: &Self::Pk) -> Result<Self, sqlx::Error> {
        let data = sqlx::query_as!(Self, "SELECT * FROM guild WHERE id = $1", *key)
            .fetch_one(pool)
            .await?;
        Ok(data)
    }
}

#[async_trait]
impl CreateModel for Guild {
    type CreateSchema = Self;
    async fn create(pool: &sqlx::PgPool, payload: Self::CreateSchema) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO guild (id, name, icon_url, locale, voice_model) VALUES ($1, $2, $3, $4, $5)",
            payload.id,
            payload.name,
            payload.icon_url,
            payload.locale,
            payload.voice_model,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl UpdateModel for Guild {
    type UpdateSchema = GuildUpdate;
    async fn update(
        pool: &sqlx::PgPool,
        id: &i64,
        payload: Self::UpdateSchema,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE guild SET name = $2, icon_url = $3, locale = $4, voice_model = $5WHERE id = $1",
            *id,
            payload.name,
            payload.icon_url,
            payload.locale,
            payload.voice_model,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl DeleteModel for Guild {
    async fn delete(pool: &sqlx::PgPool, key: &Self::Pk) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM guild WHERE id = $1", *key)
            .execute(pool)
            .await?;
        Ok(result)
    }
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
            Guild {
                id: 330,
                name: "Test".to_string(),
                icon_url: "http://example.com".to_string(),
                locale: "en_US".to_string(),
                voice_model: "a".to_string(),
            },
        )
        .await
        .unwrap();

        let guild = Guild::get(&pool, &330).await.unwrap();

        assert_eq!(guild.name, "Test".to_string());

        Guild::update(
            &pool,
            &guild.id,
            GuildUpdate {
                name: "Piyo".to_string(),
                icon_url: guild.icon_url,
                locale: guild.locale,
                voice_model: "a".to_string(),
            },
        )
        .await
        .unwrap();

        let guild = Guild::get(&pool, &330).await.unwrap();

        assert_eq!(guild.name, "Piyo".to_string());

        let result = Guild::delete(&pool, &330).await.unwrap();
        assert_eq!(result, 1);

        let result = Guild::delete(&pool, &330).await.unwrap();
        assert_eq!(result, 0);
    }
}
