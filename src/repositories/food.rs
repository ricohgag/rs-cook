use crate::error::Result;
use crate::models::food::{Food, FoodParam, FoodQueryParam};
use crate::db::mysql::Db;
use axum::async_trait;
use anyhow::Context;


pub struct FoodRepoImpl {
    pool: Db,
}

impl FoodRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

// #[automock]
#[async_trait]
pub trait FoodRepo {
    async fn find_all(&self, food_query_param: &FoodQueryParam) -> Result<Vec<Food>>;

    async fn find_by_id(&self, user_id: i32) -> Result<Food>;

    async fn insert(&self, food_param: &FoodParam) -> Result<()>;

    async fn update_by_id(&self, food_param: &FoodParam) -> Result<()>;

    async fn delete_by_id(&self, id: i32) -> Result<()>;

}


#[async_trait]
impl FoodRepo for FoodRepoImpl {
    async fn find_all(&self, food_query_param: &FoodQueryParam) -> Result<Vec<Food>> {
        let mut query = sqlx::query_as::<_, Food>("select * from cook_food");
        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }

    async fn find_by_id(&self, food_id: i32) -> Result<Food> {
        let row = sqlx::query_as::<_, Food>("select * from cook_food where id = $1")
            .bind(food_id)
            .fetch_one(&*self.pool)
            .await
            .context("DB ERROR (find food by id)")?;
        Ok(row)
    }

    async fn insert(&self, food_param: &FoodParam) -> Result<()> {
        todo!()
    }

    async fn update_by_id(&self, food_param: &FoodParam) -> Result<()> {
        todo!()
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        todo!()
    }
}
