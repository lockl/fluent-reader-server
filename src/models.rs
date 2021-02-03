use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

// Database Models

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "fruser")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass: String,
    pub created_on: SystemTime,
    pub native_lang: String,
}

// Request/Response Models

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub native_lang: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: &'static str,
}

// Authentication

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub exp: usize,
    pub user: User,
}
