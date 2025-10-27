use axum::{extract::State, Json};
use sqlx::query_as;
use crate::{state::AppState, models::{User, NewUser, Message}};

pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(users)
}

pub async fn add_user(State(state): State<AppState>, Json(payload): Json<NewUser>) -> Json<Message> {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(&payload.name)
        .execute(&state.db)
        .await
        .unwrap();

    Json(Message {
        message: "User added to database!".to_string(),
    })
}
