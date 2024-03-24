use std::sync::Arc;
use deadpool_postgres::{Object, Pool};
use serde_json::{json, Value};
use crate::db::AuthToken;
use crate::{db, ExecutionContext};

async fn handle_instructions(body:Value, client:Object, context:Arc<ExecutionContext>) -> Option<Value>{
    let method = *&body[0].as_str()?;
    let auth_token = AuthToken(body[1].as_str()?.to_string());
    let data = &body[2];

    return match method {
        "answer" => {
            let timeout = *context.timeouts.lock().await.get(&auth_token.0).unwrap_or(&0);
            let blocked = timeout>1;
            let answer = data["answer"].as_str()?.to_string();

            let mut correct = false;
            if !blocked {
                correct = db::check_answer(&client, &auth_token, &answer, context.clone()).await;
            }
            let mut respose = json!({
                "correct": correct,
                "timeout": timeout,
                "block": blocked
            });
            if correct{
                respose["next"] = json!(*db::current_question(&client, &auth_token, context.clone()).await);
            }

            Some(respose)
        },
        "stats" => {
            Some(json!({
                "count": &context.question_set.count(),
                "progress": db::get_progress(&client, &auth_token).await,
                "top_progress": db::retrieve_ranking(&client).await[0].1,
                "nickname": db::get_nickname(&client, &auth_token).await
            }))

        },
        "rename" => {
            let nickname = data["nickname"].as_str()?.to_string();
            if nickname.len()>20{return Some(json!("nickname too long"))}
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