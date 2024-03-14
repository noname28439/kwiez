use tokio_postgres::{Client, NoTls};

pub struct DatabaseConnection {
    connection: String,
    pub client: Client,
}

impl DatabaseConnection {
    pub async fn new(connection: String) -> DatabaseConnection {
        let (client, conn) = tokio_postgres::connect(&connection, NoTls).await.unwrap();

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        DatabaseConnection {
            connection,
            client
        }
    }
}