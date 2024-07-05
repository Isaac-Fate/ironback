use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub id: Option<i32>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new<S: AsRef<str>>(email: S, password: S) -> Self {
        Self {
            // id: None,
            email: email.as_ref().to_string(),
            password: password.as_ref().to_string(),
        }
    }
}
