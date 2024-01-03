use axum::extract::{Path, Query};
use axum::{Extension, Json};
use crate::models::menu::{Menu, MenuParam, MenuQueryParam};
use crate::repositories::RepoExt;
use crate::service;
use crate::models::result::{R, ToResult};


pub async fn select_page(Extension(repo): RepoExt, 
    Query(menu_query_param): Query<MenuQueryParam>
) -> R<Vec<Menu>> {
    let users_res = service::menu::select_page(repo.clone(), &menu_query_param).await;
    users_res.to_result()
}

pub async fn select_one(Extension(repo): RepoExt, Path(id): Path<i32>) -> R<Menu> {
    let user_res = service::menu::select_one(repo.clone(), id).await;
    user_res.to_result()
}

pub async fn insert(Extension(repo): RepoExt, Json(menu_param): Json<MenuParam>) -> R<()> {
    let res = service::menu::insert(repo.clone(), menu_param).await;
    res.to_result()
}

pub async fn update(Extension(repo): RepoExt, Json(menu_param): Json<MenuParam>) -> R<()> {
    let res = service::menu::update(repo.clone(), menu_param).await;
    res.to_result()
}

pub async fn delete(Extension(repo): RepoExt, Path(id): Path<i32>) -> R<()> {
    let res = service::menu::delete(repo.clone(), id).await;
    res.to_result()
}
