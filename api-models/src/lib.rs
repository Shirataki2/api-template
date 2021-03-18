pub mod guild;
use async_trait::async_trait;

#[async_trait]
pub trait ListModel: Sized + GetModel {
    async fn list(pool: &sqlx::PgPool, key: Self::Pk) -> Result<Vec<Self>, sqlx::Error>;
}

#[async_trait]
pub trait GetModel: Sized {
    type Pk: Send + Sync;
    async fn get(pool: &sqlx::PgPool, key: &Self::Pk) -> Result<Self, sqlx::Error>;
}

#[async_trait]
pub trait CreateModel: Sized + GetModel {
    type CreateSchema: Send + Sync;
    async fn create(pool: &sqlx::PgPool, payload: Self::CreateSchema) -> Result<(), sqlx::Error>;

    async fn checked_create(pool: &sqlx::PgPool, key: &Self::Pk, payload: Self::CreateSchema) -> Result<Self, sqlx::Error> {
        match Self::get(pool, key).await {
            Ok(data) => Ok(data),
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    match Self::create(pool, payload).await {
                        Ok(()) => {
                            Self::get(pool, key).await
                        }
                        Err(e) => Err(e)
                    }
                }
                e => Err(e)
            }
        }
    }
}

#[async_trait]
pub trait UpdateModel: Sized + GetModel + CreateModel {
    type UpdateSchema: Send + Sync;
    async fn update(pool: &sqlx::PgPool, key: &Self::Pk, payload: Self::UpdateSchema) -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait DeleteModel: Sized + GetModel {
    async fn delete(pool: &sqlx::PgPool, key: &Self::Pk) -> Result<u64, sqlx::Error>;
}

#[async_trait]
pub trait CRUDModel: GetModel + CreateModel + UpdateModel + DeleteModel {}
impl<T: GetModel + CreateModel + UpdateModel + DeleteModel> CRUDModel for T {}
