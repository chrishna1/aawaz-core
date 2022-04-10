// @generated automatically by Diesel CLI.

diesel::table! {
    app (id) {
        id -> Int4,
        name -> Varchar,
        domain -> Text,
        owner -> Int4,
        deleted -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    comment (id) {
        id -> Int4,
        created_by -> Int4,
        page_id -> Int4,
        parent_id -> Nullable<Int4>,
        content -> Text,
        deleted -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page (id) {
        id -> Int4,
        app_id -> Int4,
        slug -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        handle -> Varchar,
        password -> Text,
        name -> Varchar,
        email -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(app -> users (owner));
diesel::joinable!(comment -> page (page_id));
diesel::joinable!(comment -> users (created_by));
diesel::joinable!(page -> app (app_id));

diesel::allow_tables_to_appear_in_same_query!(
    app,
    comment,
    page,
    users,
);
