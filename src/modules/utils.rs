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

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Attempts to create a connection to a PostgreSQL database given a database
/// URL.
pub async fn create_connection(db_url: &String) -> Result<Pool<Postgres>, JadeErr> {
    let conn = match sqlx::postgres::PgPool::connect(db_url).await{
        Ok(conn) => conn,
        Err(e) => return Err::<Pool<Postgres>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(conn)
}