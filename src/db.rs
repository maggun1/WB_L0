use crate::model::Order;
use tokio_postgres::NoTls;
use anyhow::Error;

/// Клиент для работы с базой данных
pub struct DbClient {
    pub client: tokio_postgres::Client,
}

impl DbClient {
    /// Функция для подключения к базе данных
    pub async fn connect(conn_str: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection to database failed: {}", e);
            }
        });

        Ok(DbClient { client })
    }

    /// Функция для получения заказа по order_uid
    pub async fn get_order_by_uid(&self, order_uid: &str) -> Result<Option<Order>, Error> {
        let row = self.client
            .query_opt("SELECT data FROM orders WHERE order_uid = $1", &[&order_uid])
            .await?;

        if let Some(row) = row {
            let data: serde_json::Value = row.get(0);
            let order: Order = serde_json::from_value(data)?;
            Ok(Some(order))
        } else {
            Ok(None)
        }
    }

    /// Функция для сохранения нового заказа
    pub async fn create_order(&self, order: &Order) -> Result<(), Error> {
        let order_json = serde_json::to_value(order)?;

        self.client
            .execute(
                "INSERT INTO orders (order_uid, data) VALUES ($1, $2)",
                &[&order.order_uid, &order_json],
            )
            .await?;

        Ok(())
    }
}
