use derive_getters::Getters;
use derive_new::new;

#[derive(Clone, Debug, Getters, new)]
pub struct CreateUserRequest {
    pub role_id: i32,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct UpdateUserRequest {
    id: i32,
    pub role_id: i32,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct DeleteUserRequest {
    id: i32,
}

#[derive(Clone, Debug, Getters, new)]
pub struct UsersFilterRequest {
    search_string: Option<String>,
}
