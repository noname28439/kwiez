mod db;
mod question;

use std::env;
use std::future::Future;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use deadpool_postgres::{Manager, Pool, PoolConfig};
use dotenv::dotenv;
use serde_json::{json, Value};
use tokio::runtime::Runtime;
use tokio::task::block_in_place;
use tokio_postgres::config;
use warp::Filter;
use warp::http::Uri;
use crate::db::{AuthToken};

async fn ep(body:Value, pool:Pool) -> Result<impl warp::Reply, std::convert::Infallible>{
    let method = &body[0].as_str().expect("no method specified...");
    let data = &body[1];

    let client = pool.get().await.unwrap();

    match *method {
        "cq" => {
            let res = db::current_question(client, AuthToken("".to_string())).await;
            return Ok(warp::reply::json(&res));
        },
        _=>{
            return Ok(warp::reply::json(&"invalid method"))
        }
    }

    Ok(warp::reply::json(&"test"))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_creds = env::var("DB_CREDS").expect("Environment variable DB_CREDS not set");
    let cfg = tokio_postgres::Config::from_str(&*db_creds).unwrap();
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let pool = Pool::builder(mgr).config(PoolConfig::new(16)).max_size(16).build().unwrap();


    let db_filter = warp::any().map(move ||{pool.clone()});

    let api_ep = warp::path!("api").and(warp::body::json()).and(db_filter.clone()).and_then(ep);
    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/index.html")));

    let routes = public_route.or(api_ep).or(fallback_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
