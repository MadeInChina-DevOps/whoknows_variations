use rocket::serde::{json::Json, Serialize};
use rocket::serde::serde_json::json;
use sqlx::postgres::PgPool;
use rocket::State;

// Have not tested this

#[derive(Serialize)]
struct SearchResponse {
    id: i32, // 32-bit signed integer. To store the id of the document.
    title: String, // String. To store the title of the document.
    url: String, // String. To store the url of the document.
    language: String, // String. To store the language of the document.
    last_updated: NativeDateTime, // DateTime. To store the last updated date of the document.
    content: String, // String. To store the content of the document.
}

// The search function using SQLx and Rocket.
#[get("/search?<query>")] // Define a GET route with optional query parameter.
pub async fn api_search(
    pool: &State<PgPool>, // Get a reference to the PostgreSQL connection pool.
    query: <Option<String>>, // Optional query parameter for search term.
) -> Json<serde_json::Value> { // Return a JSON response.

    // Default to an empty string if no query is provided.
    let query = query.unwrap_or_else(|| "".to_string());

    // Query the database using SQLx and queries to avoid SQL injection.
    let results = sqlx::query_as!(
        SearchResponse, // specify that the result should be deserialized into the SearchResponse struct.
        r#"
        SELECT id, title, url, language, last_updated, content
        FROM pages
        WHERE LANGEUAGE = $1 AND CONTENT ILIKE $2
        "#, // SQL query to select relevant fields from the pages table.
        format!("%{}%", query) // Bind the query parameter to the SQL query.
    )
    .fetch_all(pool.inner()) // Execute the query and fetch all results.
    .await // Await the result.
    .unwrap_or_else(|_| vec![]); // Default to an empty vector if no results are found.

    // Return results as a JSON response, wrapping it in a JSON object
    Json(json!({
        "results": results
    }))
}