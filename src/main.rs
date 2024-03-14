use warp::Filter;
use warp::http::Uri;

#[tokio::main]
async fn main() {

    let public_route = warp::any().and(warp::fs::dir("./frontend/public"));
    //let fallback_route = warp::any().map(|| warp::redirect(Uri::from_static("/public/index.html")));

    let routes = public_route;

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
