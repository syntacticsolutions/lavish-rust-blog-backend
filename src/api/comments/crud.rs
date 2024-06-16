use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::api::posts::crud::get_author_map;
use crate::api::users::crud::User;
use crate::database::establish_connection;
use crate::schema::blog_post_comments::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Comment {
    id: i32,
    post_id: i32,
    author_id: i32,
    comment: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentWithAuthor {
    comment: Comment,
    author: User,
}

pub async fn find_comments_by_post_id(
    target_post_id: i32,
) -> Result<Json<Vec<CommentWithAuthor>>, Status> {
    let mut conn = establish_connection();

    let comments = blog_post_comments
        .filter(post_id.eq(target_post_id))
        .load::<Comment>(&mut conn)
        .expect("Error loading comments");

    let author_ids = comments.iter().map(|c| c.author_id).collect();

    let author_map = get_author_map(author_ids).await;

    let comments_with_authors: Vec<CommentWithAuthor> = comments
        .into_iter()
        .map(|c| CommentWithAuthor {
            author: author_map
                .clone()
                .unwrap()
                .get(&c.author_id)
                .cloned()
                .unwrap_or_else(User::default),
            comment: c,
        })
        .collect();

    Ok(Json(comments_with_authors))
}
