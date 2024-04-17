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

use crate::profanity_filter::ProfanityFilter;
use crate::question::FragenSet;
use crate::rq_handler::api_endpoint;

mod db;
mod question;
mod rq_handler;
mod profanity_filter;

pub const BLOCK_TIMEOUT: u8 = 2;
pub const MAX_NICKNAME_LENGTH: usize = 20;
pub const MAX_ANSWER_LENGTH: usize = 50;
pub const QUESTION_FILE: &str = "Questions.tsv";
pub const PROFANITY_FILTER_WORDLIST: &str = "swearwords.txt";


pub struct ExecutionContext {
    question_set: Arc<FragenSet>,
    timeouts: Arc<Mutex<HashMap<String, u8>>>,
    profanity_filter: ProfanityFilter
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_default();
    let port = port.parse::<u16>().unwrap_or(8000);

    let profanity_filter:ProfanityFilter = match File::open(PROFANITY_FILTER_WORDLIST) {
        Ok(f) => ProfanityFilter::from_file(f),
        Err(_) => {
            println!("Could not open profanity file, using empty filter...");
            ProfanityFilter::empty()
        }
    };

    let qset = Arc::new(match File::open(QUESTION_FILE){
        Ok(f) => FragenSet::from_file(BufReader::new(f)),
        Err(e) => {
            println!("Could not open question file, using dummie questions...");
            FragenSet::_dummie()
        }
    });
    let tmgr = Arc::new(Mutex::new(HashMap::new()));

    let execution_context = Arc::new(ExecutionContext {
        question_set: qset,
        timeouts: tmgr.clone(),
        profanity_filter
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

    pool.get().await.unwrap().query("create table if not exists kwiez_users (\
    token varchar(64) not null, \
    nickname varchar, \
    progress int default 0, \
    profanity_block varchar);\
    ", &[]).await.expect("Could not create table");

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
