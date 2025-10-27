mod state;
mod models;
mod routes;

use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;
use state::AppState;
use crate::routes::{get_users, add_user};

#[tokio::main]
async fn main() {
    let state = AppState::new().await;

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users", post(add_user))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
