// #[macro_use]
use anyhow::Result;
use chrono::prelude::*;
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct User {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub password_hash: String,  // Hash not implemented, we store a string for now
    pub email: String,
    pub full_name: String,
    pub is_active: bool,
    pub date_joined: String,
    pub last_login: Option<String>,
}

pub struct NewUser {
    username: String,
    password: String,
    email: String,
    full_name: String,

}
async fn insert_user(user: NewUser, pool: &SqlitePool) -> Result<User> {
    let uuid = uuid::Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let last_login: Option<String> = None;
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users__users (uuid, username, password_hash, email, full_name, is_active, date_joined, last_login)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
        uuid,
        user.username,
        user.password,
        user.email,
        user.full_name,
        false,
        now,
        last_login
    )
    .fetch_one(pool)
    .await?;
    Ok(user)

}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite://test.db")
        .await
        .expect("Could not connect to sqlite db");
    let user = NewUser {
        username: String::from("jon123"),
        password: String::from("easypassword"),
        email: String::from("jondoe@gmail.com"),
        full_name: String::from("John Doe"),
    };
    let user = insert_user(user, &pool).await.expect("Failed to create user");
    println!("User {:?} inserted", user);

    Ok(())
}
