use tracing_subscriber;

/// Функция инициализации логирования
pub fn init() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
