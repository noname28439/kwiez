use std::sync::Arc;
use deadpool_postgres::{Object, Pool};
use serde_json::{json, Value};
use crate::db::AuthToken;
use crate::{BLOCK_TIMEOUT, db, ExecutionContext, MAX_NICKNAME_LENGTH};

async fn handle_instructions(body:Value, client:Object, context:Arc<ExecutionContext>) -> Option<Value>{
    let method = *&body[0].as_str()?;
    let auth_token = AuthToken(body[1].as_str()?.to_string());
    let data = &body[2];

    return match method {
        "answer" => {
            let timeout = *context.timeouts.lock().await.get(&auth_token.0).unwrap_or(&0);
            let blocked = timeout>=BLOCK_TIMEOUT;
            let answer = data["answer"].as_str()?.to_string();

            if blocked{return Some(json!({"error": "blocked"}))}

            if answer.len()>MAX_NICKNAME_LENGTH{return Some(json!({"error": "answer too long"}))}

            let mut correct = false;
            correct = db::check_answer(&client, &auth_token, &answer, context.clone()).await;

            let mut respose = json!({
                "correct": correct
            });
            if correct{
                respose["next"] = json!(*db::current_question(&client, &auth_token, context.clone()).await);
            }

            Some(respose)
        },
        "stats" => {
            let ranking = db::retrieve_ranking(&client).await;
            let rank = match db::get_rank(&client, &auth_token).await {
                Some(v) => Value::Number(v.into()),
                None => Value::Null
            };
            Some(json!({
                "count": &context.question_set.count(),
                "progress": db::get_progress(&client, &auth_token).await,
                "rank": rank,
                "top_progress": ranking[0].1,
                "nickname": db::get_nickname(&client, &auth_token).await
            }))

        },
        "rename" => {
            let nickname = data["nickname"].as_str()?.to_string();
            if nickname.len()>MAX_NICKNAME_LENGTH{return Some(json!({"error": "nickname too long"}))}
            db::set_nickname(&client, &auth_token, &nickname).await;
            Some(json!("ok"))
        },
        "ranking" => {
            let ranking = db::retrieve_ranking(&client).await;
            Some(json!(ranking))
        },
        "cq" => {
            let cq = db::current_question(&client, &auth_token, context.clone()).await;
            Some(json!(*cq))
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