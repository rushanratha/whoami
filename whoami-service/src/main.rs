#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use whoami_service::AppStatus;

#[get("/")]
fn index() -> Json<AppStatus> {
    Json(AppStatus::warning())
}

#[launch]
fn start() -> _ {
    rocket::build().mount("/", routes![index])
}
