#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
mod database;
mod api;
mod schema;

use rocket::serde::json::Json;
use api::User;

use crate::api::get_users;

#[get("/users")]
fn index() -> Json<Vec<User>>  {
    return get_users();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}