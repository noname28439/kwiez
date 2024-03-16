use deadpool_postgres::{Object};
use crate::question::{Frage, Schwierigkeit};


pub struct AuthToken(pub String);


pub async fn current_question(client: Object, token:AuthToken) -> Frage {
    let x = client.query("SELECT * from test;", &[]).await;

    println!("X: {:?}", x);
    return Frage("Was ist 1+1?".to_string(), Schwierigkeit::Einfach);
}
