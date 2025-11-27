use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Db;

#[derice(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    password: String,
}

impl Db {
    pub async fn create_user(
        &self,
        username: &String,
        password: &String,
    ) -> Result<CreateUserResponse> {
        let u = sqlx::query_as!(CreateUserResponse, "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id" username, password).fetch_one(&self.pool).await?;

        Ok(CreateUserResponse { id: u.id })
    }

    pub async fn get_user_by_username(&self, username: String) -> Result<User> {
        let u = sqlx::query_as!(
            User,
            "SELECT id, username, password FROM users WHERE username=$1",
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(u)
    }

    pub async fn get_user_by_id(&self, id: String) {
        let u = sqlx::quesry_as!(
            User,
            "SELECT id, username, password FROM users WHERE id=$1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
    }
}
