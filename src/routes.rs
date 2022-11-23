
use warp::{self, Filter};

use crate::handlers;




pub fn create_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_wasm()
}

fn send_json() -> impl Filter<Extract = (handlers::Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn get_wasm(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone  {


    warp::get()
        .and(warp::path("api"))
        .and(warp::path("WasmHttpExample"))
        .and(warp::path::end())
        .and(send_json()) 
        .and_then(handlers::get_wasm)
}