use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde_json::json;

use rust_axum_ros2_training::gateway::Gateway;
use rust_axum_ros2_training::user::{CreateUser, User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gateway = Gateway::new()?;
    let arc_gateway = Arc::new(Mutex::new(gateway));

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(json_hello))
        .route("/user", post(create_user))
        .with_state(arc_gateway.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    println!("Hello, world!");
    "Hello, World!"
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({ "message": hello + greeting })))
}

async fn create_user(
    State(arc_gateway): State<Arc<Mutex<Gateway>>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User::new(1111, payload.username.clone());
    let gateway = arc_gateway.lock().unwrap();

    gateway.publish_user(user.clone());

    (StatusCode::CREATED, Json(user))
}
