use crate::*;

#[derive(Debug, Clone)]
pub struct Context {
    settings: Settings,
    common_db_pool: Pool,
}