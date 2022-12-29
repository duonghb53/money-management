use crate::{
    collections::user_collection::UserCollection, entities::users::Model,
    request::users_request::UsersFilterRequest, shared::Context,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UsersRepository {
    async fn find_by_id(&self, context: &Context, id: u32) -> Result<Model>;

    async fn find_all(
        &self,
        context: &Context,
        filter: Option<UsersFilterRequest>,
        limit: usize,
        offset: usize,
    ) -> Result<UserCollection>;

    async fn find_count(
        &self,
        context: &Context,
        filter: Option<UsersFilterRequest>,
    ) -> Result<usize>;

    async fn create(&self, context: &Context, role_id: i32) -> Result<()>;

    async fn update(&self, context: &Context, entity: Model) -> Result<()>;

    async fn delete(&self, context: &Context, entity: Model) -> Result<()>;
}
