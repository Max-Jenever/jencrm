mod modules;

use axum::{
    routing::{get, post , put , delete},
    Router,
    response::Json,
    extract::{State, Path}
};
use std::net::SocketAddr;
use serde::Serialize;
use modules::database;
use modules::deals::{Deal, CreateDeal, DealStatus};

// Убираем ненужные импорты - будем использовать полные пути

#[derive(Clone)]
struct AppState {
    db_pool: database::DbPool,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
    db_status: String,
}

async fn root() -> Json<StatusMessage> {
    Json(StatusMessage {
        message: "JenCRM API работает!".to_string(),
        db_status: "Не проверено".to_string(),
    })
}

async fn health_check(State(state): State<AppState>) -> Json<StatusMessage> {
    let db_status = match sqlx::query("SELECT 1").execute(&state.db_pool).await {
        Ok(_) => "Подключено".to_string(),
        Err(e) => format!("Ошибка: {}", e),
    };

    Json(StatusMessage {
        message: "Проверка здоровья".to_string(),
        db_status,
    })
}

// GET /api/clients - получить всех клиентов
async fn get_clients(State(state): State<AppState>) -> Json<Vec<modules::clients::Client>> {
    let clients = sqlx::query_as::<_, modules::clients::Client>("SELECT * FROM clients ORDER BY id")
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(clients)
}

// POST /api/clients - создать нового клиента
async fn create_client(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<modules::clients::CreateClient>,
) -> Json<modules::clients::Client> {
    let client = sqlx::query_as::<_, modules::clients::Client>(
        "INSERT INTO clients (first_name, last_name, email, phone, passport_data) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING *"
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.email)
    .bind(payload.phone)
    .bind(payload.passport_data)
    .fetch_one(&state.db_pool)
    .await
    .expect("Failed to create client");

    Json(client)
}

// GET /api/clients/{id} - получить одного клиента по ID
async fn get_client(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<modules::clients::Client>, String> {
    let client = sqlx::query_as::<_, modules::clients::Client>(
        "SELECT * FROM clients WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match client {
        Some(client) => Ok(Json(client)),
        None => Err(format!("Client with id {} not found", id)),
    }
}

// PUT /api/clients/{id} - обновить клиента
async fn update_client(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<modules::clients::CreateClient>,
) -> Result<Json<modules::clients::Client>, String> {
    let client = sqlx::query_as::<_, modules::clients::Client>(
        "UPDATE clients 
         SET first_name = $1, last_name = $2, email = $3, phone = $4, passport_data = $5
         WHERE id = $6
         RETURNING *"
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.email)
    .bind(payload.phone)
    .bind(payload.passport_data)
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match client {
        Some(client) => Ok(Json(client)),
        None => Err(format!("Client with id {} not found", id)),
    }
}

// DELETE /api/clients/{id} - удалить клиента
async fn delete_client(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, String> {
    let result = sqlx::query("DELETE FROM clients WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    if result.rows_affected() > 0 {
        Ok(Json(serde_json::json!({
            "message": format!("Client with id {} deleted", id)
        })))
    } else {
        Err(format!("Client with id {} not found", id))
    }
}

// GET /api/deals - получить все сделки
async fn get_deals(State(state): State<AppState>) -> Json<Vec<Deal>> {
    let deals = sqlx::query_as::<_, Deal>("SELECT * FROM deals ORDER BY id")
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(deals)
}

// POST /api/deals - создать сделку
async fn create_deal(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeal>,
) -> Json<Deal> {
    let status = payload.status.unwrap_or(DealStatus::Draft);
    let commission_amount = payload.calculate_commission();
    
    let deal = sqlx::query_as::<_, Deal>(
        "INSERT INTO deals (client_id, deal_amount, commission_percent, commission_amount, tour_operator, deal_date, payment_due_date, status, description) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
         RETURNING *"
    )
    .bind(payload.client_id)
    .bind(payload.deal_amount)
    .bind(payload.commission_percent)
    .bind(commission_amount)  // Добавляем рассчитанную комиссию
    .bind(payload.tour_operator)
    .bind(payload.deal_date)
    .bind(payload.payment_due_date)
    .bind(status)
    .bind(payload.description)
    .fetch_one(&state.db_pool)
    .await
    .expect("Failed to create deal");

    Json(deal)
}

// GET /api/deals/{id} - получить сделку по ID
async fn get_deal(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Deal>, String> {
    let deal = sqlx::query_as::<_, Deal>("SELECT * FROM deals WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    match deal {
        Some(deal) => Ok(Json(deal)),
        None => Err(format!("Deal with id {} not found", id)),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let db_pool = database::create_db_pool()
        .await
        .expect("Ошибка подключения к БД");

    let state = AppState { db_pool };

    let app = Router::new()
    .route("/", get(root))
    .route("/health", get(health_check))
    // Clients routes
    .route("/api/clients", get(get_clients))
    .route("/api/clients", post(create_client))
    .route("/api/clients/:id", get(get_client))      // Добавляем
    .route("/api/clients/:id", put(update_client))   // Добавляем  
    .route("/api/clients/:id", delete(delete_client)) // Добавляем
    .route("/api/deals", get(get_deals))
    .route("/api/deals", post(create_deal))
    .route("/api/deals/:id", get(get_deal))
    .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> Сервер запущен на http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    )
    .await
    .unwrap();
}