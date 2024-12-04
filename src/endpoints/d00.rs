use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

pub(crate) async fn hello_world() -> &'static str {
    "Hello, bird!"
}

pub(crate) async fn seek() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
}
