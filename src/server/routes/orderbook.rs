use axum::{extract::State, Json, Router};
use serde_json::{json, Value};

use crate::model::{orderbook::Orders, ModelManager};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/orders", axum::routing::get(get_all_orders))
        .with_state(mm)
}

async fn get_all_orders(State(mm): State<ModelManager>) -> Json<Value> {
    let order = Orders::getOrderById(&mm, 1).await;

    Json(json!(order.unwrap()))
}
