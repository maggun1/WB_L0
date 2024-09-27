use axum::{Json, extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use tracing::error;
use crate::{model::Order, state::AppState};

/// Обработчик для получения заказа по order_uid (GET-запрос)
pub async fn get_order_by_uid(
    Path(order_uid): Path<String>,
    state: State<Arc<AppState>>
) -> Result<Json<Order>, StatusCode> {
    match state.get_db().get_order_by_uid(&order_uid).await {
        Ok(Some(order)) => Ok(Json(order)),
        Ok(None) => {
            error!("Order with UID {} not found", order_uid);
            Err(StatusCode::NOT_FOUND)
        },
        Err(e) => {
            error!("Order getting failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// Обработчик для добавления нового заказа (POST-запрос)
pub async fn create_order(
    state: State<Arc<AppState>>,
    Json(order): Json<Order>
) -> Result<StatusCode, StatusCode> {
    match state.get_db().create_order(&order).await {
        Ok(_) => {
            tracing::info!("Order with UID {} successfully created", order.order_uid);
            Ok(StatusCode::CREATED)
        },
        Err(e) => {
            error!("Order creation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
