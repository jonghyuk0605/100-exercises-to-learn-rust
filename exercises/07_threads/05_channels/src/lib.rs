use crate::data::TicketDraft;
use crate::store::TicketStore;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Debug)]
pub enum Command {
    Insert(TicketDraft),
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert(ticket_draft) => {
                store.add_ticket(ticket_draft);
            }
        }
    }
}
