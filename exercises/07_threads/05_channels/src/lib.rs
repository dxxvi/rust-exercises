use std::sync::mpsc::{Receiver, Sender};
use crate::data::TicketDraft;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    let mut ticket_store = store::TicketStore::new();
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert(ticket_draft) => {
                ticket_store.add_ticket(ticket_draft.clone());
                println!("Ticket added {:?}", ticket_draft);
            }
        }
    }
}
