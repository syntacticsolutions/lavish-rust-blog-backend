
table! {
    users (id) {
        id -> Integer,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        uid -> Varchar,
        image -> Nullable<Varchar>
    }
}

table! {
    blog_posts (id) {
        id -> Integer,
        author_id -> Integer,
        title -> Varchar,
        description -> Varchar,
        text -> Mediumtext,
        keyword1 -> Varchar,
        keyword2 -> Varchar,
        image -> Varchar,
        bg_src -> Varchar,
        bg_type -> Varchar,
        active -> Bool,
        updated_date -> BigInt,
        created_date -> BigInt,
    }
}

table! {
    blog_categories (id) {
        id -> Integer,
        label -> Varchar,
    }
}

table! {
    blog_post_categories (post_id, category_id) {
        post_id -> Integer,
        category_id -> Integer,
    }
}

table! {
    blog_post_comments (id) {
        id -> Integer,
        post_id -> Integer,
        author_id -> Integer,
        comment -> Varchar,
    }
}

joinable!(blog_posts -> users (author_id));
joinable!(blog_post_categories -> blog_posts (post_id));
joinable!(blog_post_categories -> blog_categories (category_id));

allow_tables_to_appear_in_same_query!(blog_posts, users, blog_categories, blog_post_categories);
