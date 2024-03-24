use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use deadpool_postgres::{Manager, Pool, PoolConfig};
use dotenv::dotenv;
use tokio::sync::Mutex;
use warp::Filter;
use warp::http::Uri;

use crate::question::FragenSet;
use crate::rq_handler::api_endpoint;

mod db;
mod question;
mod rq_handler;


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
        .and_then(api_endpoint);
    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/index.html")));

    let routes = public_route.or(api_ep).or(fallback_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
