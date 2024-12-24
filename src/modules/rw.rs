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

/// Importing the "hash"
/// function so strings can
/// be hashed.
use bcrypt::hash;

/// Importing the "verify"
/// function so 
/// hashed strings can
/// be verified.
use bcrypt::verify;

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the default
/// hashing speed for hashing
/// strings.
use bcrypt::DEFAULT_COST;

/// Importing the function
/// to get the current time
/// to get proper timestamps.
use super::time::get_time;

/// Importing the stucture that
/// contains information on
/// Jade users.
use super::units::JadeUser;

/// Importing the stucture that
/// contains information on
/// the mood of a Jade user.
use super::units::JadeMood;

/// Importing the structure
/// for storing information
/// on a user's API tokens.
use super::units::APIToken;

/// Importing the function
/// to send an email.
use super::email::send_email;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the structure
/// to conduct operations on
/// a user's moods.
use crate::MoodActionPayload;

/// Importing the structure
/// to conduct operations on
/// a user's account info.
use crate::ChangeEntityPayload;

/// Importing the structure that
/// helps store user-uploaded files.
use super::units::JadeUserFile;

/// Importing the structure
/// to see whether an operation
/// was successful or not.
use super::units::StatusResponse;

/// Importing the structure
/// to conduct operations that only
/// require a user's token.
use super::units::TokenOnlyPayload;

/// Importing the structure
/// to conduct the creation of a 
/// user account.
use super::units::CreateUserPayload;

/// Importing the structure to show
/// active and inactive mooods.
use super::units::UserMoodsResponse;

/// Importing the structure
/// to conduct the creation of a 
/// new API token.
use super::units::CreateTokenPayload;

/// Importing the structure
/// to conduct the deletion of a 
/// new API token.
use super::units::DeleteTokenPayload;

/// Importing the structure
/// to conduct operations that only
/// require a user's handle.
use super::units::UsernameOnlyPayload;

/// Importing the structure to 
/// retrieve information on active
/// API tokens.
use super::units::UserAPITokensPayload;

/// This function attempts to get the
/// user associated with the supplied API
/// token. If this operation succeeds, an
/// instance of the user whom the token belongs
/// to is supplied. If the operation fails, an error
/// is returned.
pub async fn get_user_from_token(
    api_token: &String, 
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr> {
    let api_tokens: Vec<APIToken> = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut username: String = "".to_string();
    for token in api_tokens {
        if &token.token == api_token && token.is_active {
            username = token.username;
        }
        else {}
    }
    if username == "".to_string(){
        let e: String = "No user with the specified API token found.".to_string();
        Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
    else {
        let user: JadeUser = match get_user_by_handle(&username, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(user)
    }
}

pub async fn store_file(
    file: &Vec<u8>,
    api_token: &String, 
    name: &String,
    pool: &Pool<Postgres>
) -> Result<JadeUserFile, JadeErr>{
    let user: JadeUser = match get_user_from_token(api_token,pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let hashed_id: String = match hash(format!("{}:{}", &name, get_time()), DEFAULT_COST){
        Ok(hashed_id) => hashed_id,
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let new_file: JadeUserFile = JadeUserFile{
        file_id: hashed_id,
        file_name: name.to_owned(),
        username: user.username,
        data: file.to_owned()
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO user_files (file_id, username, file_name, data) VALUES ($1, $2, $3, $4)",
        new_file.file_id,
        new_file.username,
        new_file.file_name,
        new_file.data,
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(new_file)  
}

/// This function attempts to verify the email
/// the user has submitted. If the operation succeeds,
/// a boolean "true" is returned. If the operation fails,
/// an error is returned or a boolean "false" is returned.
pub async fn verify_user_email(
    email_token: &String,
    pool: &Pool<Postgres>
) -> Result<bool, JadeErr> {
    let mut result: bool = false;
    let users: Vec<JadeUser> = match sqlx::query_as!(JadeUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut user_vec: Vec<JadeUser> = Vec::new();
    for user in users {
        if &user.email_token == email_token {
            result = true;
            user_vec.push(user);
        }
        else {}
    }
    let hashed_time: String = match hash(get_time(), DEFAULT_COST){
        Ok(hashed_time) => hashed_time,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if user_vec.len() == 1 {}
    else {
        let e: String = "No user with the specified token found.".to_string();
        return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    }
    let user: JadeUser = user_vec[0].clone();
    let _update_op_active: () = match sqlx::query!("UPDATE users SET is_active = $1 WHERE username = $2", true, user.username)
            .execute(pool)
            .await
    {
        Ok(_feedback) => result = true,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _update_token: () = match sqlx::query!("UPDATE users SET email_token = $1 WHERE username = $2", hashed_time, user.username)
            .execute(pool)
            .await
    {
        Ok(_feedback) => result = true,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Attempts to create a new user with the given payload.
/// If this operation succeeds, an instance of the "JadeUser" structure is
/// returned. If this operation fails, an error is returned.
pub async fn write_user(
    payload: &CreateUserPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String
) -> Result<JadeUser, JadeErr> {
    let hashed_pwd = match hash(payload.password.clone(), DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let hashed_email_token = match hash(&format!("{}{}{}", &payload.username, &payload.email, get_time()), DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let hashed_email = match hash(&payload.email, DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let new_user: JadeUser = JadeUser{
        username: payload.username.clone(),
        email: hashed_email.clone(),
        pwd: hashed_pwd,
        email_token: hashed_email_token.clone(),
        is_active: false
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO users (username, email, pwd, email_token, is_active) VALUES ($1, $2, $3, $4, $5)",
        new_user.username,
        new_user.email,
        new_user.pwd,
        new_user.email_token,
        new_user.is_active
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let email_sub: String = format!("Confirm your email address, {}.", &payload.username);
    let from_addr: String = format!("Jade <noreply@{}>", smtp_server);
    let to_addr: String = format!("{} <{}>", &payload.username, &payload.email);
    let message: String = format!("Please copy and paste this link into your browser to confirm your email address: {}/email/verify/{}",smtp_server, hashed_email_token.clone());
    let send_res: bool = match send_email(&from_addr, &to_addr, &email_sub, &message, smtp_server).await {
        Ok(send_res) => send_res,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if send_res{
        let res: JadeUser = match get_user_by_handle(&payload.username, pool).await {
            Ok(res) => res,
            Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(res)
    }
    else {
        let e: String = "Could not send verification email.".to_string();
        Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
    

}

/// Attempts to fetch the user with the given handle from the database.
/// If this operation succeeds, an instance of the "JadeUser" structure is
/// returned. If this operation fails, an error is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_by_handle(
    username: &String,
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr> {
    let users: Vec<JadeUser> = match sqlx::query_as!(JadeUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeUser> = Vec::new();
    for user in users {
        if &user.username == username {
            result.push(user);
        }
        else {}
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else{
        let e: String = format!("User \"{}\" does not exist.", &username);
        Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete a user given one of their API tokens.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn wipe_user(
    payload: &TokenOnlyPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _wipe_op: () = match sqlx::query!("DELETE FROM users WHERE username = $1", token.username)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// Attempts to create a new mood for a user with the given
/// payload. If this operation succeeds, an instance of 
/// the "JadeMood" structure. If this operation fails, an 
/// error is returned.
pub async fn create_new_mood(
    payload: &MoodActionPayload,
    pool: &Pool<Postgres>,
) -> Result<JadeMood, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let username: String = token.username;
    let all_moods: Vec<JadeMood> = match sqlx::query_as!(JadeMood, "SELECT * FROM moods")
        .fetch_all(pool)
        .await
    {
        Ok(all_moods) => all_moods,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    for mood in all_moods{
        let _update_op: () = match sqlx::query!("UPDATE moods SET is_active = $1 WHERE username = $2", false, mood.username)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
        };
    }
    if token.can_set_mood{
        let new_mood: JadeMood = JadeMood {
            mood: payload.mood.clone(),
            is_active: true,
            username: username,
            created_at: get_time()
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO moods (username, is_active, mood, created_at) VALUES ($1, $2, $3, $4)",
            new_mood.username,
            new_mood.is_active,
            new_mood.mood,
            new_mood.created_at,
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(new_mood)
    }
    else {
        let e: String = format!("User \"{}\" does not have the correct permissions.", &username);
        Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete a mood for a user given 
/// one of their API tokens. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1.
pub async fn wipe_mood(
    payload: &MoodActionPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _wipe_op: () = match sqlx::query!("DELETE FROM moods WHERE username = $1", token.username)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// Attempts to create a new API token for a user with
/// the given payload. If this operation succeeds, 
/// an instance of  the "JadeMood" structure. If this 
/// operation fails, an  error is returned.
pub async fn create_new_token(
    payload: &CreateTokenPayload,
    pool: &Pool<Postgres>
) -> Result<APIToken, JadeErr> {
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let is_valid: bool = match verify(&user.pwd,&payload.password){
        Ok(is_valid) => is_valid,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if is_valid {
        let hashed: String = match hash(format!("{}:{}", get_time(), &payload.username), DEFAULT_COST){
            Ok(hashed) => hashed,
            Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let new_token: APIToken = APIToken{
            username: payload.username.clone(),
            token: hashed,
            created_at: get_time(),
            is_active: true,
            can_change_pwd: payload.can_change_pwd,
            can_set_mood: payload.can_set_mood,
            can_delete_user: payload.can_delete_user,
            can_change_email: payload.can_change_email.clone(),
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO api_tokens (username, token, created_at, is_active, can_change_pwd, can_set_mood, can_delete_user, can_change_email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            new_token.username,
            new_token.token,
            new_token.created_at,
            new_token.is_active,
            new_token.can_change_pwd,
            new_token.can_set_mood,
            new_token.can_delete_user,
            new_token.can_change_email
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(new_token)

    }
    else {
        let e: String = format!("Passwords did not match for user \"{}\"!", &payload.username);
        Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete an API token of a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn wipe_token(
    payload: &DeleteTokenPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if user.pwd == payload.password{
        let _wipe_op: () = match sqlx::query!("DELETE FROM users WHERE username = $1", token.username)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: 0 };
        Ok(status)
    }
    else {
        let e: String = format!("Passwords did not match for user \"{}\"!", &payload.username);
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to update the password for a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_password(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr>{
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.username == user.username
    {
        let _update_op: () = match sqlx::query!("UPDATE users SET pwd = $1 WHERE username = $2", payload.new_entity, user.username)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: 0 };
        Ok(status)
    }
    else {
        let e: String = "Token not active or usernames did not match!".to_string();
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to update the mail address for a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_email(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String
) -> Result<StatusResponse, JadeErr>{
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.username == user.username 
    {
        let hashed_email: String = match hash(&payload.new_entity, DEFAULT_COST){
            Ok(hashed_email) => hashed_email,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let _update_op: () = match sqlx::query!("UPDATE users SET email = $1 WHERE username = $2", hashed_email, user.username)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let hashed_email_token = match hash(&format!("{}{}{}", &user.username, &payload.new_entity, get_time()), DEFAULT_COST){
            Ok(hashed) => hashed,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let _update_token_op: () = match sqlx::query!("UPDATE users SET email_token = $1 WHERE username = $2", hashed_email_token, user.username)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let email_sub: String = format!("Confirm your new email address, {}.", &user.username);
        let from_addr: String = format!("Jade <noreply@{}>", smtp_server);
        let to_addr: String = format!("{} <{}>", &user.username, &payload.new_entity);
        let message: String = format!("Please copy and paste this link into your browser to confirm your email address: {}/email/verify/{}",smtp_server, hashed_email_token.clone());
        let send_res: bool = match send_email(&from_addr, &to_addr, &email_sub, &message, smtp_server).await {
            Ok(send_res) => send_res,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        if send_res{
            let status: StatusResponse = StatusResponse{ status: 0};
            Ok(status)
        }
        else {
            let e: String = "Could not send verification email.".to_string();
            Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        }
    }
    else {
        let e: String = "Token not active or usernames did not match!".to_string();
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to fetch the mood of a user with the given
/// username. If this operation succeeds, the currently-active
/// instance of the user's mood is returned. If this operation
/// fails, an error is returned.
pub async fn get_user_mood(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
)-> Result<JadeMood, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let moods: Vec<JadeMood> = match sqlx::query_as!(JadeMood, "SELECT * FROM moods WHERE username = $1", user.username)
        .fetch_all(pool)
        .await
    {
        Ok(moods) => moods,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeMood> = Vec::new();
    for mood in moods {
        if mood.is_active {
            result.push(mood);
        }
        else {}
    }
    if result.len() == 1 {
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("The user \"{}\" either does not exist or has not created any moods.", &user.username);
        Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to retrieve all moods for a user.
/// If this operation is successful, an instance of
/// the "UserMoodsResponse" structure is returned.
/// If this operation fails, an error is returned.
pub async fn get_user_moods(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<UserMoodsResponse, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserMoodsResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let moods: Vec<JadeMood> = match sqlx::query_as!(JadeMood, "SELECT * FROM moods")
        .fetch_all(pool)
        .await
    {
        Ok(moods) => moods,
        Err(e) => return Err::<UserMoodsResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeMood> = Vec::new();
    for mood in moods {
        if mood.username == user.username {
            if mood.is_active {}
            else {
                result.push(mood);
            }
        }
        else {}
    }
    let active_mood: JadeMood = match get_user_mood(payload, pool).await {
        Ok(active_mood) => active_mood,
        Err(e) => return Err::<UserMoodsResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(UserMoodsResponse{ active_mood: active_mood, inactive_moods: result})
}

/// Attempts to retrieve all active API tokens for a user.
/// If this operation is successful, a vector of the
/// instances of the "APIToken" structure is returned.
/// If this operation fails, an error is returned.
pub async fn get_user_tokens(
    payload: &UserAPITokensPayload,
    pool: &Pool<Postgres>
) -> Result<Vec<APIToken>, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if user.pwd == payload.password{
        let tokens: Vec<APIToken>  = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens")
            .fetch_all(pool)
            .await
        {
            Ok(tokens) => tokens,
            Err(e) => return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let mut result: Vec<APIToken> = Vec::new();
        for token in tokens {
            if token.username == user.username {
                if token.is_active {}
                else {
                    result.push(token);
                }
            }
            else {}
        }
        Ok(result)
    }
    else {
        let e: String = format!("Passwords do not match for user \"{}\"!", &user.username);
        Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
    }
}