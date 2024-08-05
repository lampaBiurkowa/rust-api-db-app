// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (id) {
        id -> Integer,
        title -> Text,
        content_html -> Text,
        created_at -> Timestamp,
        category -> Text,
    }
}
