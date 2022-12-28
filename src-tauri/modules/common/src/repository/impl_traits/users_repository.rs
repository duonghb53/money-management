use crate::repository::traits::generic_trait::GenericRepository;
use crate::{repository::traits::users_trait::UsersRepository, Context};
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UsersRdbRepository {}

#[async_trait]
impl<Users, UsersFilterRequest, UserCollection>
    GenericRepository<Users, UsersFilterRequest, UserCollection> for UsersRdbRepository
{
    async fn find_by_id(&self, context: &Context, id: u32) -> Result<Users> {
        !unimplemented!();
    }

    async fn find_all(
        &self,
        filter: Option<UsersFilterRequest>,
        limit: usize,
        offset: usize,
    ) -> Result<UserCollection> {
        !unimplemented!();
    }

    async fn find_count(&self, filter: Option<UsersFilterRequest>) -> Result<usize> {
        !unimplemented!();
    }

    async fn create(&self, entity: Users) -> Result<()> {
        !unimplemented!();
    }

    async fn update(&self, entity: Users) -> Result<()> {
        !unimplemented!();
    }

    async fn delete(&self, entity: Users) -> Result<()> {
        !unimplemented!();
    }
}
