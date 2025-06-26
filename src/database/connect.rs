use tokio_postgres::{Client, Error};
use log::error;

pub async fn connect_db(db_url: &str) -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;
    tokio::spawn( async move {
        if let Err(e) = connection.await {
            error!("Database connection failed: {}", e);
        }
    });
    Ok(client)
}