use std::env;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;

use deadpool_postgres::{Manager, Pool, PoolConfig};
use dotenv::dotenv;
use serde_json::{json, Value};
use warp::Filter;
use warp::http::Uri;

use crate::db::AuthToken;
use crate::question::{Frage, FragenSet};

mod db;
mod question;

async fn ep(body:Value, pool:Pool, context:Arc<ExecutionContext>) -> Result<impl warp::Reply, std::convert::Infallible>{
    let method = *&body[0].as_str().expect("no method specified...");
    let auth_token = AuthToken(body[1].as_str().expect("no auth token specified...").to_string());
    let data = &body[2];

    let client = pool.get().await.unwrap();

    return match method {
        "answer" => {
            let correct = db::check_answer(&client, &auth_token, &data["answer"].as_str().expect("no answer specified...").to_string(), &context.question_set).await;
            let mut respose = json!({
                "correct": correct,
                "timeout": 0
            });
            if correct{
                respose["next"] = json!(*db::current_question(&client, &auth_token, &context.question_set).await);
            }

            Ok(warp::reply::json(&respose))
        },
        "stats" => {
            Ok(warp::reply::json(&json!({
                "count": &context.question_set.count(),
                "progress": db::get_progress(&client, &auth_token).await,
                "top_progress": 0, //TODO: Implement top progress
            })))

        },
        "rename" => {
            db::set_nickname(&client, &auth_token, &data["nickname"].as_str().expect("no nickname specified...").to_string()).await;
            Ok(warp::reply::json(&"ok"))
        },
        "ranking" => {
            let ranking = db::ranking(&client, &auth_token, &context.question_set).await;
            Ok(warp::reply::json(&ranking))
        },
        "cq" => {
            let cq = db::current_question(&client, &auth_token, &context.question_set).await;
            Ok(warp::reply::json(&*cq))
        },
        _ => {
            Ok(warp::reply::json(&"invalid method"))
        }
    };
}

pub struct ExecutionContext {
    question_set: Arc<FragenSet>
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let qset = Arc::new(FragenSet::dummie());

    let execution_context = Arc::new(ExecutionContext {
        question_set: qset
    });


    let db_creds = env::var("DB_CREDS").expect("Environment variable DB_CREDS not set");
    let cfg = tokio_postgres::Config::from_str(&*db_creds).unwrap();
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let pool = Pool::builder(mgr).config(PoolConfig::new(16)).max_size(16).build().unwrap();


    let db_filter = warp::any().map(move ||{pool.clone()});
    let context_filter = warp::any().map(move ||{Arc::clone(&execution_context)});

    let api_ep = warp::path!("api")
        .and(warp::body::json())
        .and(db_filter.clone())
        .and(context_filter.clone())
        .and_then(ep);
    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/index.html")));

    let routes = public_route.or(api_ep).or(fallback_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
