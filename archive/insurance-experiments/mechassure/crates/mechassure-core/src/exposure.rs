#[derive(Debug, Clone)]
pub struct ExposureSchema {
    pub fields: Vec<String>,
}

impl ExposureSchema {
    pub fn new(fields: Vec<&str>) -> Self {
        Self {
            fields: fields.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}
