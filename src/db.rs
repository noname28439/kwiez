use std::sync::Arc;
use deadpool_postgres::{Object};
use crate::ExecutionContext;
use crate::question::{Frage, FragenSet, Schwierigkeit};


#[derive(Debug)]
pub struct AuthToken(pub String);

async fn token_exits(client:&Object, token:AuthToken) -> bool{
    let x = client.query("SELECT 1 FROM kwiez_users WHERE token = $1;", &[&token.0]).await.unwrap();
    x.len()!=0
}

async fn get_progress(client:&Object, token:&AuthToken) -> i32{
    let x = client.query("SELECT progress FROM kwiez_users WHERE token = $1;", &[&token.0]).await.unwrap();
    match x.get(0) {Some(v) => v.get(0),
        None=>0
    }
}

pub async fn current_question(client: &Object, token:&AuthToken, questions:&FragenSet) -> Arc<Frage> {
    let progress = get_progress(client, token).await;
    questions.n_te_frage(progress)
}
