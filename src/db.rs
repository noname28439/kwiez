use std::sync::Arc;

use deadpool_postgres::Object;
use serde_json::Value;
use warp::reply::Json;

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
    match x.get(0) {Some(v) => {
        match v.get(0){
            Some(p) => p,
            None => 0
        }
    },
        None=>0
    }
}

pub async fn set_profanity_block(client: &Object, token:&AuthToken, profanity_name:&String){
    client.query("update kwiez_users set profanity_block = $2 where token=$1;", &[&token.0, &profanity_name]).await.expect("Error setting profanity block");
}
pub async fn get_profanity_block(client: &Object, token:&AuthToken) -> Option<String>{
    let res = client.query("select profanity_block from kwiez_users where token=$1;", &[&token.0]).await.expect("Could not check profanity block");
    if let Some(row) = res.get(0){
        return row.get(0);
    }
    None
}


/*
Returns the nickname or if blocked the profanity blocked name to make the user believe that the profane nickname was set
 */
pub async fn get_own_nickname(client: &Object, token:&AuthToken) -> Value {
    let res = client.query("select COALESCE(profanity_block, nickname) from kwiez_users where token=$1;", &[&token.0]).await.expect("Could not get nickname");
    if let Some(row) = res.get(0){
        if let Some(nickname) = row.get(0){
            return Value::String(nickname);
        }
    }
    Value::Null
}

pub async fn set_nickname(client: &Object, token:&AuthToken, nickname:&String){
    client.query("update kwiez_users set nickname = $1 where token=$2;", &[&nickname, &token.0]).await.expect("Could not set nickname");
}

pub async fn get_rank(client:&Object, token:&AuthToken) -> Option<i64>{
    let res = client.query("with ranking as (select token, rank() over (order by progress desc) as rank
                 from kwiez_users
                 where nickname is not null
                 order by progress
    ) select rank from ranking where token=$1;", &[&token.0]).await.expect("Could not get rank");
    let row = res.get(0)?;
    row.get(0)
}

//TODO: Maybe cache the ranking
pub async fn retrieve_ranking(client: &Object) -> Vec<(String, i32)> {
    let res = client.query("select nickname, progress from kwiez_users where nickname is not null order by progress desc limit 3;", &[]).await.expect("Could not create ranking");
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
    let correct = compare_answers(answer, &frage);
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

fn simplify_answer(answer:&String) -> String{
    answer.to_lowercase().trim().replace(",", ".")
}

fn compare_answers(provided_answer:&String, question:&Frage) -> bool{
    let answers = question.antwort.split(";");
    for answer in answers{
        println!("Comparing {} with {}", provided_answer, answer.to_string());
        if simplify_answer(provided_answer) == simplify_answer(&answer.to_string()){
            return true;
        }
    }
    false
}