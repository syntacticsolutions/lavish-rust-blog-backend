use crate::schema::users;
use crate::{database::establish_connection, schema::users::dsl::*};
use diesel::insertable::Insertable;
use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use diesel::result::Error;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    uid: String,
    image: Option<String>,
}
// Create a model
#[derive(Queryable, Serialize, Deserialize)]
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

    return Json(results);
}

pub fn create_user(new_user: Json<NewUser>) -> Json<User> {
    let connection = &mut establish_connection();

    diesel::insert_into(users)
        .values(&*new_user)
        .execute(connection)
        .expect("Error creating user");

    let inserted_user = users::table
        .order(users::id.desc())
        .first::<User>(connection)
        .expect("Error creating user");

    return Json(inserted_user);
}

pub fn update_user(user_id: i32, user_data: Json<NewUser>) -> Json<User> {
    let connection = &mut establish_connection();

    diesel::update(users::table.find(user_id))
        .set((
            users::first_name.eq(&user_data.first_name),
            users::last_name.eq(&user_data.last_name),
            users::email.eq(&user_data.email),
            users::uid.eq(&user_data.uid),
            users::image.eq(&user_data.image),
        ))
        .execute(connection)
        .expect("Error updating user");

    // Query the updated user to return
    let updated_user = users::table
        .find(user_id)
        .first::<User>(connection)
        .expect("Error loading user");

    return Json(updated_user);
}

pub fn delete_user(user_id: i32) -> Result<(), Error>{
    let connection = &mut establish_connection();

    diesel::delete(users::table.find(user_id))
        .execute(connection)?;

    Ok(())
}
