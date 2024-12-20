/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

use sqlx::Pool;
use sqlx::query;
use bcrypt::hash;
use bcrypt::verify;
use super::err::JadeErr;
use bcrypt::DEFAULT_COST;
use super::time::get_time;
use super::units::JadeUser;
use super::units::JadeMood;
use super::units::APIToken;
use sqlx::postgres::Postgres;
use crate::ChangeEntityPayload;
use crate::MoodActionPayload;
use super::utils::hash_string;
use super::units::StatusResponse;
use super::units::TokenOnlyPayload;
use super::units::CreateUserPayload;
use super::units::CreateTokenPayload;
use super::units::DeleteTokenPayload;
use super::units::UsernameOnlyPayload;

pub async fn write_user(
    payload: &CreateUserPayload, 
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr> {
    let hashed = match hash(payload.password, DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let now: String = format!("{}", &get_time());
    let token: String = format!("{}:{}", &hash_string(&now), &payload.username);
    let moods: Vec<JadeMood> = Vec::new();
    let mut tokens: Vec<APIToken> = Vec::new();
    let first_token: APIToken = APIToken{
        token: token,
        created_at: now,
        is_active: true,
        can_change_pwd: true,
        can_set_mood: true,
        can_delete_user: true
    };
    tokens.push(first_token);
    let new_user: JadeUser = JadeUser{
        username: payload.username,
        email: payload.email,
        pwd: hashed,
        moods: moods,
        api_tokens: tokens
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO users (username, pwd, email, moods, api_tokens) VALUES ($1, $2, $3, $4, $5)",
        new_user.username,
        new_user.password,
        new_user.email,
        new_user.moods,
        new_user.api_tokens
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let created: JadeUser = match get_user_by_api_token(&first_token.token, pool).await {
        Ok(created) => created,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(created)

}

pub async fn get_user_by_api_token(
    sup_token: &String, 
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr>{
    let objects = match sqlx::query_as!(JadeUser,"SELECT * FROM users").fetch_all(pool).await {
        Ok(objects) => objects,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut res_vec: Vec<JadeUser> = Vec::new();
    for user in objects {
        let tokens: Vec<APIToken> = user.api_tokens;
        for token in tokens {
            if &token.token == sup_token && token.is_active {
                res_vec.push(user)
            }
            else {}
        }
    }
    if res_vec.len() == 1 {
        Ok(res_vec[0].clone())
    }
    else {
        let e: String = "A user with this token could not be retrieved.".to_string();
        return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

pub async fn get_user_by_handle(
    username: &String, 
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr>{
    let objects = match sqlx::query_as!(JadeUser,"SELECT * FROM users").fetch_all(pool).await {
        Ok(objects) => objects,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut res_vec: Vec<JadeUser> = Vec::new();
    for user in objects {
        
            if &user.username == username {
                res_vec.push(user)
            }
            else {}
    }
    if res_vec.len() == 1 {
        Ok(res_vec[0].clone())
    }
    else {
        let e: String = "A user with this token could not be retrieved.".to_string();
        return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

pub async fn wipe_user(
    payload: &TokenOnlyPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr>{
    let mut result: usize = 1;
    let user: JadeUser = match get_user_by_api_token(&payload.api_token, pool).await {
        Ok(created) => created,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let username: String = user.username;
    let _wipe_op: () = match sqlx::query!("DELETE FROM users WHERE username=$1", username)
        .execute(pool)
        .await
    {
        Ok(_feedback) => result = 0,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: result };
    Ok(status)
} 

pub async fn verify_token(api_token: &String, pool: &Pool<Postgres>) -> Result<bool, JadeErr> {
    let mut result: bool = false;
    let user: JadeUser = match get_user_by_api_token(&api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    for token in user.api_tokens {
        if &token.token == api_token && token.is_active {
            result = true;
        }
        else {}
    }
    Ok(result)
}

pub async fn create_new_token(
    payload: &CreateTokenPayload, 
    pool: &Pool<Postgres>
) -> Result<APIToken, JadeErr> {
    let mut result: usize = 1;
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let now: String = format!("{}", &get_time());
    let token: String = format!("{}:{}", &hash_string(&now), &payload.username);
    let api_token: APIToken = APIToken{
        token: token,
        created_at: now,
        is_active: true,
        can_change_pwd: payload.can_change_pwd,
        can_delete_user: payload.can_delete_user,
        can_set_mood: payload.can_set_mood
    };
    let password_verif: bool = match verify(&payload.password, &user.pwd){
        Ok(user) => user,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut new_tokens: Vec<APIToken> = user.api_tokens;
    new_tokens.push(api_token);
    if password_verif{
        let update_op: () = match query!(
            "UPDATE users SET api_tokens = $1 WHERE username = $2",
            new_tokens,
            username,
        ).execute(pool).await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(api_token)
    }
    else {
        let e: String = "Wrong password.".to_string();
        return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()));
    }    
}

pub async fn wipe_token(
    payload: &DeleteTokenPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let mut result: usize = 1;
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let password_verif: bool = match verify(&payload.password, &user.pwd){
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut new_tokens: Vec<APIToken> = Vec::new();
    for token in user.api_tokens{
        if token.token == payload.api_token {}
        else {
            new_tokens.push(token);
        }
    }
    if password_verif{
        let update_op: () = match query!(
            "UPDATE users SET api_tokens = $1 WHERE username = $2",
            new_tokens,
            username,
        ).execute(pool).await
        {
            Ok(_feedback) => result = 0,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: result };
        Ok(status)
    }
    else {
        let e: String = "Wrong password.".to_string();
        return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()));
    }

}

pub async fn create_new_mood(
    payload: &MoodActionPayload, 
    pool: &Pool<Postgres>
) -> Result<JadeMood, JadeErr> {
    let user: JadeUser = match get_user_by_api_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let new_mood: JadeMood = JadeMood{
        id: hash_string(&format!("{}:{}", get_time(), user.username)),
        mood: payload.mood,
        created_at: get_time()
    };
    let token_verif: bool = match verify_token(&payload.api_token, pool).await {
        Ok(token_verif) => token_verif,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut new_moods: Vec<JadeMood> = user.moods;
    new_moods.push(new_mood);
    if token_verif{
        let update_op: () = match query!(
            "UPDATE users SET moods = $1 WHERE username = $2",
            new_moods,
            user.username,
        ).execute(pool).await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(new_mood)
    }
    else {
        let e: String = "Wrong token or the token is not active.".to_string();
        return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()));
    }    
}

pub async fn wipe_mood(
    payload: &MoodActionPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let mut result: usize = 1;
    let user: JadeUser = match get_user_by_api_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut new_moods: Vec<JadeMood> = Vec::new();
    for mood in user.moods{
        if mood.mood == payload.mood {}
        else {
            new_moods.push(mood);
        }
    }
    let token_verif: bool = match verify_token(&payload.api_token, pool).await {
        Ok(token_verif) => token_verif,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token_verif{
        let update_op: () = match query!(
            "UPDATE users SET moods = $1 WHERE username = $2",
            new_moods,
            user.username,
        ).execute(pool).await
        {
            Ok(_feedback) => result = 0,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: result };
        Ok(status)
    }
    else {
        let e: String = "Wrong token or the token is not active.".to_string();
        return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()));
    }

}

pub async fn get_mood_from_db(payload: &UsernameOnlyPayload, pool: &Pool<Postgres>) -> Result<JadeMood, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(user.moods[0].clone())
} 

pub async fn get_user_moods(api_token: &String, pool: &Pool<Postgres>) -> Result<Vec<JadeMood>, JadeErr>{
    let user: JadeUser = match get_user_by_api_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<JadeMood>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let token_verif: bool = match verify_token(api_token, pool).await {
        Ok(token_verif) => token_verif,
        Err(e) => return Err::<Vec<JadeMood>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token_verif {
        Ok(user.moods)
    }
    else {
        let e: String = "Wrong token or no moods have been set yet.".to_string();
        return Err::<Vec<JadeMood>, JadeErr>(JadeErr::new(&e.to_string()));
    }
}

pub async fn get_user_tokens(password: &String, username: &String, pool: &Pool<Postgres>) -> Result<Vec<APIToken>, JadeErr>{
    let user: JadeUser = match get_user_by_handle(username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let pwd_verif: bool = match verify(password, &user.pwd) {
        Ok(pwd_verif) => pwd_verif,
        Err(e) => return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if pwd_verif {
        Ok(user.api_tokens)
    }
    else {
        let e: String = "Wrong token or no moods have been set yet.".to_string();
        return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()));
    }
}

pub async fn update_user_pwd(
    payload: &ChangeEntityPayload, 
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let mut result: usize = 1;
    let user: JadeUser = match get_user_by_api_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let token_verif: bool = match verify_token(&payload.api_token, pool).await {
        Ok(token_verif) => token_verif,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token_verif{
        let _update_op: () = match query!(
            "UPDATE users SET pwd = $1 WHERE username = $2",
            payload.new_entity,
            user.username,
        ).execute(pool).await
        {
            Ok(_feedback) => result = 0,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status_resp: StatusResponse = StatusResponse{ status: result };
        Ok(status_resp)
    }
    else {
        let e: String = "Wrong token or the token is not active.".to_string();
        return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()));
    }    
}

pub async fn update_user_email(
    payload: &ChangeEntityPayload, 
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let mut result: usize = 1;
    let user: JadeUser = match get_user_by_api_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let token_verif: bool = match verify_token(&payload.api_token, pool).await {
        Ok(token_verif) => token_verif,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token_verif{
        let _update_op: () = match query!(
            "UPDATE users SET email = $1 WHERE username = $2",
            payload.new_entity,
            user.username,
        ).execute(pool).await
        {
            Ok(_feedback) => result = 0,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status_resp: StatusResponse = StatusResponse{ status: result };
        Ok(status_resp)
    }
    else {
        let e: String = "Wrong token or the token is not active.".to_string();
        return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()));
    }    
}

pub async fn get_user_info(
    payload: &TokenOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr> {
    let user: JadeUser = match get_user_by_api_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(user)
}