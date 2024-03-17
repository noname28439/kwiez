use std::sync::Arc;

use deadpool_postgres::Object;

use crate::question::{Frage, FragenSet};

#[derive(Debug)]
pub struct AuthToken(pub String);

async fn token_exits(client:&Object, token:&AuthToken) -> bool{
    let x = client.query("SELECT 1 FROM kwiez_users WHERE token = $1;", &[&token.0]).await.unwrap();
    x.len()!=0
}

async fn get_progress(client:&Object, token:&AuthToken) -> i32{
    let x = client.query("SELECT progress FROM kwiez_users WHERE token = $1;", &[&token.0]).await.unwrap();
    match x.get(0) {Some(v) => v.get(0),
        None=>0
    }
}

pub async fn increase_progress(client: &Object, token:&AuthToken){
    client.query("update kwiez_users set progress = progress + 1 where token=$1;", &[&token.0]).await.expect("Could not increase progress");
}

pub async fn current_question(client: &Object, token:&AuthToken, questions:&FragenSet) -> Arc<Frage> {
    let progress = get_progress(client, token).await;
    questions.n_te_frage(progress)
}

pub async fn check_answer(client: &Object, token: &AuthToken, answer:&String, questions:&FragenSet) -> bool{
    let progress = get_progress(client, token).await;
    let frage = questions.n_te_frage(progress);
    let correct = compare_answers(&frage.antwort, answer);
    if correct {increase_progress(client, token).await}//TODO: Don't increase if it was the last answer
    correct
}

fn compare_answers(a1:&String, a2:&String) -> bool{
    a1.to_lowercase().trim() == a2.to_lowercase().trim()
}