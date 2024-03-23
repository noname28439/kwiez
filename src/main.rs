use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use deadpool_postgres::{Manager, Pool, PoolConfig};
use dotenv::dotenv;
use serde_json::{json, Value};
use tokio::sync::Mutex;
use warp::Filter;
use warp::http::Uri;

use crate::db::AuthToken;
use crate::question::{FragenSet};

mod db;
mod question;

async fn ep(body:Value, pool:Pool, context:Arc<ExecutionContext>) -> Result<impl warp::Reply, std::convert::Infallible>{
    let method = *&body[0].as_str().expect("no method specified...");
    let auth_token = AuthToken(body[1].as_str().expect("no auth token specified...").to_string());
    let data = &body[2];

    let client = pool.get().await.unwrap();

    return match method {
        "answer" => {
            let timeout = *context.timeouts.lock().await.get(&auth_token.0).unwrap_or(&0);
            let blocked = timeout>1;
            let answer = data["answer"].as_str().expect("no answer specified...").to_string();

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

            Ok(warp::reply::json(&respose))
        },
        "stats" => {
            Ok(warp::reply::json(&json!({
                "count": &context.question_set.count(),
                "progress": db::get_progress(&client, &auth_token).await,
                "top_progress": db::retrieve_ranking(&client).await[0].1,
                "nickname": db::get_nickname(&client, &auth_token).await
            })))

        },
        "rename" => {
            let nickname = data["nickname"].as_str().expect("no nickname specified...").to_string();
            if nickname.len()>20{return Ok(warp::reply::json(&"nickname too long"))}
            db::set_nickname(&client, &auth_token, &nickname).await;
            Ok(warp::reply::json(&"ok"))
        },
        "ranking" => {
            let ranking = db::retrieve_ranking(&client).await;
            Ok(warp::reply::json(&ranking))
        },
        "cq" => {
            let cq = db::current_question(&client, &auth_token, context.clone()).await;
            Ok(warp::reply::json(&*cq))
        },
        _ => {
            Ok(warp::reply::json(&"invalid method"))
        }
    };
}

pub struct ExecutionContext {
    question_set: Arc<FragenSet>,
    timeouts: Arc<Mutex<HashMap<String, u8>>>
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let file = File::open("Questions.csv").expect("File not found");
    let qset = Arc::new(FragenSet::from_file(BufReader::new(file)));
    let tmgr = Arc::new(Mutex::new(HashMap::new()));

    let execution_context = Arc::new(ExecutionContext {
        question_set: qset,
        timeouts: tmgr.clone()
    });

    let tmgr_clone = tmgr.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            let mut tmgr = tmgr_clone.lock().await;
            for (_token, v) in tmgr.iter_mut(){
                if *v == 0{continue;}
                *v -= 1;
                //println!("dec[{}]: {}", _token, v);
            }
        }
    });


    let db_creds = env::var("DB_CREDS").expect("Environment variable DB_CREDS not set");
    let cfg = tokio_postgres::Config::from_str(&*db_creds).unwrap();
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let pool = Pool::builder(mgr).config(PoolConfig::new(16)).max_size(16).build().unwrap();


    let db_filter = warp::any().map(move ||{pool.clone()});
    let context_filter = warp::any().map(move ||{Arc::clone(&execution_context)});

    let api_ep = warp::path!("api")
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(db_filter.clone())
        .and(context_filter.clone())
        .and_then(ep);
    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/index.html")));

    let routes = public_route.or(api_ep).or(fallback_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
