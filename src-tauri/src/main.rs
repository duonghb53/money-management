#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;

use common::{
    entities::users::Model,
    shared::{Context, Settings},
};
use common::repository::impl_traits::users_repository::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_user(role_id: i32) -> String {
    let entity = Model {
        id: 1,
        role_id: role_id,
        full_name: None,
        phone_number: None,
        email: None,
        created_at: None,
        updated_at: None,
    };

    UsersRdbRepository.create()

    "Ok".to_string()
}

#[tokio::main]
async fn main() {
    let settings = Settings::instance().unwrap();
    let ctx = Context::connect(*settings).await;
    match ctx {
        Ok(_ctx) => println!("Connected OK"),
        Err(_) => println!("Connect Error"),
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
