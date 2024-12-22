/*
Jade by Alyx Shang.
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

/// A data structure containing information
/// on a Jade User.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeUser {
    pub email: String,
    pub username: String,
    pub pwd: String,
}

/// A structure containing information
/// on a saved Jade mood.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeMood {
    pub mood: String,
    pub is_active: bool,
    pub username: String,
    pub created_at: String
}

/// A structure containing information
/// on a created API token.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct APIToken {
    pub token: String,
    pub created_at: String,
    pub is_active: bool,
    pub username: String,
    pub can_change_pwd: bool,
    pub can_set_mood: bool,
    pub can_delete_user: bool,
}

/// A structure containing
/// information to submit
/// a payload for creating
/// users.
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub email: String,
    pub username: String,
    pub password: String
}

/// A structure containing
/// information to submit
/// a payload for operations
/// only requiring an API token.
#[derive(Deserialize)]
pub struct TokenOnlyPayload {
    pub api_token: String,
}

/// A structure containing
/// information to submit
/// a payload for changing
/// account information.
#[derive(Deserialize)]
pub struct ChangeEntityPayload {
    pub new_entity: String,
    pub api_token: String,
}

/// A structure containing
/// information to submit
/// a payload for creating
/// an API token.
#[derive(Deserialize)]
pub struct CreateTokenPayload {
    pub username: String,
    pub password: String,
    pub is_active: bool,
    pub can_change_pwd: bool,
    pub can_set_mood: bool,
    pub can_delete_user: bool
}

/// A structure containing
/// information to submit
/// a payload for deleting
/// an API token.
#[derive(Deserialize)]
pub struct DeleteTokenPayload {
    pub username: String,
    pub password: String,
    pub api_token: String,
}

/// A structure containing
/// information to submit
/// a payload for creating
/// or deleting a new Jade
/// mood.
#[derive(Deserialize)]
pub struct MoodActionPayload {
    pub api_token: String,
    pub mood: String,
}

/// A structure containing
/// information for confirming
/// whether data-less operations
/// are successful.
#[derive(Serialize)]
pub struct StatusResponse {
    pub status: usize
}

/// A structure containing
/// information to submit
/// a payload for operations
/// that only require a username
/// as a unique identifier of a user.
#[derive(Deserialize)]
pub struct UsernameOnlyPayload{
    pub username: String
}

/// This structure returns
/// all moods a user has.
/// These include both active
/// and inactive moods.
#[derive(Serialize)]
pub struct UserMoodsResponse {
    pub active_mood: JadeMood,
    pub inactive_moods: Vec<JadeMood>
}

/// A structure containing
/// a payload to let the 
/// user retrieve their
/// active tokens.
#[derive(Deserialize)]
pub struct UserAPITokensPayload {
    pub username: String,
    pub password: String
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