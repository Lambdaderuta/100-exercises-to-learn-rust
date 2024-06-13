use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::data::{Status, Ticket, TicketDraft, TicketUpdate};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketId(pub u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Done" => Self::Done,
            "InProgress" => Self::InProgress,
            "Todo" | _ => Self::ToDo,
        }
    }
}

impl From<Status> for String {
    fn from(value: Status) -> Self {
        match value {
            Status::ToDo => "Todo".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Done => "Todo".to_string(),
        }
    }
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub async fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;

        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };

        let ticket = Arc::new(RwLock::new(ticket));

        println!("{:?}", ticket);

        self.tickets.insert(id, ticket);

        println!("{:?}", self.tickets);
        id
    }

    pub async fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }

    pub async fn update_ticket(&mut self, ticket_update: TicketUpdate) -> Result<Ticket, ()> {
        let TicketUpdate {
            id,
            title,
            status,
            description,
        } = ticket_update;

        let current_ticket = self.tickets.get(&id);

        if current_ticket.is_none() {
            return Err(());
        }

        let current_ticket = current_ticket.unwrap().read().unwrap().clone();

        let title = title.unwrap_or(current_ticket.title);
        let description = description.unwrap_or(current_ticket.description);
        let converted: String = current_ticket.status.into();
        let status = status.unwrap_or(converted);

        let updated_ticket = Ticket {
            id,
            title,
            description,
            status: status.into(),
        };

        self.tickets
            .insert(id, Arc::new(RwLock::new(updated_ticket.clone())));

        Ok(updated_ticket)
    }
}
