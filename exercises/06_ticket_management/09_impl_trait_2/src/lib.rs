use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
    //     self.tickets.push(ticket.into());
    // }

    pub fn add_ticket<T>(&mut self, ticket: T) 
    where
        T: Into<Ticket>
    {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // This won't compile if `add_ticket` uses `impl Trait` syntax in argument position.
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
