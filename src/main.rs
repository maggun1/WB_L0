mod model;
mod db;
mod handler;
mod state;
mod logger;

use axum::{Router, routing::{get, post}};
use tracing::info;
use tokio::net::TcpListener;
use std::{sync::Arc, net::SocketAddr};
use crate::state::AppState;
use crate::db::DbClient;

const DATABASE_URL: &str = "postgres://wb:wb@localhost/wb_db";

/// Функция для инициализации и запуска сервера
#[tokio::main]
async fn main() {
    logger::init();
    info!("Log initialized.");

    let db_client = match DbClient::connect(DATABASE_URL).await {
        Ok(client) => {
            info!("Connection to database is successful.");
            client
        },
        Err(e) => {
            eprintln!("Connection to database failed: {}", e);
            std::process::exit(1);
        },
    };

    let shared_state = Arc::new(AppState::new(db_client));

    let app = Router::new()
        .route("/orders/:order_uid", get(handler::get_order_by_uid))
        .route("/orders", post(handler::create_order))
        .with_state(shared_state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let tcp_listener :TcpListener = TcpListener::bind(addr).await.unwrap();

    info!("Server started on {}", addr);
    axum::serve(tcp_listener, app)
        .await
        .unwrap();
}
