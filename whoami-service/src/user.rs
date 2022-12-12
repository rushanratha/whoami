use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    email: String,
    first_name: String,
    last_name: String,
    active: bool
}

impl User {
    pub fn new(email: String, first_name: String, last_name: String, active: bool) -> User {
        User {
            email: email,
            first_name: first_name,
            last_name: last_name,
            active: active
        }
    }
}
