// @generated automatically by Diesel CLI.

diesel::table! {
    app (id) {
        id -> Int4,
        name -> Varchar,
        domain -> Varchar,
        owner -> Int4,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    comment (id) {
        id -> Int4,
        user_id -> Int4,
        page_id -> Int4,
        parent_id -> Nullable<Int4>,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page (id) {
        id -> Int4,
        app_id -> Int4,
        slug -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Text,
        name -> Nullable<Varchar>,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(app -> users (owner));
diesel::joinable!(comment -> page (page_id));
diesel::joinable!(comment -> users (user_id));
diesel::joinable!(page -> app (app_id));

diesel::allow_tables_to_appear_in_same_query!(app, comment, page, users,);
