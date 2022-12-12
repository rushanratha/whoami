use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppStatus {
    status: String
}


impl AppStatus {
    pub fn ok() -> AppStatus {
        AppStatus { status: String::from("Ok") }
    }

    pub fn warning() -> AppStatus {
        AppStatus { status: String::from("Warning") }
    }

    pub fn error() -> AppStatus {
        AppStatus { status: String::from("Error") }
    }
}
