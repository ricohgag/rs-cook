use crate::db::{mysql};
use crate::repositories::{
    food::{FoodRepo, FoodRepoImpl},
};
use axum::extract::Extension;
use std::sync::Arc;
use crate::repositories::menu::{MenuRepo, MenuRepoImpl};

pub mod food;
pub mod menu;

pub type RepoExt = Extension<Arc<RepoImpls>>;

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(mysql::db_connect().await);
    RepoImpls::new(
        FoodRepoImpl::new(db_pool.clone()),
        MenuRepoImpl::new(db_pool.clone())
    )
}

pub struct RepoImpls {
    pub food: FoodRepoImpl,
    pub menu: MenuRepoImpl,
}

impl RepoImpls {
    pub fn new(
        food_repo_impl: FoodRepoImpl,
        menu_repo_impl: MenuRepoImpl,
    ) -> Self {
        Self {
            food: food_repo_impl,
            menu: menu_repo_impl,
        }
    }
}

pub trait Repositories {
    type FoodRepoImpl: FoodRepo;
    type MenuRepoImpl: MenuRepo;

    fn food(&self) -> &Self::FoodRepoImpl;

    fn menu(&self) -> &Self::MenuRepoImpl;
}

impl Repositories for RepoImpls {
    type FoodRepoImpl = FoodRepoImpl;
    type MenuRepoImpl = MenuRepoImpl;


    fn food(&self) -> &Self::FoodRepoImpl {
        &self.food
    }

    fn menu(&self) -> &Self::MenuRepoImpl {
        &self.menu
    }

}
