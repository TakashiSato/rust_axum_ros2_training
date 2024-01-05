use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rust_axum_ros2_training::gateway::Gateway;
use rust_axum_ros2_training::task::{CreateTask, Task};
use rust_axum_ros2_training::user::{CreateUser, User};
use serde_json::json;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum GatewayCommand {
    // Get {
    //     key: String,
    //     resp: Responder<Option<Bytes>>,
    // },
    PublishUser { user: User, resp: Responder<()> },
    PublishTask { task: Task, resp: Responder<()> },
}

type Responder<T> = oneshot::Sender<r2r::Result<T>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gateway = Gateway::new()?;
    // let arc_gateway = Arc::new(Mutex::new(gateway));

    let (tx, mut rx) = mpsc::channel(2);

    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                GatewayCommand::PublishUser { user, resp } => {
                    println!("PublishUser: {:?}", user);
                    let res = gateway.publish_user(user);
                    let _ = resp.send(res);
                }
                GatewayCommand::PublishTask { task, resp } => {
                    println!("PublishTask: {:?}", task);
                    let res = gateway.publish_task(task);
                    let _ = resp.send(res);
                }
            }
        }
    });

    // tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(json_hello))
        .route("/user", post(create_user))
        .route("/task", post(create_task))
        .with_state(tx.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

    manager.await.unwrap();

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
    State(tx): State<mpsc::Sender<GatewayCommand>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User::new(1111, payload.username.clone());

    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = GatewayCommand::PublishUser {
        user: user.clone(),
        resp: resp_tx,
    };
    tx.send(cmd).await.unwrap();

    let res = resp_rx.await.unwrap();
    match res {
        Ok(_) => (StatusCode::CREATED, Json(user)),
        Err(e) => {
            println!("Error publishing user: {:?}", e);
            (StatusCode::BAD_REQUEST, Json(user))
        }
    }
}

async fn create_task(
    State(tx): State<mpsc::Sender<GatewayCommand>>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let task = Task::new(2222, payload.taskname.clone());

    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = GatewayCommand::PublishTask {
        task: task.clone(),
        resp: resp_tx,
    };
    tx.send(cmd).await.unwrap();

    let res = resp_rx.await.unwrap();
    match res {
        Ok(_) => (StatusCode::CREATED, Json(task)),
        Err(e) => {
            println!("Error publishing task: {:?}", e);
            (StatusCode::BAD_REQUEST, Json(task))
        }
    }
}
