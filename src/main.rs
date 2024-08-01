use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::sqlite::SqliteConnection;
use diesel::{connection, prelude::*};

mod models;
mod schema;
use models::*;
use serde_json::json;
use self::schema::blog_posts::dsl::*;

const DATABASE_SRC: &str = "db.db";

pub fn establish_connection() -> SqliteConnection {

    SqliteConnection::establish(DATABASE_SRC)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_SRC))
}

// async fn create_blog_post() -> impl IntoResponse {
//     Response::builder()
//         .status(StatusCode::CREATED)
//         .body(Body::from(""))
//         .unwrap()
// }

async fn list_blog_posts() -> impl IntoResponse {
    let connection = &mut establish_connection();
    let results = blog_posts
        .select(BlogPost::as_select())
        .load(connection)
        .expect("Error getting blog posts");
    Json(results)
}

async fn get_blog(Path(user_id) : Path<i32>) -> impl IntoResponse {
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

#[tokio::main]
async fn main() {
    let router = Router::new()
        // .route("/blog", post(create_blog_post))
        .route("/blog", get(list_blog_posts))
        .route("/blog/:id", get(get_blog));

    println!("Running on http://localhost:3456");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3456").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}