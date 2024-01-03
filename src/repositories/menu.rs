use crate::error::Result;
use crate::models::menu::{Menu, MenuParam, MenuQueryParam};
use crate::db::mysql::Db;
use axum::async_trait;
use anyhow::Context;


pub struct MenuRepoImpl {
    pool: Db,
}

impl MenuRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

// #[automock]
#[async_trait]
pub trait MenuRepo {
    async fn find_all(&self, menu_query_param: &MenuQueryParam) -> Result<Vec<Menu>>;

    async fn find_by_id(&self, user_id: i32) -> Result<Menu>;

    async fn insert(&self, menu_param: &MenuParam) -> Result<()>;

    async fn update_by_id(&self, menu_param: &MenuParam) -> Result<()>;

    async fn delete_by_id(&self, id: i32) -> Result<()>;

}


#[async_trait]
impl MenuRepo for MenuRepoImpl {
    async fn find_all(&self, menu_query_param: &MenuQueryParam) -> Result<Vec<Menu>> {
        let mut query = sqlx::query_as::<_, Menu>("select * from cook_menu");
        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }

    async fn find_by_id(&self, menu_id: i32) -> Result<Menu> {
        let row = sqlx::query_as::<_, Menu>("select * from cook_menu where id = $1")
            .bind(menu_id)
            .fetch_one(&*self.pool)
            .await
            .context("DB ERROR (find menu by id)")?;
        Ok(row)
    }

    async fn insert(&self, menu_param: &MenuParam) -> Result<()> {
        todo!()
    }

    async fn update_by_id(&self, menu_param: &MenuParam) -> Result<()> {
        todo!()
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        todo!()
    }
}
