use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    email: String,
    password_hash: String
}
