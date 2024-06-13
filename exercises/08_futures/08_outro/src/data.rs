use std::sync::Arc;

use tokio::sync::Mutex;

use crate::store::{TicketId, TicketStore};
use serde::{Deserialize, Serialize};
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Deserialize)]
pub struct TicketUpdate {
    pub id: TicketId,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

pub struct AppState {
    pub store: Arc<Mutex<TicketStore>>,
}
