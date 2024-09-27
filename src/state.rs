use std::sync::Arc;
use crate::db::DbClient;

/// Структура глобального состояния приложения
pub struct AppState {
    db: Arc<DbClient>,
}

impl AppState {
    pub fn new(db: DbClient) -> Self {
        Self { db: Arc::new(db) }
    }

    pub fn get_db(&self) -> &Arc<DbClient> {
        &self.db
    }
}