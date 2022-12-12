#[macro_use] extern crate rocket;

use rocket::{serde::json::{Json, Value, serde_json::json}, Request};
use whoami_service::AppStatus;
use user::User;
use loginrequest::LoginRequest;

pub mod user;
pub mod loginrequest;

#[get("/")]
fn index() -> Json<AppStatus> {
    Json(AppStatus::ok())
}

#[post("/register", format = "application/json", data = "<user>")]
fn register_user(user: Json<User>) -> Value {
    println!("{:#?}", user);

    json!({"status" : "ok"})
}

#[post("/login", format = "application/json", data="<login_request>")]
fn login(login_request: Json<LoginRequest>) -> Json<User> {
    println!("received login request {:#?}", login_request);
    
    let email = String::from("john.smith@dev.com");
    let first_name = String::from("John");
    let last_name = String::from("Smith");
    let active = true;
    let user = User::new(email, first_name, last_name, active);

    Json(user)
}

#[catch(404)]
fn not_found(req: &Request) -> Value {
    json!({"message" : "The requested resource could not be found."})
}

#[launch]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![register_user])
        .mount("/", routes![login])
        .register("/", catchers![not_found])

}
