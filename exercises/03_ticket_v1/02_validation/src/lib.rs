struct Ticket {
    title: String,
    description: String,
    status: String,
}

enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn as_str(&self) -> String {
        match self {
            Status::Done => "Done".to_string(),
            Status::Todo => "To-Do".to_string(),
            Status::InProgress => "In Progress".to_string(),
        }
    }

    fn try_from(value: &String) -> Result<Self, ()> {
        let converted = value.as_str();

        match converted {
            "Done" => Ok(Status::Done),
            "To-Do" => Ok(Status::Done),
            "In Progress" => Ok(Status::Done),
            _ => Err(()),
        }
    }
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Self {
        if title.len() == 0 {
            panic!("Title cannot be empty")
        }

        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes")
        }

        if description.len() == 0 {
            panic!("Description cannot be empty")
        }

        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes")
        }

        let status_res = Status::try_from(&status);

        if status_res.is_err() {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed")
        }

        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
