#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod api;
mod database;
mod schema;

use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::api::users::{
    create_user_route, delete_user_route, get_users_route, update_user_route,
};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:4321", // Add your frontend URL here
        "http://127.0.0.1:4321", // Add any other allowed origins
    ]);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build().mount(
        "/",
        routes![
            get_users_route,
            create_user_route,
            update_user_route,
            delete_user_route
        ],
    ).attach(cors)
}