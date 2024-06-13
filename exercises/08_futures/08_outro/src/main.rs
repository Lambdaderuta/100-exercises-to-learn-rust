use crate::store::TicketStore;
use axum::routing::{get, patch, post};
use axum::Router;
use data::AppState;
use handlers::{create_ticket, get_ticket, update_ticket};
use std::sync::Arc;
use tokio::sync::Mutex;

// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
mod data;
mod handlers;
mod store;

async fn create_app() -> Router {
    let store = AppState {
        store: Arc::new(Mutex::new(TicketStore::new())),
    };

    Router::new()
        .route("/get", get(get_ticket))
        .route("/create", post(create_ticket))
        .route("/update", patch(update_ticket))
        .with_state(Arc::new(store))
}

#[tokio::main]
async fn main() {
    let app = create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
