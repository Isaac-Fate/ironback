mod config;
pub use config::DatabaseConfig;

mod operations;
pub use operations::{ connect_to_database, is_user_signed_up, validate_user };
