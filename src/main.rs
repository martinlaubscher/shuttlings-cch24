mod endpoints;

use axum::{routing::get, Router};
use endpoints::d00::{hello_world, seek};
use endpoints::d02::{dest, dest_v6, key, key_v6};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(seek))
        .route("/2/dest", get(dest))
        .route("/2/key", get(key))
        .route("/2/v6/dest", get(dest_v6))
        .route("/2/v6/key", get(key_v6));

    Ok(router.into())
}
