use std::sync::Arc;

use deadpool_postgres::Object;
use log::{debug, info};
use serde_json::Value;
use warp::reply::Json;

use crate::{ExecutionContext, MAX_SKIPS};
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

pub async fn reset_account(client: &Object, token:&AuthToken){
    client.query("update kwiez_users set progress = 0 where token=$1;", &[&token.0]).await.expect("Could not reset account");
}

pub async fn set_nickname(client: &Object, token:&AuthToken, nickname:&String){
    client.query("update kwiez_users set nickname = $1 where token=$2;", &[&nickname, &token.0]).await.expect("Could not set nickname");
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

pub async fn get_remaining_skips(client: &Object, token: &AuthToken, context: Arc<ExecutionContext>) -> i32{
    let used_skips = client.query("select used_skips from kwiez_users where token=$1;", &[&token.0]).await.expect("Could not get used skips");
    let used_skips:i32 = used_skips.get(0).expect("Could not get used skips").get(0);
    used_skips-MAX_SKIPS
}

pub async fn skip(client: &Object, token: &AuthToken, context: Arc<ExecutionContext>) -> bool{
    if !token_exits(client, token).await {return false}
    let progress = get_progress(client, token).await;
    if progress >= context.question_set.count() as i32{return false}
    let used_skips = client.query("select used_skips from kwiez_users where token=$1;", &[&token.0]).await.expect("Could not get used skips");
    let used_skips:i32 = used_skips.get(0).expect("Could not get used skips").get(0);
    if used_skips<MAX_SKIPS {
        client.query("update kwiez_users set progress = progress + 1, used_skips = used_skips + 1 where token=$1;", &[&token.0]).await.expect("Could not skip");
        return true;
    }
    false
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

pub async fn get_top_progress(client:&Object) -> Option<i32>{
    let res = client.query("select progress from kwiez_users order by progress desc limit 1;", &[]).await.expect("Could not get top progress");
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

pub async fn current_question(client: &Object, token:&AuthToken, context: Arc<ExecutionContext>) -> Option<Arc<Frage>> {
    let progress = get_progress(client, token).await;
    let questions = &context.question_set;
    questions.n_te_frage(progress)
}

pub async fn check_answer(client: &Object, token: &AuthToken, answer:&String, context: Arc<ExecutionContext>) -> bool{
    let progress = get_progress(client, token).await;
    if progress >= context.question_set.count() as i32{return false}
    let questions = &context.question_set;
    let frage = questions.n_te_frage(progress).expect("Invalid question id");
    let correct = compare_answers(answer, &frage);
    if correct {
        if !token_exits(client, token).await {create_user(client, token).await;}
        increase_progress(client, token).await;
        if progress+1 == questions.count() as i32 {
            info!("winner: {}", token.0);
        }
    }
    correct
}

fn simplify_answer(answer:&String) -> String{
    answer.to_lowercase().split_whitespace().collect::<String>()
        .replace(",", ".")
        .replace("€", "").
        replace("\"", "")
        .replace("ä", "ae")
        .replace("ö", "oe")
        .replace("ü", "ue")
        .replace("ß", "ss")
}

fn compare_answers(provided_answer:&String, question:&Frage) -> bool{
    let answers = question.antwort.split(";");
    let provided_answer = simplify_answer(provided_answer);
    for answer in answers{
        let answer = simplify_answer(&answer.to_string());

        let numerical_answer = answer.parse::<f64>().is_ok();
        debug!("comparing \"{provided_answer}\" and \"{answer}\" (Numerical: {numerical_answer})");

        if numerical_answer{
            let provided_answer_num = match provided_answer.parse::<f64>() {
                Ok(v)=> v,
                Err(e)=> {  //provided answer invalid f64
                    return false;
                }
            };
            let answer_num = answer.parse::<f64>().unwrap();

            return answer_num==provided_answer_num;
        }else {
            return provided_answer.contains(&answer);
        }
    }
    false
}

