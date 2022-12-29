use crate::collections::user_collection::UserCollection;
use crate::entities::users::{self, Model};
use crate::request::users_request::UsersFilterRequest;
use crate::{repository::traits::users_trait::UsersRepository, Context};
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

#[derive(Clone)]
pub struct UsersRdbRepository {}

#[async_trait]
impl UsersRepository for UsersRdbRepository {
    async fn find_by_id(&self, context: &Context, id: u32) -> Result<Model> {
        unimplemented!();
    }

    async fn find_all(
        &self,
        context: &Context,
        filter: Option<UsersFilterRequest>,
        limit: usize,
        offset: usize,
    ) -> Result<UserCollection> {
        unimplemented!();
    }

    async fn find_count(
        &self,
        context: &Context,
        filter: Option<UsersFilterRequest>,
    ) -> Result<usize> {
        unimplemented!();
    }

    async fn create(&self, context: &Context, role_id: i32) -> Result<()> {
        let model = users::ActiveModel {
            role_id: Set(role_id),
            created_at: Set(Some(chrono::offset::Utc::now().naive_utc())),
            updated_at: Set(Some(chrono::offset::Utc::now().naive_utc())),
            ..Default::default()
        };

        model.insert(&context.connection).await
    }

    async fn update(&self, context: &Context, entity: Model) -> Result<()> {
        unimplemented!();
    }

    async fn delete(&self, context: &Context, entity: Model) -> Result<()> {
        unimplemented!();
    }
}
