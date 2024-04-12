#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use log::LevelFilter;
use dioxus_fullstack::prelude::*;
use chrono::{DateTime, Utc};

pub mod types;

#[server(PostServerData)]
pub async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}