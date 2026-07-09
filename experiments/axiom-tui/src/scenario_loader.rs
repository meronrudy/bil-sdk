use anyhow::{Context, Result};
use std::{fs, path::Path};

use crate::models::Scenario;

pub fn load_default_scenario() -> Result<Scenario> {
    serde_json::from_str(include_str!("../scenarios/default.json"))
        .context("failed to parse bundled default scenario")
}

pub fn load_scenario(path: &Path) -> Result<Scenario> {
    let json = fs::read_to_string(path)
        .with_context(|| format!("failed to read scenario file {}", path.display()))?;
    serde_json::from_str(&json)
        .with_context(|| format!("failed to parse scenario file {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_scenario_loads() {
        let scenario = load_default_scenario().expect("default scenario");
        assert_eq!(scenario.scenario_id, "default");
        assert!(!scenario.risks.is_empty());
        assert!(!scenario.submissions.is_empty());
    }
}
