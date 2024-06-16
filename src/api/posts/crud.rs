use std::collections::HashMap;

use crate::api::users::crud::User;
use crate::database::establish_connection;
use crate::schema::blog_categories::dsl::blog_categories;
use crate::schema::blog_post_categories::dsl::blog_post_categories;
use crate::schema::blog_posts;
use crate::{schema};

use crate::schema::users::dsl::users;

use diesel::mysql::Mysql;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// Create a model
#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub description: String,
    pub text: String,
    pub keyword1: String,
    pub keyword2: String,
    pub image: String,
    pub bg_src: String,
    pub bg_type: String,
    pub active: bool,
    pub updated_at: i64,   // Using i64 for Unix timestamp
    pub created_date: i64, // Using i64 for Unix timestamp
}
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = blog_posts, check_for_backend(Mysql))]
pub struct PostSummary {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub description: String,
    pub image: String,
    pub updated_date: i64, // Using i64 for Unix timestamp
    pub created_date: i64, // Using i64 for Unix timestamp
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct BlogCategory {
    pub id: i32,
    pub label: String,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BlogPostWithAuthorAndCategories {
    pub post: Post,
    pub author: User,
    pub categories: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PostSummaryWithAuthorAndCategories {
    pub post: PostSummary,
    pub author: User,
    pub categories: Vec<String>,
}

pub async fn get_author_map(author_ids: Vec<i32>) -> Result<HashMap<i32, User>, Status> {
    let mut conn = establish_connection();
    let authors = users
        .filter(schema::users::id.eq_any(author_ids))
        .load::<User>(&mut conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })?;
    Ok(authors
        .into_iter()
        .map(|author| (author.id, author))
        .collect())
}

pub async fn get_category_map(post_ids: &Vec<i32>) -> Result<HashMap<i32, Vec<String>>, Status> {
    use diesel::prelude::*;
    use diesel::result::Error as DieselError;
    use rocket::http::Status;
    use std::collections::HashMap;

    let mut conn = establish_connection();

    // Implement the function asynchronously
    let categories = blog_post_categories
        .filter(schema::blog_post_categories::post_id.eq_any(post_ids))
        .inner_join(
            blog_categories
                .on(schema::blog_post_categories::category_id.eq(schema::blog_categories::id)),
        )
        .select((
            schema::blog_categories::label,
            schema::blog_post_categories::post_id,
        ))
        .load::<(String, i32)>(&mut conn)
        .map_err(|err| match err {
            DieselError::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })?;

    let mut post_category_map: HashMap<i32, Vec<String>> = HashMap::new();
    for (label, post_id) in categories {
        post_category_map
            .entry(post_id)
            .or_insert_with(Vec::new)
            .push(label);
    }
    Ok(post_category_map)
}

pub fn fetch_posts(post_ids: &Vec<i32>) -> Result<Vec<Post>, diesel::result::Error> {
    use crate::schema::blog_posts::dsl::*;

    let mut connection = establish_connection();
    let results = blog_posts
        .filter(id.eq_any(post_ids))
        .load::<Post>(&mut connection)?;
    Ok(results)
}

pub async fn get_post(post_id: i32) -> Result<Json<BlogPostWithAuthorAndCategories>, Status> {
    let post_ids = vec![post_id];
    let posts = fetch_posts(&post_ids);

    let post = posts.unwrap().first().unwrap().clone();

    let author_user_id = post.author_id;

    let author_ids = vec![author_user_id];

    let (author_map, category_map) =
        tokio::try_join!(get_author_map(author_ids), get_category_map(&post_ids))?;

    Ok(Json(BlogPostWithAuthorAndCategories {
        post: post,
        author: author_map[&author_user_id].clone(),
        categories: category_map[&post_id].clone(),
    }))
}

pub async fn get_all_posts() -> Result<Json<Vec<PostSummaryWithAuthorAndCategories>>, Status> {
    use crate::schema::blog_posts::dsl::*;
    let mut conn = establish_connection();

    // Select fields that match the PostSummary struct
    let posts = blog_posts
        .select(PostSummary::as_select())
        .load::<PostSummary>(&mut conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })?;

    let post_ids = posts.iter().map(|post| post.id).collect();
    let author_ids = posts.iter().map(|post| post.author_id).collect();

    let (author_map, category_map) =
        tokio::try_join!(get_author_map(author_ids), get_category_map(&post_ids))?;

    let post_summaries_with_author_and_categories: Vec<PostSummaryWithAuthorAndCategories> = posts
        .into_iter()
        .map(|post| PostSummaryWithAuthorAndCategories {
            author: author_map.get(&post.author_id).cloned().unwrap_or_else(User::default),
            categories: category_map.get(&post.id).cloned().unwrap_or(vec![]),
            post,
        })
        .collect();

    Ok(Json(post_summaries_with_author_and_categories))
}
