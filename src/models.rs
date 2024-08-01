use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::blog_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content_html: String,
    pub created_at: NaiveDateTime,
}