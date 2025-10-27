// use serde::{Serialize, Deserialize};
// use sqlx::FromRow;

// #[derive(Serialize, Deserialize, FromRow)]
// pub struct User {
//     pub id: i32,
//     pub name: String,
// }

// #[derive(Serialize)]
// pub struct Message {
//     pub message: String,
// }
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}
