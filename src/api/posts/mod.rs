mod crud;

use crate::api::posts::crud::{
    get_all_posts, get_post, BlogPostWithAuthorAndCategories, PostSummaryWithAuthorAndCategories,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/posts")]
pub async fn get_posts_route(
) -> Result<Json<Vec<PostSummaryWithAuthorAndCategories>>, status::Custom<String>> {
    match get_all_posts().await {
        Ok(json_posts) => Ok(json_posts),
        Err(status) if status == Status::NotFound => Err(status::Custom(
            Status::NotFound,
            "No posts found".to_string(),
        )),
        Err(err) => {
            eprintln!("Error fetching posts: {:?}", err);
            Err(status::Custom(
                Status::InternalServerError,
                format!("Error fetching posts: {:?}", err),
            ))
        }
    }
}

#[get("/posts/<post_id>")]
pub async fn get_post_route(
    post_id: i32,
) -> Result<Json<BlogPostWithAuthorAndCategories>, status::Custom<String>> {
    match get_post(post_id).await {
        Ok(json_post) => Ok(json_post),
        Err(status) if status == Status::NotFound => Err(status::Custom(
            Status::NotFound,
            format!("Post with id {} not found", post_id),
        )),
        Err(err) => {
            eprintln!("Error fetching post: {:?}", err);
            Err(status::Custom(
                Status::InternalServerError,
                format!("Error fetching post: {:?}", err),
            ))
        }
    }
}
