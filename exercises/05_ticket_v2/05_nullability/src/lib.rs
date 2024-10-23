// TODO: Implement `Ticket::assigned_to` using `Option` as the return type.

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

#[allow(dead_code)]
impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
    pub fn assigned_to(&self) -> Option<&String> {
        if let Status::InProgress { assigned_to } = &self.status {
            Some(assigned_to)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    fn test_todo() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        assert!(ticket.assigned_to().is_none());
    }

    #[test]
    fn test_done() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::Done);
        assert!(ticket.assigned_to().is_none());
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        assert_eq!(ticket.assigned_to(), Some(&"Alice".to_string()));
    }
}
