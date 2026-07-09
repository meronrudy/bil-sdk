use crate::{ExposureSchema, FailureMode, RiskReducer, UnderwritingRule, FeatureDictionary};

pub trait DomainPack {
    fn id(&self) -> &'static str;
    fn exposure_schema(&self) -> ExposureSchema;
    fn failure_modes(&self) -> Vec<FailureMode>;
    fn reducers(&self) -> Vec<Box<dyn RiskReducer>>;
    fn underwriting_rules(&self) -> Vec<UnderwritingRule>;
    fn feature_dictionary(&self) -> FeatureDictionary;
}
