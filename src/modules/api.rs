/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

use crate::APIToken;
use crate::JadeMood;
use crate::wipe_token;
use super::err::JadeErr;
use super::rw::wipe_user;
use actix_web::web::Data;
use super::rw::wipe_mood;
use actix_web::web::Json;
use super::rw::write_user;
use super::units::AppData;
use actix_web::HttpResponse;
use super::units::JadeUser;
use super::rw::get_user_moods;
use crate::DeleteTokenPayload;
use super::rw::get_user_tokens;
use super::rw::create_new_mood;
use super::rw::create_new_token;
use super::rw::get_mood_from_db;
use super::units::StatusResponse;
use super::units::TokenOnlyPayload;
use super::units::MoodActionPayload;
use super::units::CreateUserPayload;
use super::units::CreateTokenPayload;
use super::units::UsernameOnlyPayload;

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

pub async fn get_mood(
    payload: Json<UsernameOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let mood: JadeMood = match get_mood_from_db(&payload, &data.pool).await {
        Ok(mood) => mood,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(mood))
}

pub async fn get_moods(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let moods: Vec<JadeMood> = match get_user_moods(&payload.api_token, &data.pool).await {
        Ok(moods) => moods,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(moods))
}

pub async fn get_tokens(
    payload: Json<CreateUserPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let tokens: Vec<APIToken> = match get_user_tokens(&payload.password, &payload.username, &data.pool).await {
        Ok(tokens) => tokens,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(tokens))
}