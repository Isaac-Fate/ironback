use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,

    /// Expiry date timestamp in seconds.
    exp: u64,
}

impl Claims {
    /// Creates a new claims from the given email.
    /// The expiry date will be generated from the current time plus some days.
    pub fn new<S: AsRef<str>>(email: S) -> Self {
        let expiry_date = Utc::now() + chrono::Duration::days(30);

        Self {
            email: email.as_ref().to_string(),
            exp: expiry_date.timestamp() as u64,
        }
    }

    /// Gets the email.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Calculates the expiry date from the exp field.
    pub fn expiry_date(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.exp as i64, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let claims = Claims::new("xxx@gmail.com");
        assert_eq!(claims.email, "xxx@gmail.com");

        println!("{:#?}", claims.expiry_date());
    }
}
