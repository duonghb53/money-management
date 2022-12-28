use crate::entities::users::Model;
use derive_getters::Getters;
use derive_new::new;
use serde::Serialize;

#[derive(Clone, Debug, Getters, new)]
pub struct UserCollection {
    list: Vec<Model>,
    total_count: usize,
    limit: usize,
    offset: usize,
}
