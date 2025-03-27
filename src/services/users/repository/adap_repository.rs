use crate::services::users::entity::user::User;
use crate::services::users::port_repository::IUserRepository;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Error, PgPool, Row};
use std::sync::Arc;

pub struct UserRepository {
    pub psql_db: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(psql_db: Arc<PgPool>) -> Self {
        UserRepository { psql_db }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn fetch_all_users(&self) -> Result<Vec<User>> {
        let script_sql = r#"
            SELECT
                COALESCE(array_to_json(array_agg("US")), '[]'::json) AS users
            FROM(
                SELECT
                    users.id,
                    users.username,
                    users.password,
                    users.email,
                    users.created_at::timestamptz AS created_at,
                    users.updated_at::timestamptz AS updated_at,
                    (
                        SELECT
                            COALESCE(array_to_json(array_agg("IMG")), '[]'::json)
                        FROM(
                            SELECT
                                images.id,
                                images.user_id,
                                images.filename,
                                images.url,
                                images.created_at::timestamptz AS created_at,
                                images.updated_at::timestamptz AS updated_at
                            FROM
                                images
                            WHERE
                                images.user_id = users.id
                        ) AS "IMG"
                    ) AS images
                FROM
                    users
            ) AS "US"
        "#;
        let row = sqlx::query(script_sql)
            .fetch_one(&*self.psql_db)
            .await
            .map_err(|e| match e {
                Error::RowNotFound => anyhow::anyhow!("No rows found"),
                e => anyhow::anyhow!(e),
            })?;

        let users_json: Value = row.get("users");

        let users: Vec<User> = serde_json::from_str(&users_json.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to deserialize users: {}", e))?;

        return Ok(users);
    }

    async fn register_user(&self, user: User) -> Result<User> {
        let script_sql = r#"
            INSERT INTO users (id, username, password, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, password, email, created_at, updated_at
        "#;
        
        let row = sqlx::query(script_sql)
            .bind(user.id)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.email)
            .bind(user.created_at)
            .bind(user.updated_at)
            .fetch_one(&*self.psql_db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to register user: {}", e))?;

        let registered_user = User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            email: row.get("email"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            images: vec![],
        };

        Ok(registered_user)
    }
}
