use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use super::server::SharedState;
use engine::CashFlow;

#[derive(Deserialize)]
pub struct CreateCashFlow {
    name: String,
    balance: f64,
    max_balance: Option<f64>,
    income_bounded: Option<bool>,
}

pub async fn cashflow_names(State(state): SharedState) -> Json<Vec<CashFlow>> {
    let mut flows = Vec::new();
    for (_, flow) in state.read().await.iter_flow() {
        flows.push(flow.clone());
    }
    Json(flows)
}

pub async fn cashflow_new(
    State(state): SharedState,
    Json(payload): Json<CreateCashFlow>,
) -> StatusCode {
    if let Ok(()) = state
        .write()
        .await
        .new_flow(
            payload.name,
            payload.balance,
            payload.max_balance,
            payload.income_bounded,
        )
        .await
    {
        return StatusCode::CREATED;
    }
    StatusCode::NOT_IMPLEMENTED
}