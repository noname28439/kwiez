mod db;

use std::env;
use dotenv::dotenv;
use warp::Filter;
use warp::http::Uri;
use crate::db::DatabaseConnection;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_creds = env::var("DB_CREDS").expect("Environment variable DB_CREDS not set");
    let mut db = DatabaseConnection::new(db_creds);

    let res = db.await.client.query("select * from test;", &[]).await.unwrap();
    println!("res: {:?}", res);

    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/index.html")));

    let routes = public_route.or(fallback_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
