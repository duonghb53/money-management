
use crate::shared::Context;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait GenericRepository<T, U, V> {
    async fn find_by_id(&self, context: &Context, id: u32) -> Result<T>;

    async fn find_all(
        &self,
        filter: Option<U>,
        limit: usize,
        offset: usize,
    ) -> Result<V>;

    async fn find_count(
        &self,
        filter: Option<U>,
    ) -> Result<usize>;

    async fn create(
        &self,
        entity: T,
    ) -> Result<()>;

    async fn update(
        &self,
        entity: T,
    ) -> Result<()>;

    async fn delete(
        &self,
        entity: T,
    ) -> Result<()>;
}