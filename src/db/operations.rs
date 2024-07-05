use super::DatabaseConfig;
use crate::{ Result, User };

pub async fn connect_to_database(config: &DatabaseConfig) -> Result<sqlx::PgPool> {
    // Prepare the connection options
    let connection_options = sqlx::postgres::PgConnectOptions
        ::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database(&config.database)
        .ssl_mode(sqlx::postgres::PgSslMode::Require);

    // Get the connection pool
    let pool = sqlx::PgPool::connect_with(connection_options).await?;

    Ok(pool)
}

pub async fn is_user_signed_up(pool: &sqlx::PgPool, email: &str) -> Result<bool> {
    use sqlx::query;

    // Optionally fetch one record
    match query!("SELECT * FROM users WHERE email = $1", email).fetch_optional(pool).await? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn validate_user(pool: &sqlx::PgPool, user: &User) -> Result<bool> {
    use sqlx::query;

    // Optionally fetch one record
    match
        query!(
            "SELECT * FROM users WHERE email = $1 AND password = $2",
            user.email,
            user.password
        ).fetch_optional(pool).await?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_is_user_signed_up() {
//         // Create a connection pool
//         let pool = connect_to_database().await.unwrap();

//         let result = is_user_signed_up(&pool, "aiknow2023@gmail.com").await;
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result);

//         let result = is_user_signed_up(&pool, "xxx@gmail.com").await;
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(!result);
//     }

//     #[tokio::test]
//     async fn test_validate_user() {
//         // Create a connection pool
//         let pool = connect_to_database().await.unwrap();

//         let result = validate_user(&pool, &User::new("aiknow2023@gmail.com", "admin")).await;
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result);

//         let result = validate_user(&pool, &User::new("aiknow2023@gmail.com", "123xxx")).await;
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(!result);

//         let result = validate_user(&pool, &User::new("xxx@gmail.com", "123xxx")).await;
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(!result);
//     }
// }
