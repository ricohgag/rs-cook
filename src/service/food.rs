use crate::error::Result;
use crate::models::food::{Food, FoodParam, FoodQueryParam};
use crate::repositories::{food::UserRepo, Repositories};
use std::sync::Arc;


pub async fn select_page<R: Repositories>(repo: Arc<R>,
    food_query_param: &FoodQueryParam,
) -> Result<Vec<User>> {
    let users = repo.food().find_all(food_query_param).await?;
    Ok(users)
}

pub async fn select_one<R: Repositories>(repo: Arc<R>, food_id: i32) -> Result<Food> {
    let food = repo.food().find_by_id(food_id).await?;
    Ok(food)
}

pub async fn insert<R: Repositories>(repo: Arc<R>, food_param: FoodParam) -> Result<()> {
    repo.food().insert(food_param).await?;
    Ok(())
}

pub async fn update<R: Repositories>(repo: Arc<R>, food_param: FoodParam) -> Result<()> {
    repo.food().update(food_param).await?;
    Ok(())
}

pub async fn delete<R: Repositories>(repo: Arc<R>, food_id: i32) -> Result<()> {
    repo.food().delete(food_id).await?;
    Ok(())
}
