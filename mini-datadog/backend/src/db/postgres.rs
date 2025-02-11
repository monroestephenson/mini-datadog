//! postgres.rs
//! Module for Postgres connection and queries.

use tokio_postgres::{Client, NoTls};

pub async fn get_pg_client(db_url: &str) -> Client {
    // Placeholder: connect to Postgres
    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}

// Additional helper functions for queries