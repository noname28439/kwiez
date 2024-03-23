use std::sync::Arc;

use deadpool_postgres::Object;
use serde_json::Value;

use crate::ExecutionContext;
use crate::question::Frage;

#[derive(Debug)]
pub struct AuthToken(pub String);

impl Into<String> for AuthToken{
    fn into(self) -> String {
        self.0
    }
}

async fn token_exits(client:&Object, token:&AuthToken) -> bool{
    let x = client.query("SELECT 1 FROM kwiez_users WHERE token = $1;", &[&token.0]).await.unwrap();
    x.len()!=0
}

async fn create_user(client: &Object, token:&AuthToken){
    client.query("insert into kwiez_users values ($1);", &[&token.0]).await.expect("Could not create user");
}

async fn increase_progress(client: &Object, token:&AuthToken){
    client.query("update kwiez_users set progress = progress + 1 where token=$1;", &[&token.0]).await.expect("Could not increase progress");
}

pub async fn get_progress(client:&Object, token:&AuthToken) -> i32{
    let x = client.query("SELECT progress FROM kwiez_users WHERE token = $1;", &[&token.0]).await.unwrap();
    match x.get(0) {Some(v) => v.get(0),
        None=>0
    }
}

pub async fn get_nickname(client: &Object, token:&AuthToken) -> Value {
    let res = client.query("select nickname from kwiez_users where token=$1;", &[&token.0]).await.expect("Could not get nickname");
    match res.get(0){
        Some(row) => {
            Value::String(row.get(0))
        },
        None => Value::Null
    }
}

pub async fn set_nickname(client: &Object, token:&AuthToken, nickname:&String){
    client.query("update kwiez_users set nickname = $1 where token=$2;", &[&nickname, &token.0]).await.expect("Could not set nickname");
}

//TODO: Maybe cache the ranking
pub async fn retrieve_ranking(client: &Object) -> Vec<(String, i32)> {
    let res = client.query("select nickname, progress from kwiez_users where nickname is not null order by progress desc;", &[]).await.expect("Could not create ranking");
    let mut ranking:Vec<(String, i32)> = Vec::new();
    for row in res{
        ranking.push((row.get(0), row.get(1)));
    }
    ranking
}

pub async fn current_question(client: &Object, token:&AuthToken, context: Arc<ExecutionContext>) -> Arc<Frage> {
    let progress = get_progress(client, token).await;
    let questions = &context.question_set;
    questions.n_te_frage(progress).expect("Invalid progress")
}

pub async fn check_answer(client: &Object, token: &AuthToken, answer:&String, context: Arc<ExecutionContext>) -> bool{
    let progress = get_progress(client, token).await;
    let questions = &context.question_set;
    let frage = questions.n_te_frage(progress).expect("Invalid progress");
    let correct = compare_answers(&frage.antwort, answer);
    if correct {
        if progress == 0 {create_user(client, token).await;}
        if progress+1 < questions.count() as i32 {
            increase_progress(client, token).await;
        }else{
            println!("winner: {}", token.0);
        }
    }else{
        let mut a = context.timeouts.lock().await;
        let timeout = a.entry(token.0.clone()).or_insert(0);
        *timeout += 1;
    }
    correct
}

fn compare_answers(a1:&String, a2:&String) -> bool{
    a1.to_lowercase().trim() == a2.to_lowercase().trim()
}