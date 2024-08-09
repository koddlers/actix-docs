use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
}
