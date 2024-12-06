use axum::http::StatusCode;
use axum::response::IntoResponse;
use cargo_manifest::Manifest;
use serde::{Deserialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use toml::{Table, Value};

#[derive(Deserialize, Debug)]
struct Order {
    item: String,
    quantity: usize,
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.item, self.quantity)
    }
}

pub(crate) async fn manifest(body: String) -> impl IntoResponse {
    if Manifest::from_str(&body).is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
    }

    let table = body.parse::<Table>().unwrap();
    
    if !(table["package"].get("keywords").is_some()
        && table["package"].get("keywords").unwrap().is_array()
        && table["package"]["keywords"]
            .as_array()
            .unwrap()
            .contains(&Value::from("Christmas 2024")))
    {
        return (
            StatusCode::BAD_REQUEST,
            "Magic keyword not provided".to_string(),
        );
    }

    if table.get("package").is_none()
        || table["package"].get("metadata").is_none()
        || table["package"]["metadata"].get("orders").is_none()
    {
        return (StatusCode::NO_CONTENT, String::new());
    }

    let orders = table["package"]["metadata"]["orders"].as_array().unwrap();
    let mut response = Vec::<String>::new();

    for order in orders {
        if let Ok(o) = order.clone().try_into::<Order>() {
            response.push(o.to_string());
        }
    }

    if response.is_empty() {
        (StatusCode::NO_CONTENT, String::new())
    } else {
        (StatusCode::OK, response.join("\n"))
    }
}
