mod db;

use std::env;
use dotenv::dotenv;
use serde_json::json;
use warp::Filter;
use warp::http::Uri;
use crate::db::DatabaseConnection;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_creds = env::var("DB_CREDS").expect("Environment variable DB_CREDS not set");
    let mut db = DatabaseConnection::new(db_creds);

    // let res = db.await.client.query("select * from test;", &[]).await.unwrap();
    // println!("res: {:?}", res);

    let answer_ep = warp::path!("answer").and(warp::body::json()).map(|body: serde_json::Value| {
        let response = json!({"ok":true, "name": body["name"]});
        warp::reply::json(&response)
    });
    let stats_ep = warp::path!("stats").and(warp::body::json()).map(|body: serde_json::Value| {
        warp::reply::json(&"not implemented yet")
    });
    let rename_ep = warp::path!("rename").and(warp::body::json()).map(|body: serde_json::Value| {
        warp::reply::json(&"not implemented yet")
    });
    let ranking_ep = warp::path!("ranking").and(warp::body::json()).map(|body: serde_json::Value| {
        warp::reply::json(&"not implemented yet")
    });
    let current_question_ep = warp::path!("cq").and(warp::body::json()).map(|body: serde_json::Value| {
        warp::reply::json(&"not implemented yet")
    });

    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/index.html")));

    let routes = public_route
        .or(answer_ep)
        .or(stats_ep)
        .or(rename_ep)
        .or(ranking_ep)
        .or(current_question_ep)
        .or(fallback_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
