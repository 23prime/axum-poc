use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = "3000";

    let app = Router::new().route("/", get(root));

    axum::Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize)]
struct Hello {
    message: String,
}

impl Hello {
    fn new(message: &str) -> Self {
        Hello {
            message: message.to_string(),
        }
    }
}

async fn root() -> (StatusCode, Json<Hello>) {
    return (StatusCode::OK, Json(Hello::new("Hello, World!")));
}
