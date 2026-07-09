use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct FeatureDictionary {
    pub features: HashMap<String, String>,
}

impl FeatureDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_feature(&mut self, name: &str, description: &str) {
        self.features.insert(name.to_string(), description.to_string());
    }
}
