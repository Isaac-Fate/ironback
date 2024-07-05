use envconfig::Envconfig;
use dotenv::dotenv;
use shuttle_runtime::SecretStore;
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

    pub fn init_from_shuttle_secrets(secrets: &SecretStore) -> Result<Self> {
        // Get host
        let host = match secrets.get("POSTGRES_HOST") {
            Some(host) => host,
            None => {
                return Err(Error::DatabaseConfig);
            }
        };

        // Get port
        let port = match secrets.get("POSTGRES_PORT") {
            Some(port) =>
                match port.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        return Err(Error::DatabaseConfig);
                    }
                }
            None => {
                return Err(Error::DatabaseConfig);
            }
        };

        // Get username
        let username = match secrets.get("POSTGRES_USER") {
            Some(username) => username,
            None => {
                return Err(Error::DatabaseConfig);
            }
        };

        // Get password
        let password = match secrets.get("POSTGRES_PASSWORD") {
            Some(password) => password,
            None => {
                return Err(Error::DatabaseConfig);
            }
        };

        // Get database
        let database = match secrets.get("POSTGRES_DB") {
            Some(database) => database,
            None => {
                return Err(Error::DatabaseConfig);
            }
        };

        // Create the configuration
        Ok(Self { host, port, username, password, database })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn init_from_dotenv() {
        let database_config = DatabaseConfig::init_from_dotenv();

        println!("{:?}", database_config);
    }
}
