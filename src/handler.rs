// handler.rs
use axum::{Json, extract::Path, http::StatusCode};
use crate::{model::Order, db::DbClient};
use std::sync::Arc;

// Обработчик для получения заказа по UID (GET)
pub async fn get_order(
    Path(order_uid): Path<String>,
    db: Arc<DbClient>
) -> Result<Json<Order>, StatusCode> {
    match db.get_order_by_uid(&order_uid).await {
        Ok(Some(order)) => Ok(Json(order)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Новый обработчик для добавления заказа (POST)
pub async fn create_order(
    Json(order): Json<Order>, // Получаем JSON заказа и десериализуем его в структуру Order
    db: Arc<DbClient>
) -> Result<StatusCode, StatusCode> {
    match db.save_order(&order).await {
        Ok(_) => Ok(StatusCode::CREATED), // Если заказ успешно сохранён, возвращаем код 201 Created
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // При ошибке возвращаем код 500
    }
}
