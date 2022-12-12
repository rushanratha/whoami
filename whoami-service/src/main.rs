#[macro_use] extern crate rocket;

use rocket::{serde::json::{Json, Value, serde_json::json}, Request};
use whoami_service::AppStatus;
use user::User;

pub mod user;

#[get("/")]
fn index() -> Json<AppStatus> {
    Json(AppStatus::ok())
}

#[post("/user", format = "application/json", data = "<user>")]
fn post_user(user: Json<User>) -> Value {
    println!("{:#?}", user);
    json!({"status" : "ok"})
}

#[catch(404)]
fn not_found(req: &Request) -> Value {
    json!({"message" : "The requested resource could not be found."})
}

#[launch]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![post_user])
        .register("/", catchers![not_found])

}
