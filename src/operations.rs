#[derive(Debug)]
pub enum Operation {
    Search,
    Create,
    Update,
    Remove,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::Search => write!(f, "Search"),
            Operation::Create => write!(f, "Create"),
            Operation::Update => write!(f, "Update"),
            Operation::Remove => write!(f, "Remove"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Operation::Search.to_string(), "Search");
        assert_eq!(Operation::Create.to_string(), "Create");
        assert_eq!(Operation::Update.to_string(), "Update");
        assert_eq!(Operation::Remove.to_string(), "Remove");
    }
}
