#[macro_use] extern crate rocket;

use user::User;

pub mod user;

use rocket::serde::json::{Json, Value, serde_json::json};
use whoami_service::AppStatus;

#[get("/")]
fn index() -> Json<AppStatus> {
    Json(AppStatus::ok())
}

#[post("/user", format = "application/json", data = "<user>")]
fn post_user(user: Json<User>) -> Value {
    println!("{:#?}", user);
    json!({"status" : "ok"})
}

#[launch]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![post_user])
}
