use envconfig::Envconfig;
use dotenv::dotenv;
use crate::{ Result, Error };

#[derive(Debug, Envconfig)]
pub struct DatabaseConfig {
    #[envconfig(from = "POSTGRES_HOST")]
    pub host: String,

    #[envconfig(from = "POSTGRES_PORT")]
    pub port: u16,

    #[envconfig(from = "POSTGRES_USER")]
    pub username: String,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub password: String,

    #[envconfig(from = "POSTGRES_DB")]
    pub database: String,
}

impl DatabaseConfig {
    pub fn init_from_dotenv() -> Result<Self> {
        // Load the .env file
        match dotenv() {
            Err(_) => Err(Error::DatabaseConfig),
            Ok(_) =>
                match Self::init_from_env() {
                    Err(_) => Err(Error::DatabaseConfig),
                    Ok(config) => Ok(config),
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config() {
        let database_config = DatabaseConfig::init_from_dotenv();

        println!("{:?}", database_config);
    }
}
