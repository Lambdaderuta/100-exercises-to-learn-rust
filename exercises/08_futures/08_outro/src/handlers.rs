use std::sync::Arc;

use crate::data::{AppState, TicketDraft, TicketUpdate};
use crate::store::TicketId;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_macros::debug_handler;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTicket {
    id: u64,
}

#[debug_handler]
pub async fn get_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GetTicket>,
) -> Response {
    let store = state.store.clone();

    let ticket = store.lock().await.get(TicketId(payload.id)).await;

    println!("{:?}", ticket);

    if let Some(val) = ticket {
        let test = val.read().unwrap().clone();

        Json(test).into_response()
    } else {
        StatusCode::IM_A_TEAPOT.into_response()
    }
}

#[debug_handler]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TicketDraft>,
) -> Response {
    let store = state.store.clone();

    let ticket = store.lock().await.add_ticket(payload).await;

    Json(ticket).into_response()
}

#[debug_handler]
pub async fn update_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TicketUpdate>
) -> Response {
    let store = state.store.clone();

    let ticket_res = store.lock().await.update_ticket(payload).await;

    if let Ok(ticket) = ticket_res {
        Json(ticket).into_response()
    } else {
        StatusCode::IM_A_TEAPOT.into_response()
    }
}
