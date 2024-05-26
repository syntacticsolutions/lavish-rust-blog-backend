
mod crud;

use rocket::serde::json::Json;
use crate::api::users::crud::{NewUser, User, get_users, create_user, update_user, delete_user};


#[get("/users")]
pub fn get_users_route() -> Json<Vec<User>> {
    return get_users();
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user_route(new_user: Json<NewUser>) -> Json<User> {
    return create_user(new_user);
}

#[put("/users/<user_id>", format = "json", data = "<user_data>")]
pub fn update_user_route(user_id: i32, user_data: Json<NewUser>) -> Json<User> {
    return update_user(user_id, user_data);
}

#[delete("/users/<user_id>")]
pub fn delete_user_route(user_id: i32) -> Result<Json<String>, String> {
    match delete_user(user_id) {
        Ok(_) => Ok(Json(format!(
            "User with id {} deleted successfully",
            user_id
        ))),
        Err(e) => Err(format!("Error deleting user: {}", e)),
    }
}
