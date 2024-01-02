use axum::extract::{Path, Query};
use axum::{Extension, Json};
use crate::models::food::{Food, FoodParam, FoodQueryParam};
use crate::repositories::RepoExt;
use crate::service;
use crate::models::result::{R, ToResult};


pub async fn select_page(Extension(repo): RepoExt, 
    Query(food_query_param): Query<FoodQueryParam>
) -> R<Vec<Food>> {
    let users_res = service::food::select_page(repo.clone(), &food_query_param).await;
    users_res.to_result()
}

pub async fn select_one(Extension(repo): RepoExt, Path(id): Path<i32>) -> R<Food> {
    let user_res = service::food::select_one(repo.clone(), id).await;
    user_res.to_result()
}

pub async fn insert(Extension(repo): RepoExt, Json(food_param): Json<FoodParam>) -> R<()> {
    let res = service::food::insert(repo.clone(), food_param).await;
    res.to_result()
}

pub async fn update(Extension(repo): RepoExt, Json(food_param): Json<FoodParam>) -> R<()> {
    let res = service::food::update(repo.clone(), food_param).await;
    res.to_result()
}

pub async fn delete(Extension(repo): RepoExt, Path(id): Path<i32>) -> R<()> {
    let res = service::food::delete(repo.clone(), id).await;
    res.to_result()
}
