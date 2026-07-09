#[derive(Debug, Clone)]
pub struct FailureMode {
    pub name: String,
}

impl FailureMode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
