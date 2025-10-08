use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, DateTime, Utc};

// Enum для статусов сделки
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "deal_status", rename_all = "snake_case")]
pub enum DealStatus {
    Draft,
    Active, 
    Paid,
    Cancelled,
    Completed,
}

// Структура для сделки из БД
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Deal {
    pub id: i32,
    pub client_id: i32,
    pub deal_amount: f64,
    pub commission_percent: f64,
    pub commission_amount: f64,
    pub tour_operator: String,
    pub deal_date: NaiveDate,
    pub payment_due_date: NaiveDate,
    pub status: DealStatus,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Структура для создания сделки
#[derive(Debug, Deserialize)]
pub struct CreateDeal {
    pub client_id: i32,
    pub deal_amount: f64,
    pub commission_percent: f64,
    pub tour_operator: String,
    pub deal_date: NaiveDate,
    pub payment_due_date: NaiveDate,
    pub status: Option<DealStatus>,
    pub description: Option<String>,
}

// В структуру CreateDeal добавляем метод
impl CreateDeal {
    pub fn calculate_commission(&self) -> f64 {
        (self.deal_amount * self.commission_percent) / 100.0
    }
}