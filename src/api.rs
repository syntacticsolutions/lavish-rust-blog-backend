
use diesel::prelude::*;
use crate::{database::establish_connection, schema::users::dsl::*};
use rocket::serde::{Serialize, json::Json};

// Create a model
#[derive(Queryable, Serialize)]
pub struct User {
    id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    uid: String,
    image: Option<String>,
}

pub fn get_users() -> Json<Vec<User>> {
    let mut connection = establish_connection();

    let results = users
        .load::<User>(&mut connection)
        .expect("Error loading users");

    return Json(results)
}
