#[derive(Debug, Clone)]
pub enum RuleType {
    MaxRate(f64),
    MinScore(f64),
}

#[derive(Debug, Clone)]
pub struct UnderwritingRule {
    pub metric: String,
    pub rule_type: RuleType,
}

impl UnderwritingRule {
    pub fn max_rate(metric: &str, threshold: f64) -> Self {
        Self {
            metric: metric.to_string(),
            rule_type: RuleType::MaxRate(threshold),
        }
    }

    pub fn min_score(metric: &str, threshold: f64) -> Self {
        Self {
            metric: metric.to_string(),
            rule_type: RuleType::MinScore(threshold),
        }
    }
}
