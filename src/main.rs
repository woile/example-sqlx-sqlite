// #[macro_use]
use chrono::prelude::*;
use fake::faker::internet::en::SafeEmail;
use fake::faker::{name::en::Name};
use fake::Fake;
use sqlx::{FromRow, SqlitePool};
use std::error;
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub password_hash: String, // Hash not implemented, we store a string for now
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

impl NewUser {
    fn fake() -> Self {
        Self {
            username: Name().fake(),
            password: (8..20).fake::<String>(),
            email: SafeEmail().fake(),
            full_name: Name().fake(),
        }
    }
}

async fn insert_user(user: NewUser, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let uuid = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let last_login: Option<String> = None;
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users__users (uuid, username, password_hash, email, full_name, is_active, date_joined, last_login)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#)
        .bind(uuid)
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(user.full_name)
        .bind(false)
        .bind(now)
        .bind(last_login)
    .fetch_one(&mut conn)
    .await?;
    Ok(user)
}

async fn insert_user_with_macro(user: NewUser, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let uuid = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let last_login: Option<String> = None;
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users__users (uuid, username, password_hash, email, full_name, is_active, date_joined, last_login)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING uuid as "uuid: Uuid", username, password_hash, email, full_name, is_active, date_joined, last_login
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
    .fetch_one(&mut conn)
    .await?;
    Ok(user)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let pool = SqlitePool::connect("sqlite://test.db")
        .await
        .expect("Could not connect to sqlite db");
    let user = NewUser::fake();
    let user = insert_user(user, &pool)
        .await
        .expect("Failed to create user");
    println!("User {:?} inserted", user);
    let user = NewUser::fake();
    let user = insert_user_with_macro(user, &pool)
        .await
        .expect("Failed to create user");

    println!("User {:?} inserted", user);
    Ok(())
}
