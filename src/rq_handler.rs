use std::sync::Arc;
use deadpool_postgres::{Object, Pool};
use log::info;
use serde_json::{json, Value};
use warp::reply::Json;
use crate::db::AuthToken;
use crate::{BLOCK_TIMEOUT, db, ExecutionContext, MAX_ANSWER_LENGTH, MAX_NICKNAME_LENGTH};

async fn handle_instructions(body:Value, client:Object, context:Arc<ExecutionContext>) -> Option<Value>{
    let method = *&body[0].as_str()?;
    let auth_token = AuthToken(body[1].as_str()?.to_string());
    let data = &body[2];

    return match method {
        "answer" => {
            let timeout = *context.timeouts.lock().await.get(&auth_token.0).unwrap_or(&0);
            let blocked = timeout>=BLOCK_TIMEOUT;
            let answer = data["answer"].as_str()?.to_string();

            if blocked{return Some(json!({"error": "blocked", "timeout": timeout}))} //timeout-1 = minutes left

            if answer.len()>MAX_ANSWER_LENGTH{return Some(json!({"error": "answer too long"}))}

            let correct = db::check_answer(&client, &auth_token, &answer, context.clone()).await;

            let mut respose = json!({
                "correct": correct
            });
            if correct{
                respose["next"] = match db::current_question(&client, &auth_token, context.clone()).await {
                    Some(v) => json!(*v),
                    None => Value::Null
                };
            }

            Some(respose)
        },
        "stats" => {
            let ranking = db::retrieve_ranking(&client).await;
            let rank = match db::get_rank(&client, &auth_token).await {
                Some(v) => Value::Number(v.into()),
                None => Value::Null
            };
            let top_rank = db::get_top_progress(&client).await.unwrap_or(0);
            Some(json!({
                "count": &context.question_set.count(),
                "progress": db::get_progress(&client, &auth_token).await,
                "rank": rank,
                "top_progress": top_rank,
                "nickname": db::get_own_nickname(&client, &auth_token).await
            }))

        },
        "rename" => {
            let nickname = data["nickname"].as_str()?.to_string();
            if nickname.len()>MAX_NICKNAME_LENGTH{return Some(json!({"error": "nickname too long"}))}
            let contains_profanity = context.profanity_filter.check(&nickname);
            let has_profanity_block = db::get_profanity_block(&client, &auth_token).await.is_some();

            if contains_profanity{
                info!("Profanity from {} detected (profanity: \"{nickname}\", hasBlock: \"{has_profanity_block}\")", auth_token.0);
            }

            if has_profanity_block || contains_profanity{
                db::set_profanity_block(&client, &auth_token, &nickname).await;
                let first_letter:String = nickname.chars().take(1).collect();
                client.query("update kwiez_users set nickname = $2 where token=$1 and nickname is null;", &[&auth_token.0, &first_letter]).await.expect("Could not set nickname to first letter");
            }else{
                db::set_nickname(&client, &auth_token, &nickname).await;
            }
            Some(json!("ok"))
        },
        "ranking" => {
            let ranking = db::retrieve_ranking(&client).await;
            Some(json!(ranking))
        },
        "cq" => {
            Some(match db::current_question(&client, &auth_token, context.clone()).await {
                Some(v) => json!(*v),
                None => return Some(json!({"error": "no question"}))
            })
        },
        "reset_account" => {
            let cq = db::reset_account(&client, &auth_token).await;
            Some(json!("ok"))
        },
        _ => {
            Some(json!("invalid method"))
        }
    };
}

pub async fn api_endpoint(body:Value, pool:Pool, context:Arc<ExecutionContext>) -> Result<impl warp::Reply, std::convert::Infallible>{

    let client = pool.get().await.unwrap();

    Ok(match handle_instructions(body, client, context).await{
        Some(v) => warp::reply::json(&v),
        None => warp::reply::json(&json!("error"))
    })
}