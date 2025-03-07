// @generated automatically by Diesel CLI.

diesel::table! {
    posts (slug) {
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        author_id -> Nullable<Int4>,
        is_published -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (slug) {
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 6]
        background_color -> Bpchar,
        #[max_length = 6]
        foreground_color -> Bpchar,
    }
}

diesel::table! {
    tags_to_posts (id) {
        id -> Int4,
        #[max_length = 255]
        post -> Varchar,
        #[max_length = 255]
        tag -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 64]
        email -> Nullable<Varchar>,
        is_staff -> Bool,
    }
}

diesel::joinable!(posts -> users (author_id));
diesel::joinable!(tags_to_posts -> posts (post));
diesel::joinable!(tags_to_posts -> tags (tag));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    tags,
    tags_to_posts,
    users,
);
