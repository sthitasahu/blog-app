// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        content -> Text,
        published -> Nullable<Bool>,
        published_at -> Nullable<Timestamp>,
        author_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Nullable<Varchar>,
        password -> Varchar,
    }
}

diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
