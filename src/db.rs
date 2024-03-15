use tokio_postgres::{Client, NoTls};

enum Schwierigkeit {
    Einfach,
    Mittel,
    Schwer,
}

pub struct AuthToken(String);
pub struct Frage(String, Schwierigkeit);


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

    pub async fn get_next_question(&mut self, token:AuthToken) -> Frage {
        return Frage("Was ist 1+1?".to_string(), Schwierigkeit::Einfach);
    }

}