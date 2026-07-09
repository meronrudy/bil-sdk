use crate::{
    models::{
        AuditRecord, BiasRecord, ComplianceRecord, LossRecord, LossSummary, ModelRecord,
        QuoteEngineState, ReserveSnapshot, RiskRecord, SubmissionRecord, TriageSnapshot,
    },
    theme::{C_AMBER, C_BLUE, C_GREEN, C_MUTED, C_PURPLE, C_RED},
};

pub const ASSIGNEES: &[&str] = &["M. Chen", "Priya Rao", "J. Martinez"];
pub const URGENCIES: &[&str] = &["Standard", "Priority", "Critical"];
pub const REFERRAL_REASONS: &[&str] = &[
    "High model risk",
    "Bias breach",
    "Limit escalation",
    "Manual governance review",
];
pub const REFERRAL_TARGETS: &[&str] = &["Senior UW Desk", "Actuarial Review", "AI Governance"];
pub const EXPORT_AUDIENCES: &[&str] = &["Board", "Regulator", "Internal Archive"];
pub const SIM_ITERATIONS: &[usize] = &[1_000, 5_000, 10_000];

pub static LOG_POOL: &[(&str, &str, &str)] = &[
    (
        "I",
        "RiskScore-v3",
        "Scored submission UW-2941 - result HIGH 0.87",
    ),
    (
        "W",
        "BiasMonitor",
        "Equalized odds gap 0.09 breached for age<35 cohort",
    ),
    (
        "I",
        "ClaimPredict",
        "Estimated loss for UW-2928: $620K (90% CI)",
    ),
    (
        "E",
        "PricingLLM",
        "PSI drift 0.28 exceeds threshold 0.20 - alerting",
    ),
    (
        "I",
        "AuditLogger",
        "Decision signed: hash a3f2e91 - J.Martinez override",
    ),
    ("D", "ModelRegistry", "ClaimPredict v2.0.5 health check OK"),
    (
        "W",
        "DriftDetector",
        "KL divergence 0.14 rising on ClaimPredict input dist",
    ),
    (
        "I",
        "ComplianceEngine",
        "EU AI Act Art.9 periodic check passed",
    ),
    (
        "I",
        "QuoteEngine",
        "Premium calculated for UW-2930: $44,200",
    ),
    ("D", "DBPool", "Connection pool 12/20 active"),
    (
        "I",
        "FraudSentinel",
        "Claim CL-8821 flagged - fraud score 0.94",
    ),
    (
        "W",
        "BiasMonitor",
        "Disparate impact ratio approaching threshold: 0.82",
    ),
    (
        "I",
        "ReserveEngine",
        "IBNR recalculated: $1.08M (BF method, 91% conf)",
    ),
    ("D", "Scheduler", "Retro rating job queued for AY 2022"),
];

pub fn default_scenario_id() -> String {
    "default".to_string()
}

pub fn default_scenario_title() -> String {
    "AXIOM default scenario".to_string()
}

pub fn default_quote_state() -> QuoteEngineState {
    QuoteEngineState {
        owner: "Pricing Desk".to_string(),
        urgency: "Standard".to_string(),
        routing_status: "Ready for indication".to_string(),
        referral_target: None,
        referral_reason: None,
    }
}

pub fn default_reserve_snapshot() -> ReserveSnapshot {
    ReserveSnapshot {
        scenario: "Baseline reserve refresh".to_string(),
        iterations: 10_000,
        case_reserves: 840_000,
        selected_ibnr: 600_000,
        ulae: 86_400,
        total_reserves: 1_526_400,
        confidence_pct: 91,
        cov: 0.18,
        risk_margin: 152_640,
        status: "ADEQUATE".to_string(),
    }
}

pub fn default_loss_summary() -> LossSummary {
    LossSummary {
        scenario: "Baseline trend".to_string(),
        iterations: 10_000,
        expected_loss_ratio_pct: 63.2,
        ultimate_loss: 2_600_000,
        tail_factor: 1.42,
        severity_trend_pct: 8.4,
        frequency_trend_pct: 4.1,
        pure_premium_trend_pct: 12.9,
        note: "AI liability trend is accelerating due to scaling LLM deployments and expanding regulatory exposure.".to_string(),
    }
}

pub fn default_triage_snapshot() -> TriageSnapshot {
    TriageSnapshot {
        scenario: "Portfolio baseline".to_string(),
        iterations: 10_000,
        model_score: 0.72,
        data_score: 0.61,
        fairness_score: 0.44,
        explainability_score: 0.88,
        operational_score: 0.55,
        composite_score: 0.87,
        composite_label: "HIGH".to_string(),
        recommendation: "Refer to Senior Underwriter".to_string(),
    }
}

pub fn seed_risks() -> Vec<RiskRecord> {
    vec![
        RiskRecord {
            id: "R-001".to_string(),
            category: "Model Risk".to_string(),
            category_color: C_RED,
            description: "LLM hallucination in claims".to_string(),
            frequency: "0.14".to_string(),
            severity: "CRIT".to_string(),
            severity_color: C_RED,
            mitigation: "Human-in-loop".to_string(),
        },
        RiskRecord {
            id: "R-002".to_string(),
            category: "Data Risk".to_string(),
            category_color: C_AMBER,
            description: "Training data poisoning".to_string(),
            frequency: "0.07".to_string(),
            severity: "HIGH".to_string(),
            severity_color: C_AMBER,
            mitigation: "Provenance audit".to_string(),
        },
        RiskRecord {
            id: "R-003".to_string(),
            category: "Fairness".to_string(),
            category_color: C_RED,
            description: "Protected class disparate impact".to_string(),
            frequency: "0.21".to_string(),
            severity: "CRIT".to_string(),
            severity_color: C_RED,
            mitigation: "Bias monitor".to_string(),
        },
        RiskRecord {
            id: "R-004".to_string(),
            category: "Operational".to_string(),
            category_color: C_BLUE,
            description: "Model version rollout failure".to_string(),
            frequency: "0.09".to_string(),
            severity: "HIGH".to_string(),
            severity_color: C_AMBER,
            mitigation: "Canary deploy".to_string(),
        },
        RiskRecord {
            id: "R-005".to_string(),
            category: "Systemic".to_string(),
            category_color: C_PURPLE,
            description: "Correlated AI vendor failure".to_string(),
            frequency: "0.03".to_string(),
            severity: "CRIT".to_string(),
            severity_color: C_RED,
            mitigation: "Diversification".to_string(),
        },
        RiskRecord {
            id: "R-006".to_string(),
            category: "Data Risk".to_string(),
            category_color: C_AMBER,
            description: "Concept drift - silent failure".to_string(),
            frequency: "0.18".to_string(),
            severity: "HIGH".to_string(),
            severity_color: C_AMBER,
            mitigation: "Drift alerting".to_string(),
        },
    ]
}

pub fn seed_submissions() -> Vec<SubmissionRecord> {
    vec![
        SubmissionRecord {
            ref_id: "UW-2941".to_string(),
            insured: "Apex Autonomy Ltd".to_string(),
            line: "AI Liab".to_string(),
            limit: "$5M".to_string(),
            risk_band: "HIGH".to_string(),
            risk_score: 0.87,
            score_color: C_RED,
            flag: "Hallucination risk".to_string(),
            flag_color: C_RED,
            status: "Review".to_string(),
            status_color: C_AMBER,
            owner: "Queue".to_string(),
            urgency: "Standard".to_string(),
            triage_note: "Composite risk remains elevated due to absent human-in-loop control."
                .to_string(),
            referred_to: None,
            referral_reason: None,
        },
        SubmissionRecord {
            ref_id: "UW-2938".to_string(),
            insured: "NeuralPay Inc".to_string(),
            line: "E&O".to_string(),
            limit: "$2M".to_string(),
            risk_band: "MED".to_string(),
            risk_score: 0.54,
            score_color: C_AMBER,
            flag: "Drift detected".to_string(),
            flag_color: C_AMBER,
            status: "In Prog".to_string(),
            status_color: C_BLUE,
            owner: "Pricing Desk".to_string(),
            urgency: "Priority".to_string(),
            triage_note: "Require refreshed model monitoring before bind.".to_string(),
            referred_to: None,
            referral_reason: None,
        },
        SubmissionRecord {
            ref_id: "UW-2935".to_string(),
            insured: "VisionAI Corp".to_string(),
            line: "Cyber".to_string(),
            limit: "$10M".to_string(),
            risk_band: "HIGH".to_string(),
            risk_score: 0.91,
            score_color: C_RED,
            flag: "Bias ΔFairness".to_string(),
            flag_color: C_RED,
            status: "Review".to_string(),
            status_color: C_AMBER,
            owner: "Queue".to_string(),
            urgency: "Critical".to_string(),
            triage_note: "Active fairness breach requires governance sign-off.".to_string(),
            referred_to: None,
            referral_reason: None,
        },
        SubmissionRecord {
            ref_id: "UW-2930".to_string(),
            insured: "Cognify Health".to_string(),
            line: "Prod L".to_string(),
            limit: "$3M".to_string(),
            risk_band: "LOW".to_string(),
            risk_score: 0.22,
            score_color: C_GREEN,
            flag: "-".to_string(),
            flag_color: C_MUTED,
            status: "Quoted".to_string(),
            status_color: C_GREEN,
            owner: "Pricing Desk".to_string(),
            urgency: "Standard".to_string(),
            triage_note: "Guardrail coverage and provenance controls are strong.".to_string(),
            referred_to: None,
            referral_reason: None,
        },
        SubmissionRecord {
            ref_id: "UW-2928".to_string(),
            insured: "AutoDrive AG".to_string(),
            line: "D&O".to_string(),
            limit: "$8M".to_string(),
            risk_band: "MED".to_string(),
            risk_score: 0.61,
            score_color: C_AMBER,
            flag: "Explainability".to_string(),
            flag_color: C_AMBER,
            status: "Actuarial".to_string(),
            status_color: C_BLUE,
            owner: "Actuarial".to_string(),
            urgency: "Priority".to_string(),
            triage_note: "Need revised explanation pack before committee review.".to_string(),
            referred_to: None,
            referral_reason: None,
        },
    ]
}

pub fn seed_loss_rows() -> Vec<LossRecord> {
    vec![
        LossRecord {
            ay: "2020".to_string(),
            m12: "342".to_string(),
            m24: "480".to_string(),
            m36: "510".to_string(),
            m48: "521".to_string(),
            m60: "524".to_string(),
            ultimate: "524".to_string(),
            ultimate_color: C_GREEN,
            method: "Closed".to_string(),
        },
        LossRecord {
            ay: "2021".to_string(),
            m12: "410".to_string(),
            m24: "590".to_string(),
            m36: "635".to_string(),
            m48: "648".to_string(),
            m60: "-".to_string(),
            ultimate: "658*".to_string(),
            ultimate_color: C_AMBER,
            method: "CL proj".to_string(),
        },
        LossRecord {
            ay: "2022".to_string(),
            m12: "521".to_string(),
            m24: "740".to_string(),
            m36: "792".to_string(),
            m48: "-".to_string(),
            m60: "-".to_string(),
            ultimate: "831*".to_string(),
            ultimate_color: C_AMBER,
            method: "BF blend".to_string(),
        },
        LossRecord {
            ay: "2023".to_string(),
            m12: "680".to_string(),
            m24: "970".to_string(),
            m36: "-".to_string(),
            m48: "-".to_string(),
            m60: "-".to_string(),
            ultimate: "1,091*".to_string(),
            ultimate_color: C_AMBER,
            method: "BF blend".to_string(),
        },
        LossRecord {
            ay: "2024".to_string(),
            m12: "890".to_string(),
            m24: "-".to_string(),
            m36: "-".to_string(),
            m48: "-".to_string(),
            m60: "-".to_string(),
            ultimate: "1,440*".to_string(),
            ultimate_color: C_AMBER,
            method: "ELR prior".to_string(),
        },
    ]
}

pub fn seed_models() -> Vec<ModelRecord> {
    vec![
        ModelRecord {
            id: "M-01".to_string(),
            name: "RiskScore-v3".to_string(),
            purpose: "UW scoring".to_string(),
            version: "3.2.1".to_string(),
            drift: "OK".to_string(),
            drift_color: C_GREEN,
            bias: "WARN".to_string(),
            bias_color: C_AMBER,
            explainability: "SHAP".to_string(),
            explainability_color: C_GREEN,
            status: "Live".to_string(),
            status_color: C_GREEN,
        },
        ModelRecord {
            id: "M-02".to_string(),
            name: "ClaimPredict".to_string(),
            purpose: "Loss estimate".to_string(),
            version: "2.0.5".to_string(),
            drift: "WARN".to_string(),
            drift_color: C_AMBER,
            bias: "OK".to_string(),
            bias_color: C_GREEN,
            explainability: "LIME".to_string(),
            explainability_color: C_GREEN,
            status: "Live".to_string(),
            status_color: C_GREEN,
        },
        ModelRecord {
            id: "M-03".to_string(),
            name: "FraudSentinel".to_string(),
            purpose: "Fraud detect".to_string(),
            version: "4.1.0".to_string(),
            drift: "OK".to_string(),
            drift_color: C_GREEN,
            bias: "BREACH".to_string(),
            bias_color: C_RED,
            explainability: "SHAP".to_string(),
            explainability_color: C_GREEN,
            status: "Review".to_string(),
            status_color: C_AMBER,
        },
        ModelRecord {
            id: "M-04".to_string(),
            name: "PricingLLM".to_string(),
            purpose: "Rate advisory".to_string(),
            version: "1.0.2".to_string(),
            drift: "HIGH".to_string(),
            drift_color: C_RED,
            bias: "OK".to_string(),
            bias_color: C_GREEN,
            explainability: "Partial".to_string(),
            explainability_color: C_AMBER,
            status: "Watch".to_string(),
            status_color: C_AMBER,
        },
        ModelRecord {
            id: "M-05".to_string(),
            name: "DocExtract".to_string(),
            purpose: "NLP parsing".to_string(),
            version: "2.3.0".to_string(),
            drift: "OK".to_string(),
            drift_color: C_GREEN,
            bias: "OK".to_string(),
            bias_color: C_GREEN,
            explainability: "Attn".to_string(),
            explainability_color: C_GREEN,
            status: "Live".to_string(),
            status_color: C_GREEN,
        },
    ]
}

pub fn seed_bias_rows() -> Vec<BiasRecord> {
    vec![
        BiasRecord {
            attribute: "Gender".to_string(),
            group: "Female".to_string(),
            approval_rate: "74%".to_string(),
            di_ratio: "0.91".to_string(),
            fpr: "0.12".to_string(),
            fnr: "0.19".to_string(),
            status: "Pass".to_string(),
            status_color: C_GREEN,
        },
        BiasRecord {
            attribute: "Gender".to_string(),
            group: "Male".to_string(),
            approval_rate: "81%".to_string(),
            di_ratio: "1.00".to_string(),
            fpr: "0.09".to_string(),
            fnr: "0.14".to_string(),
            status: "Baseline".to_string(),
            status_color: C_BLUE,
        },
        BiasRecord {
            attribute: "Race".to_string(),
            group: "Group A".to_string(),
            approval_rate: "65%".to_string(),
            di_ratio: "0.80".to_string(),
            fpr: "0.18".to_string(),
            fnr: "0.24".to_string(),
            status: "Watch".to_string(),
            status_color: C_AMBER,
        },
        BiasRecord {
            attribute: "Race".to_string(),
            group: "Group B".to_string(),
            approval_rate: "81%".to_string(),
            di_ratio: "1.00".to_string(),
            fpr: "0.09".to_string(),
            fnr: "0.14".to_string(),
            status: "Baseline".to_string(),
            status_color: C_BLUE,
        },
        BiasRecord {
            attribute: "Age".to_string(),
            group: "<35".to_string(),
            approval_rate: "58%".to_string(),
            di_ratio: "0.72".to_string(),
            fpr: "0.22".to_string(),
            fnr: "0.31".to_string(),
            status: "BREACH".to_string(),
            status_color: C_RED,
        },
        BiasRecord {
            attribute: "Age".to_string(),
            group: "35-55".to_string(),
            approval_rate: "81%".to_string(),
            di_ratio: "1.00".to_string(),
            fpr: "0.09".to_string(),
            fnr: "0.14".to_string(),
            status: "Baseline".to_string(),
            status_color: C_BLUE,
        },
    ]
}

pub fn seed_audit_rows() -> Vec<AuditRecord> {
    vec![
        AuditRecord {
            ts: "2025-04-24 09:41".to_string(),
            actor: "RiskScore-v3".to_string(),
            action: "UW Score".to_string(),
            model: "M-01".to_string(),
            outcome: "HIGH 0.87".to_string(),
            outcome_color: C_RED,
            hash: "a3f2e91…".to_string(),
            detail: "Score signed with model digest v3.2.1 and immutable policy snapshot."
                .to_string(),
        },
        AuditRecord {
            ts: "2025-04-24 09:39".to_string(),
            actor: "J.Martinez".to_string(),
            action: "Override".to_string(),
            model: "M-01".to_string(),
            outcome: "Escalate".to_string(),
            outcome_color: C_AMBER,
            hash: "b11cd44…".to_string(),
            detail: "Manual override references fairness exception and board-approved escalation policy.".to_string(),
        },
        AuditRecord {
            ts: "2025-04-24 08:55".to_string(),
            actor: "ClaimPredict".to_string(),
            action: "Loss Est.".to_string(),
            model: "M-02".to_string(),
            outcome: "$380K".to_string(),
            outcome_color: C_BLUE,
            hash: "cc9921f…".to_string(),
            detail: "Signed loss estimate attached to pricing workbook and monitoring snapshot.".to_string(),
        },
        AuditRecord {
            ts: "2025-04-24 08:30".to_string(),
            actor: "FraudSentinel".to_string(),
            action: "Flag".to_string(),
            model: "M-03".to_string(),
            outcome: "FRAUD 0.94".to_string(),
            outcome_color: C_RED,
            hash: "d448ab2…".to_string(),
            detail: "Fraud flag preserved with feature attribution and reviewer handoff.".to_string(),
        },
        AuditRecord {
            ts: "2025-04-24 07:15".to_string(),
            actor: "System".to_string(),
            action: "Model push".to_string(),
            model: "M-04".to_string(),
            outcome: "Canary 5%".to_string(),
            outcome_color: C_AMBER,
            hash: "e5012cc…".to_string(),
            detail: "Release manifest and rollback hooks recorded for regulatory evidence."
                .to_string(),
        },
    ]
}

pub fn seed_compliance_rows() -> Vec<ComplianceRecord> {
    vec![
        ComplianceRecord {
            framework: "EU AI Act (Art. 9)".to_string(),
            scope: "High-risk AI".to_string(),
            last_audit: "2025-03-01".to_string(),
            findings: "0 critical".to_string(),
            findings_color: C_GREEN,
            status: "PASS".to_string(),
            status_color: C_GREEN,
            next_step: "Refresh transparency annex by 2025-06-15".to_string(),
        },
        ComplianceRecord {
            framework: "NAIC Model Law".to_string(),
            scope: "US insurance AI".to_string(),
            last_audit: "2025-02-15".to_string(),
            findings: "1 minor".to_string(),
            findings_color: C_AMBER,
            status: "COND".to_string(),
            status_color: C_AMBER,
            next_step: "Document senior-override rationale templates".to_string(),
        },
        ComplianceRecord {
            framework: "GDPR / DPDPA".to_string(),
            scope: "Data lineage".to_string(),
            last_audit: "2025-01-20".to_string(),
            findings: "0".to_string(),
            findings_color: C_GREEN,
            status: "PASS".to_string(),
            status_color: C_GREEN,
            next_step: "Monitor data subject request SLA monthly".to_string(),
        },
        ComplianceRecord {
            framework: "SOC 2 Type II".to_string(),
            scope: "Operational".to_string(),
            last_audit: "2024-12-10".to_string(),
            findings: "0".to_string(),
            findings_color: C_GREEN,
            status: "PASS".to_string(),
            status_color: C_GREEN,
            next_step: "Prepare next control sample pull".to_string(),
        },
        ComplianceRecord {
            framework: "SR 11-7".to_string(),
            scope: "Model governance".to_string(),
            last_audit: "2025-03-22".to_string(),
            findings: "2 minor".to_string(),
            findings_color: C_AMBER,
            status: "COND".to_string(),
            status_color: C_AMBER,
            next_step: "Close model validation evidence gap by quarter end".to_string(),
        },
    ]
}
