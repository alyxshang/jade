/*
Jade Backend by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the 
/// "FromRow" trait
/// to derive it.
use sqlx::FromRow;

/// Importing the 
/// "Serialize" trait
/// to derive it.
use serde::Serialize;

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeUser {
    pub username: String,
    pub password: String,
    pub moods: Vec<JadeMood>,
    pub api_tokens: Vec<APIToken>
}

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeMood {
    pub mood: String,
    pub created_at: String
}

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct APIToken {
    pub token: String,
    pub created_at: String,
    pub is_active: bool,
    pub can_change_pwd: bool,
    pub can_set_mood: bool,
    pub can_delete_user: bool,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct TokenOnlyPayload {
    pub api_token: String,
}

#[derive(Deserialize)]
pub struct ChangeEntityPayload {
    pub old_entity: String,
    pub new_entity: String,
    pub api_token: String,
}

#[derive(Deserialize)]
pub struct CreateTokenPayload {
    pub username: String,
    pub password: String,
    pub is_active: bool,
    pub can_change_pwd: bool,
    pub can_set_mood: bool,
    pub can_delete_user: bool
}

#[derive(Deserialize)]
pub struct DeleteTokenPayload {
    pub username: String,
    pub password: String,
    pub api_token: String,
}

#[derive(Deserialize)]
pub struct MoodActionPayload {
    pub api_token: String,
    pub mood: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: usize
}

/// A structure containing
/// a pool of database connections
/// to make app data persist.
pub struct AppData {
    pub pool: Pool<Postgres>
}

/// Implementing generic
/// methods for the "AppData"
/// structure.
impl AppData{

    /// Implementing a method
    /// to create a new instance
    /// of the "AppData"
    /// structure.
    pub fn new(pg_pool: &Pool<Postgres>) -> AppData{
        AppData { pool: pg_pool.to_owned() }
    }

}

/// A structure containing
/// the fields required to run the
/// backend.
pub struct ConfigData{
    pub db_url: String,
    pub actix_host: String,
    pub actix_port: String
}

/// Implementing generic
/// methods for the "ConfigData"
/// structure.
impl ConfigData{

    /// Implementing a method
    /// to create a new instance
    /// of the "ConfigData"
    /// structure.
    pub fn new(
        db_url: &String,
        actix_host: &String,
        actix_port: &String
    ) -> ConfigData {
        ConfigData {
            db_url: db_url.to_owned(),
            actix_host: actix_host.to_owned(),
            actix_port: actix_port.to_owned()
        }
    }
    
}