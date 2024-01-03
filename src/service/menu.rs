use crate::error::Result;
use crate::models::menu::{Menu, MenuParam, MenuQueryParam};
use crate::repositories::{menu::MenuRepo, Repositories};
use std::sync::Arc;


pub async fn select_page<R: Repositories>(repo: Arc<R>,
    menu_query_param: &MenuQueryParam,
) -> Result<Vec<Menu>> {
    let menus = repo.menu().find_all(menu_query_param).await?;
    Ok(menus)
}

pub async fn select_one<R: Repositories>(repo: Arc<R>, menu_id: i32) -> Result<Menu> {
    let menu = repo.menu().find_by_id(menu_id).await?;
    Ok(menu)
}

pub async fn insert<R: Repositories>(repo: Arc<R>, menu_param: MenuParam) -> Result<()> {
    repo.menu().insert(&menu_param).await?;
    Ok(())
}

pub async fn update<R: Repositories>(repo: Arc<R>, menu_param: MenuParam) -> Result<()> {
    repo.menu().update_by_id(&menu_param).await?;
    Ok(())
}

pub async fn delete<R: Repositories>(repo: Arc<R>, menu_id: i32) -> Result<()> {
    repo.menu().delete_by_id(menu_id).await?;
    Ok(())
}
