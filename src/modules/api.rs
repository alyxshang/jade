/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

use super::err::JadeErr;
use super::rw::wipe_user;
use actix_web::web::Data;
use super::rw::wipe_mood;
use actix_web::web::Json;
use super::rw::wipe_token;
use super::rw::write_user;
use super::units::AppData;
use super::units::APIToken;
use super::units::JadeMood;
use super::units::JadeUser;
use actix_web::HttpResponse;
use super::rw::get_user_mood;
use super::rw::get_user_moods;
use crate::DeleteTokenPayload;
use super::rw::get_user_tokens;
use super::rw::create_new_mood;
use super::rw::create_new_token;
use super::rw::update_user_email;
use super::units::StatusResponse;
use super::units::TokenOnlyPayload;
use super::units::MoodActionPayload;
use super::rw::update_user_password;
use super::units::UserMoodsResponse;
use super::units::CreateUserPayload;
use super::units::CreateTokenPayload;
use super::units::UsernameOnlyPayload;
use super::units::ChangeEntityPayload;
use super::units::UserAPITokensPayload;

/// This API route attempts to create a new user
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn create_user(
    payload: Json<CreateUserPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let created: JadeUser = match write_user(&payload, &data.pool).await {
        Ok(created) => created,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(created))
}

/// This API route attempts to delete a user
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn delete_user(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let wiped: StatusResponse = match wipe_user(&payload, &data.pool).await {
        Ok(created) => created,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(wiped))
}

/// This API route attempts to create a new API
/// token with the given payload. If this operation
/// fails, an error response is returend.
pub async fn create_token(
    payload: Json<CreateTokenPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let wiped: APIToken = match create_new_token(&payload, &data.pool).await {
        Ok(created) => created,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(wiped))
}

/// This API route attempts to delete an API
/// token with the given payload. If this operation
/// fails, an error response is returend.
pub async fn delete_token(
    payload: Json<DeleteTokenPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let wiped: StatusResponse = match wipe_token(&payload, &data.pool).await {
        Ok(wiped) => wiped,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(wiped))
}

/// This API route attempts to create a new mood
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn set_mood(
    payload: Json<MoodActionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let new_mood: JadeMood = match create_new_mood(&payload, &data.pool).await {
        Ok(new_mood) => new_mood,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(new_mood))
}

/// This API route attempts to delete a mood
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn delete_mood(
    payload: Json<MoodActionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let status: StatusResponse = match wipe_mood(&payload, &data.pool).await {
        Ok(status) => status,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(status))
}

/// This API route attempts to change a user's
/// password with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn change_user_pwd(
    payload: Json<ChangeEntityPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let op_status: StatusResponse = match update_user_password(&payload, &data.pool).await {
        Ok(op_status) => op_status,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(op_status))
}

/// This API route attempts to change a user's
/// email with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn change_user_email(
    payload: Json<ChangeEntityPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let op_status: StatusResponse = match update_user_email(&payload, &data.pool).await {
        Ok(op_status) => op_status,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(op_status))
}

/// This API route attempts to get a user's
/// mood with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn get_mood(
    payload: Json<UsernameOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let mood: JadeMood = match get_user_mood(&payload, &data.pool).await {
        Ok(mood) => mood,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(mood))
}

/// This API route attempts to get a user's
/// moods with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn get_moods(
    payload: Json<UsernameOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let moods: UserMoodsResponse = match get_user_moods(&payload, &data.pool).await {
        Ok(moods) => moods,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(moods))
}

/// This API route attempts to get a user's
/// tokens with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn get_tokens(
    payload: Json<UserAPITokensPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let tokens: Vec<APIToken> = match get_user_tokens(&payload, &data.pool).await {
        Ok(tokens) => tokens,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(tokens))
}