pub mod crud;

use crate::api::comments::crud::{find_comments_by_post_id, Comment};
use crud::CommentWithAuthor;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/comments/<post_id>")]
pub async fn get_comments_route(
    post_id: i32,
) -> Result<Json<Vec<CommentWithAuthor>>, status::Custom<String>> {
    match find_comments_by_post_id(post_id).await {
        Ok(json_comments) => Ok(json_comments),
        Err(status) if status == Status::NotFound => Err(status::Custom(
            Status::NotFound,
            format!("Comment with id {} not found", post_id),
        )),
        Err(err) => {
            eprintln!("Error fetching comment: {:?}", err);
            Err(status::Custom(
                Status::InternalServerError,
                format!("Error fetching comment: {:?}", err),
            ))
        }
    }
}
