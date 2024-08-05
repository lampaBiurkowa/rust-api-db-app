use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::dsl::*;
use diesel::sqlite::SqliteConnection;
use diesel::{connection, prelude::*};

mod models;
mod schema;
use self::schema::blog_posts::dsl::*;
use models::*;
use serde_json::json;

const DATABASE_SRC: &str = "db.db";

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(DATABASE_SRC)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_SRC))
}

async fn create_blog_post(Json(item): Json<BlogPost>) -> impl IntoResponse {
    let connection = &mut establish_connection();
    let new_id = diesel::insert_into(blog_posts)
        .values(&item)
        .returning(id)
        .get_result::<i32>(connection)
        .unwrap();
    return Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(format!("{new_id}")))
        .unwrap();
}

async fn list_blog_posts() -> impl IntoResponse {
    let connection = &mut establish_connection();
    let results = blog_posts
        .select(BlogPost::as_select())
        .load(connection)
        .expect("Error getting blog posts");
    Json(results)
}

async fn list_blog_posts_category(Path(category_name): Path<String>) -> impl IntoResponse {
    let connection = &mut establish_connection();
    let results = blog_posts
        .filter(category.eq(category_name))
        .select(BlogPost::as_select())
        .load(connection)
        .expect("Error getting blog posts");
    Json(results)
}

async fn get_blog_post(Path(user_id): Path<i32>) -> impl IntoResponse {
    let connection = &mut establish_connection();
    let result = blog_posts
        .find(user_id)
        .select(BlogPost::as_select())
        .first(connection)
        .optional();

    match result {
        Ok(x) => Json(x).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response(),
    }
}

async fn get_blog_count() -> impl IntoResponse {
    let connection = &mut establish_connection();
    let count = blog_posts.count().get_result::<i64>(connection).unwrap();
    Json(count)
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/blog", post(create_blog_post))
        .route("/blog", get(list_blog_posts))
        .route("/blog/category/:category", get(list_blog_posts_category))
        .route("/blog/:id", get(get_blog_post))
        .route("/blog/count", get(get_blog_count));

    println!("Running on http://localhost:3456");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3456").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}