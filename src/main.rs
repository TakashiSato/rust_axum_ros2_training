use r2r::std_msgs;
use r2r::QosProfile;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let node = r2r::Node::create(ctx, "rust_axum_ros2_training_node", "")?;
    let arc_node = Arc::new(Mutex::new(node));

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(json_hello))
        .route("/user", post(create_user))
        .with_state(arc_node.clone());

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

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(
    State(arc_node): State<Arc<Mutex<r2r::Node>>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1111,
        username: payload.username,
    };

    let publisher = arc_node
        .lock()
        .unwrap()
        .create_publisher::<std_msgs::msg::String>("user", QosProfile::default())
        .unwrap();

    let msg = r2r::std_msgs::msg::String {
        data: user.username.clone(),
    };
    publisher.publish(&msg).unwrap();

    (StatusCode::CREATED, Json(user))
}
