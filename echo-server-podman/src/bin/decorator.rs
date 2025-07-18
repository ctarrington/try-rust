//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let body = reqwest::get("http://localhost:3000").await;
    let text = match body {
        Ok(body) => body.text().await.unwrap_or_else(|_| String::new()),
        _ => String::new(),
    };
    Html(format!("wrapped|{}|", text))
}
