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

/// Importing the entitiy to store 
/// metadata about files uploaded.
use actix_multipart::form::json::Json;

/// Importing the trait to make
/// multipart file uploads.
use actix_multipart::form::MultipartForm;

/// Importing the structure to store
/// temporary files to make
/// multipart file uploads.
use actix_multipart::form::tempfile::TempFile;

/// A data structure containing information
/// on a Jade User.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeUser {
    pub username: String,
    pub email: String,
    pub pwd: String,
    pub email_token: String,
    pub is_active: bool
}

/// A data structure a file
/// a Jade user has uploaded.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeUserFile {
    pub file_id: String,
    pub username: String,
    pub file_name: String,
    pub data: Vec<u8>
}

/// A structure containing information
/// on a saved Jade mood.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct JadeMood {
    pub username: String,
    pub is_active: bool,
    pub mood: String,
    pub created_at: String
}

/// A structure containing information
/// on a created API token.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct APIToken {
    pub username: String,
    pub token: String,
    pub created_at: String,
    pub is_active: bool,
    pub can_change_pwd: bool,
    pub can_set_mood: bool,
    pub can_delete_user: bool,
    pub can_change_email: bool
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
    pub can_change_pwd: bool,
    pub can_set_mood: bool,
    pub can_delete_user: bool,
    pub can_change_email: bool
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

/// This structure returns
/// a status that tells users
/// whether their email address
/// was verified or not.
#[derive(Serialize)]
pub struct EmailVerificationStatus {
    pub status: bool
}

/// A structure that holds
/// the metadata on an uploaded
/// file.
#[derive(Deserialize, Debug)]
pub struct MetaData {
    pub name: String,
    pub api_token: String,
}

/// A structure to assist with
/// file upload via "actix-multipart".
#[derive(MultipartForm, Debug)]
pub struct FileUploadForm {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub metadata: Json<MetaData>
}

/// A structure containing
/// a pool of database connections
/// to make app data persist.
pub struct AppData {
    pub pool: Pool<Postgres>,
    pub smtp_server: String
}

/// Implementing generic
/// methods for the "AppData"
/// structure.
impl AppData{

    /// Implementing a method
    /// to create a new instance
    /// of the "AppData"
    /// structure.
    pub fn new(pg_pool: &Pool<Postgres>, smtp_server: &String) -> AppData{
        AppData { pool: pg_pool.to_owned(), smtp_server: smtp_server.to_owned() }
    }

}

/// A structure containing
/// the fields required to run the
/// backend.
pub struct ConfigData{
    pub db_url: String,
    pub actix_host: String,
    pub actix_port: String,
    pub smtp_server: String
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
        actix_port: &String,
        smtp_server: &String
    ) -> ConfigData {
        ConfigData {
            db_url: db_url.to_owned(),
            actix_host: actix_host.to_owned(),
            actix_port: actix_port.to_owned(),
            smtp_server: smtp_server.to_owned()
        }
    }
    
}