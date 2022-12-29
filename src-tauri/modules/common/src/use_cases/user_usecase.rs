use crate::{repository::{
    impl_traits::users_repository::UsersRdbRepository, traits::users_trait::UsersRepository,
}, Context};
use anyhow::{anyhow, bail, Result};

pub struct UserUseCase<T>
where
    T: UsersRepository + std::clone::Clone,
{
    pub users_repository: T,
}

impl UserUseCase<UsersRdbRepository> {
    pub async fn create_user(&self, context: &Context, role_id: i32) -> Result<()> {
        self.users_repository.create(context, role_id)
    }
}
