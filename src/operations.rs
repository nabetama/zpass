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
