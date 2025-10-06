use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Убираем chrono импорт - SQLx сам конвертит в DateTime
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Client {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub passport_data: Option<serde_json::Value>,
    // SQLx автоматически конвертит TIMESTAMP в chrono::DateTime<Utc>
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateClient {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub passport_data: Option<serde_json::Value>,
}