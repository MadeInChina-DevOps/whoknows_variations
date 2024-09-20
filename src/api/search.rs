use rocket::serde::{json::Json, Serialize};
use rocket::serde::serde_json::json;
use sqlx::postgres::PgPool;
use rocket::State;

#[derive(Serialize)]
struct SearchResponse {
    id: i32, // 32-bit signed integer. To store the id of the document.
    title: String, // String. To store the title of the document.
    url: String, // String. To store the url of the document.
    language: String, // String. To store the language of the document.
    last_updated: NativeDateTime, // DateTime. To store the last updated date of the document.
    content: String, // String. To store the content of the document.
}