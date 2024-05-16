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