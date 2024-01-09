
use crate::error::Result;
use crate::models::food::{Food, FoodParam, FoodQueryParam};
use crate::db::mysql::Db;
use axum::async_trait;
use anyhow::Context;
use tracing::info;


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
        // 判断food_query_param中的name是否为空或''，如果不是，则添加where条件
        if let Some(name) = &food_query_param.food_name {
            if !name.is_empty() {
                query = sqlx::query_as::<_, Food>("select * from cook_food where food_name like ?")
                    .bind(String::from("%") + name + "%");
            }
        }

        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }

    async fn find_by_id(&self, food_id: i32) -> Result<Food> {
        let row = sqlx::query_as::<_, Food>("select * from cook_food where id = ?")
            .bind(food_id)
            .fetch_one(&*self.pool)
            .await
            .context("DB ERROR (find food by id)")?;
        Ok(row)
    }

    async fn insert(&self, food_param: &FoodParam) -> Result<()> {
        info!("insert food: {:?}", food_param);

        let now = chrono::Utc::now();
        sqlx::query("INSERT INTO `rs_cook`.`cook_food` (`food_name`, `create_time`, `update_time`) VALUES (?, ?, ?);")
            .bind(&food_param.food_name)
            .bind(now)
            .bind(now)
            .execute(&*self.pool)
            .await
            .context("DB ERROR (insert food)")?;
        Ok(())
    }

    async fn update_by_id(&self, food_param: &FoodParam) -> Result<()> {
        todo!()
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        todo!()
    }
}
