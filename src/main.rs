use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use deadpool_postgres::{Manager, Pool, PoolConfig};
use dotenv::dotenv;
use env_logger::{Builder, Env, Logger, Target};
use log::{info, warn};
use tokio::sync::Mutex;
use warp::Filter;
use warp::http::Uri;

use crate::profanity_filter::ProfanityFilter;
use crate::question::FragenSet;
use crate::rq_handler::api_endpoint;

mod db;
mod question;
mod rq_handler;
mod profanity_filter;

pub const MAX_SKIPS: i32 = 3;
pub const BLOCK_TIMEOUT: i32 = 2*60;
pub const MAX_NICKNAME_LENGTH: usize = 20;
pub const MAX_ANSWER_LENGTH: usize = 50;
pub const QUESTION_FILE: &str = "Questions.tsv";
pub const PROFANITY_FILTER_WORDLIST: &str = "swearwords.txt";


pub struct ExecutionContext {
    question_set: Arc<FragenSet>,
    timeouts: Arc<Mutex<HashMap<String, i32>>>,
    profanity_filter: ProfanityFilter
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").unwrap_or_default();
    let port = port.parse::<u16>().unwrap_or(8000);

    let db_creds = env::var("DB_CREDS").expect("Environment variable DB_CREDS not set");
    let cfg = tokio_postgres::Config::from_str(&*db_creds).unwrap();
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let pool = Pool::builder(mgr).config(PoolConfig::new(16)).max_size(16).build().unwrap();

    pool.get().await.unwrap().query("create table if not exists kwiez_users (\
    token varchar(64) not null, \
    nickname varchar, \
    progress int default 0, \
    used_skips int default 0, \
    profanity_block varchar);\
    ", &[]).await.expect("Could not create table");

    let profanity_filter:ProfanityFilter = match File::open(PROFANITY_FILTER_WORDLIST) {
        Ok(f) => ProfanityFilter::from_file(f),
        Err(_) => {
            info!("Could not open profanity file, using empty filter...");
            ProfanityFilter::empty()
        }
    };

    let question_set = match FragenSet::from_database(pool.get().await.unwrap()).await {
        Some(qs) => Arc::new(qs),
        None => {
            warn!("Could not retrieve questions from database, trying tsv...");
            Arc::new(match File::open(QUESTION_FILE){
                Ok(f) => FragenSet::from_tsv(BufReader::new(f)),
                Err(e) => {
                    warn!("Could not open tsv file, using placeholder question set...");
                    FragenSet::placeholder()
                }
            })
        }
    };

    let tmgr:Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));

    let execution_context = Arc::new(ExecutionContext {
        question_set,
        timeouts: tmgr.clone(),
        profanity_filter
    });

    let tmgr_clone = tmgr.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let mut tmgr = tmgr_clone.lock().await;
            for (_token, v) in tmgr.iter_mut(){
                if *v <= 0 {continue;}
                *v -= 1;
                //println!("dec[{}]: {}", _token, v);
            }
        }
    });

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

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
