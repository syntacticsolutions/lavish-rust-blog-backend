use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// Create a model
#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub uid: String,
    pub image: Option<String>,
}
