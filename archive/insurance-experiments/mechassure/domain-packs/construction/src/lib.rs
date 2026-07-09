use mechassure_core::{
    DomainPack, ExposureSchema, FailureMode, FeatureDictionary, RiskReducer, UnderwritingRule,
};

pub struct HumanProximityReducer;
impl RiskReducer for HumanProximityReducer {
    fn name(&self) -> &'static str {
        "human_proximity"
    }
}

pub struct ExclusionZoneReducer;
impl RiskReducer for ExclusionZoneReducer {
    fn name(&self) -> &'static str {
        "exclusion_zone"
    }
}

pub struct EmergencyStopReducer;
impl RiskReducer for EmergencyStopReducer {
    fn name(&self) -> &'static str {
        "emergency_stop"
    }
}

pub struct TelemetryCompletenessReducer;
impl RiskReducer for TelemetryCompletenessReducer {
    fn name(&self) -> &'static str {
        "telemetry_completeness"
    }
}

pub struct ConstructionPack;

impl DomainPack for ConstructionPack {
    fn id(&self) -> &'static str {
        "construction"
    }

    fn exposure_schema(&self) -> ExposureSchema {
        ExposureSchema::new(vec![
            "autonomous_operation_hours",
            "human_machine_interaction_hours",
            "critical_task_count",
        ])
    }

    fn failure_modes(&self) -> Vec<FailureMode> {
        vec![
            FailureMode::new("human_collision"),
            FailureMode::new("utility_strike"),
            FailureMode::new("property_damage"),
            FailureMode::new("control_loss"),
        ]
    }

    fn reducers(&self) -> Vec<Box<dyn RiskReducer>> {
        vec![
            Box::new(HumanProximityReducer),
            Box::new(ExclusionZoneReducer),
            Box::new(EmergencyStopReducer),
            Box::new(TelemetryCompletenessReducer),
        ]
    }

    fn underwriting_rules(&self) -> Vec<UnderwritingRule> {
        vec![
            UnderwritingRule::max_rate("emergency_stop_rate", 0.05),
            UnderwritingRule::min_score("telemetry_completeness", 0.95),
        ]
    }

    fn feature_dictionary(&self) -> FeatureDictionary {
        let mut dict = FeatureDictionary::new();
        dict.add_feature("kinetic_energy", "Maximum kinetic energy during operation");
        dict.add_feature("utility_proximity", "Minimum distance to known utilities");
        dict.add_feature("worker_proximity", "Minimum distance to human workers");
        dict.add_feature("load_weight", "Maximum load weight handled");
        dict
    }
}
