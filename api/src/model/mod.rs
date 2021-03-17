pub mod guild;
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait ListModel: Sized + GetModel {
    async fn list(pool: &sqlx::PgPool, key: Self::Pk) -> Result<Vec<Self>, AppError>;
}

#[async_trait]
pub trait GetModel: Sized {
    type Pk;
    async fn get(pool: &sqlx::PgPool, key: Self::Pk) -> Result<Self, AppError>;
}

#[async_trait]
pub trait CreateModel: Sized {
    type MessageModel;
    async fn create(pool: &sqlx::PgPool, payload: Self::MessageModel) -> Result<(), AppError>;
}

#[async_trait]
pub trait UpdateModel: Sized + CreateModel {
    async fn update(pool: &sqlx::PgPool, payload: Self::MessageModel) -> Result<(), AppError>;
}

#[async_trait]
pub trait DeleteModel: Sized + GetModel {
    async fn delete(pool: &sqlx::PgPool, key: Self::Pk) -> Result<u64, AppError>;
}

#[async_trait]
pub trait CRUDModel: GetModel + CreateModel + UpdateModel + DeleteModel {}
impl<T: GetModel + CreateModel + UpdateModel + DeleteModel> CRUDModel for T {}
