use crate::db::{mysql};
use crate::repositories::{
    food::{FoodRepo, FoodRepoImpl},
};
use axum::extract::Extension;
use std::sync::Arc;

pub mod food;

pub type RepoExt = Extension<Arc<RepoImpls>>;

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(mysql::db_connect().await);
    RepoImpls::new(
        FoodRepoImpl::new(db_pool.clone()),
    )
}

pub struct RepoImpls {
    pub food: FoodRepoImpl,
}
impl RepoImpls {
    pub fn new(
        food_repo_impl: FoodRepoImpl,
    ) -> Self {
        Self {
            food: food_repo_impl,
        }
    }
}

pub trait Repositories {
    type FoodRepoImpl: FoodRepo;
    fn food(&self) -> &Self::FoodRepoImpl;
}
impl Repositories for RepoImpls {
    type FoodRepoImpl = FoodRepoImpl;
    fn food(&self) -> &Self::FoodRepoImpl {
        &self.food
    }
}
