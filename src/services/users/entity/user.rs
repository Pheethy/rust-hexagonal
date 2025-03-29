use axum::extract::Multipart;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub images: Vec<Image>,
    pub departments: Vec<Department>,
}

impl User {
    pub fn new_id(&mut self) {
        let id = Uuid::new_v4();
        self.id = id;
    }

    pub fn set_created_at(&mut self) {
        self.created_at = Utc::now();
    }

    pub fn set_updated_at(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl User {
    pub fn new() -> Self {
        User {
            id: Uuid::new_v4(),
            email: String::new(),
            username: String::new(),
            password: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            images: vec![],
            departments: vec![],
        }
    }
}

pub async fn new_user_with_multipart(mut multipart: Multipart) -> User {
    let mut user: User = User::new();
    /* :Change: multipart to user object */
    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name() {
            match name {
                "email" => user.email = field.text().await.unwrap(),
                "username" => user.username = field.text().await.unwrap(),
                "password" => user.password = field.text().await.unwrap(),
                _ => (),
            }
        }
    }

    user
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}
