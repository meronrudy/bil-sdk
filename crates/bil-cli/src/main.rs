use bil_canonical::Hash256;
use bil_core::{
    ActorRef, AssuranceLevel, AuthorityRef, BilId, EventId, EvidenceRef, PolicyRef, ProfileId,
    SystemRef,
};
use bil_mir::{
    AuthorityEdge, AuthorityNode, BilEvent, BilEventKind, BilMirGraph, EvidenceRefNode, PolicyNode,
};
use bil_mock::{generate_bank_branch_mock, BankBranchSyntheticConfig};
use bil_replay::ReplayState;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a demo workflow
    Demo {
        #[arg(short, long, default_value = "bank_branch")]
        profile: String,
    },
    /// Check environment capabilities
    Doctor,
    /// Generate a synthetic workflow
    Mock {
        profile: String,
        #[arg(long)]
        seed: Option<u64>,
        #[arg(long)]
        out: Option<String>,
    },
    /// Compile a MIR graph
    Build {
        workflow_file: String,
        #[arg(long)]
        out: Option<String>,
    },
    /// Issue a receipt-backed artifact
    Issue {
        /// New form: input trace or MIR graph path.
        #[arg(long)]
        input: Option<String>,
        /// New form: output receipt JSON path.
        #[arg(long)]
        out: Option<String>,
        /// Receipt capability code.
        #[arg(long, default_value = "assurance-receipt")]
        capability: String,
        /// Backward-compatible positional capability or input path.
        positional_1: Option<String>,
        /// Backward-compatible positional MIR path.
        positional_2: Option<String>,
    },
    /// Verify a receipt
    Verify {
        #[arg(long)]
        receipt: Option<String>,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        pretty: bool,
        /// Backward-compatible positional receipt path.
        receipt_file: Option<String>,
    },
    /// Explain verification findings
    Explain {
        #[arg(long)]
        receipt: Option<String>,
        #[arg(long)]
        report: Option<String>,
        #[arg(long)]
        out: Option<String>,
        /// Backward-compatible positional receipt path.
        receipt_file: Option<String>,
    },
    /// Run conformance tests
    Conformance { group: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SbaExpressEvidenceTrace {
    scenario_id: String,
    profile: String,
    borrower_or_business_ref: String,
    loan_or_account_ref: String,
    generated_at: String,
    events: Vec<SyntheticVendorEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SyntheticVendorEvent {
    event_id: String,
    event_type: String,
    vendor_system: String,
    workflow_stage: String,
    borrower_or_business_ref: Option<String>,
    loan_or_account_ref: Option<String>,
    timestamp: String,
    actor_type: String,
    input_payload_hash: String,
    output_payload_hash: String,
    #[serde(default)]
    policy_refs: Vec<String>,
    #[serde(default)]
    model_refs: Vec<String>,
    #[serde(default)]
    vendor_rule_refs: Vec<String>,
    #[serde(default)]
    human_review_refs: Vec<String>,
    #[serde(default)]
    exception_refs: Vec<String>,
    #[serde(default)]
    consent_refs: Vec<String>,
    #[serde(default)]
    source_system_refs: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Demo { profile } => {
            let demo_profile = match profile.as_str() {
                "bank_branch" => bil_sdk::DemoProfile::BankBranch,
                "loan_decision" => bil_sdk::DemoProfile::LoanDecision,
                "ai_assurance" => bil_sdk::DemoProfile::AiAssurance,
                _ => {
                    println!("Unknown profile: {}", profile);
                    return;
                }
            };

            let demo = bil_sdk::Bil::demo(demo_profile).unwrap();
            let receipt = demo.receipt().unwrap();
            let memo = demo.memo().unwrap();

            fs::create_dir_all("artifacts/demo").unwrap();

            let receipt_json = serde_json::to_string_pretty(&receipt).unwrap();
            write_text("artifacts/demo/ink_receipt.v1.json", &receipt_json).unwrap();

            memo.write_markdown("artifacts/demo/assurance_memo.md")
                .unwrap();

            println!("Demo completed successfully.");
            println!("Artifacts written to artifacts/demo/");
        }
        Commands::Doctor => {
            let report = bil_sdk::Bil::doctor().unwrap();
            println!("Doctor Report:");
            println!("  Healthy: {}", report.is_healthy);
            for msg in report.messages {
                println!("  - {}", msg);
            }
        }
        Commands::Mock { profile, seed, out } => {
            if profile == "bank-branch" {
                let config = BankBranchSyntheticConfig {
                    seed: seed.unwrap_or(0),
                    branch_id: "branch-001".to_string(),
                    include_ai_assist: true,
                    include_human_review: true,
                    include_adverse_action: false,
                    signer_level: AssuranceLevel::L0SoftwareDev,
                };
                let graph = generate_bank_branch_mock(&config);
                let json = serde_json::to_string_pretty(&graph).unwrap();
                write_or_print(out.as_deref(), &json, "mock workflow").unwrap();
            } else if profile == "sba-express-evidence" {
                let trace = default_sba_express_trace(seed.unwrap_or(2026));
                let json = serde_json::to_string_pretty(&trace).unwrap();
                write_or_print(out.as_deref(), &json, "SBA Express evidence trace").unwrap();
            } else {
                println!("Unknown profile: {}", profile);
            }
        }
        Commands::Build { workflow_file, out } => {
            let json = fs::read_to_string(workflow_file).unwrap();
            let graph: BilMirGraph = serde_json::from_str(&json).unwrap();
            let out_json = serde_json::to_string_pretty(&graph).unwrap();
            write_or_print(out.as_deref(), &out_json, "built MIR").unwrap();
        }
        Commands::Issue {
            input,
            out,
            capability,
            positional_1,
            positional_2,
        } => {
            let (input_path, capability_code) =
                resolve_issue_args(input, capability, positional_1, positional_2);
            let json = fs::read_to_string(&input_path).unwrap();
            let graph = parse_issue_input(&json).unwrap();
            let artifact =
                bil_sdk::Bil::issue(graph, bil_ink::CapabilityCode(capability_code)).unwrap();

            let receipt_json = serde_json::to_string_pretty(&artifact.receipt).unwrap();
            if let Some(out_path) = out {
                write_text(out_path, &receipt_json).unwrap();
                println!("Issued receipt to {}", out_path);
            } else {
                println!("{}", receipt_json);
            }
        }
        Commands::Verify {
            receipt,
            out,
            pretty,
            receipt_file,
        } => {
            let receipt_path = receipt
                .as_deref()
                .or(receipt_file.as_deref())
                .expect("receipt path is required");
            let json = fs::read_to_string(receipt_path).unwrap();
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json).unwrap();
            let report = bil_sdk::Bil::verify(&receipt_obj).unwrap();

            if let Some(out_path) = out {
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                write_text(out_path, &report_json).unwrap();
                println!("Wrote verification report to {}", out_path);
            } else if *pretty {
                print_pretty_report(&report);
            } else {
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                println!("{}", report_json);
            }
        }
        Commands::Explain {
            receipt,
            report,
            out,
            receipt_file,
        } => {
            let receipt_path = receipt
                .as_deref()
                .or(receipt_file.as_deref())
                .expect("receipt path is required");
            let json = fs::read_to_string(receipt_path).unwrap();
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json).unwrap();

            let report_obj = if let Some(report_path) = report {
                let report_json = fs::read_to_string(report_path).unwrap();
                serde_json::from_str(&report_json).unwrap()
            } else {
                bil_sdk::Bil::verify(&receipt_obj).unwrap()
            };

            let explanation = bil_sdk::Bil::explain(&report_obj);

            if let Some(out_path) = out {
                write_text(out_path, &explanation.markdown).unwrap();
                println!("Wrote explanation to {}", out_path);
            } else {
                println!("{}", explanation.markdown);
            }
        }
        Commands::Conformance { group } => {
            if let Err(e) = bil_conformance::run_conformance_group(group) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn resolve_issue_args(
    input: &Option<String>,
    capability: &str,
    positional_1: &Option<String>,
    positional_2: &Option<String>,
) -> (String, String) {
    if let Some(input_path) = input {
        return (input_path.clone(), capability.to_string());
    }

    match (positional_1, positional_2) {
        (Some(input_path), None) => (input_path.clone(), capability.to_string()),
        (Some(capability_code), Some(input_path)) => (input_path.clone(), capability_code.clone()),
        _ => panic!("issue requires --input <trace-or-mir.json> or positional input"),
    }
}

fn parse_issue_input(json: &str) -> Result<BilMirGraph, serde_json::Error> {
    if let Ok(graph) = serde_json::from_str::<BilMirGraph>(json) {
        return Ok(graph);
    }

    let trace = serde_json::from_str::<SbaExpressEvidenceTrace>(json)?;
    Ok(trace_to_mir(&trace))
}

fn trace_to_mir(trace: &SbaExpressEvidenceTrace) -> BilMirGraph {
    let events = trace
        .events
        .iter()
        .map(|event| BilEvent {
            id: EventId(event.event_id.clone()),
            kind: map_event_kind(&event.event_type),
        })
        .collect();

    let evidence = trace
        .events
        .iter()
        .map(|event| EvidenceRefNode {
            id: EvidenceRef(format!("evd_{}_output", event.event_id)),
            hash: Hash256::sha256(event.output_payload_hash.as_bytes()),
            kind: Some(format!("{} output payload hash", event.vendor_system)),
        })
        .collect();

    let mut authority_keys = BTreeSet::new();
    let mut authorities = Vec::new();
    for event in &trace.events {
        if authority_keys.insert(event.vendor_system.clone()) {
            let fallback_policy = event
                .policy_refs
                .first()
                .cloned()
                .unwrap_or_else(|| "SBA_EXPRESS_SYNTHETIC_POLICY_V1".to_string());
            authorities.push(AuthorityNode {
                id: AuthorityRef(format!("auth_{}", normalize_ref(&event.vendor_system))),
                edge: AuthorityEdge {
                    actor: ActorRef(event.actor_type.clone()),
                    system: Some(SystemRef(event.vendor_system.clone())),
                    role: event.actor_type.clone(),
                    scope: event.workflow_stage.clone(),
                    policy_ref: PolicyRef(fallback_policy),
                    valid_from_ms: 1783070400000,
                    valid_until_ms: None,
                },
            });
        }
    }

    let mut policy_refs = BTreeSet::new();
    for event in &trace.events {
        for policy_ref in &event.policy_refs {
            policy_refs.insert(policy_ref.clone());
        }
        for rule_ref in &event.vendor_rule_refs {
            policy_refs.insert(rule_ref.clone());
        }
        for model_ref in &event.model_refs {
            policy_refs.insert(model_ref.clone());
        }
    }
    if policy_refs.is_empty() {
        policy_refs.insert("SBA_EXPRESS_SYNTHETIC_POLICY_V1".to_string());
    }

    BilMirGraph {
        graph_id: BilId(trace.scenario_id.clone()),
        profile: ProfileId(trace.profile.clone()),
        events,
        evidence,
        authorities,
        policies: policy_refs
            .into_iter()
            .map(|id| PolicyNode { id: PolicyRef(id) })
            .collect(),
        replay: ReplayState {
            initial_state_hash: Hash256::zero(),
            transition_hashes: vec![],
            final_state_hash: Hash256::zero(),
        },
    }
}

fn map_event_kind(event_type: &str) -> BilEventKind {
    match event_type {
        "CONSENT_CAPTURED" => BilEventKind::ConsentCaptured,
        "KYB_CHECK_COMPLETED" | "FRAUD_SCREENING_COMPLETED" => BilEventKind::PolicyEvaluated,
        "APPLICATION_INTAKE" => BilEventKind::BankBranchIntakeStarted,
        "DOCUMENT_UPLOADED" => BilEventKind::DocumentReceived,
        "AI_DOCUMENT_EXTRACTION" => BilEventKind::EvidenceExtracted,
        "CREDIT_ANALYSIS" => BilEventKind::PolicyEvaluated,
        "HUMAN_UNDERWRITING_REVIEW" => BilEventKind::HumanReviewed,
        "LOAN_DECISION" => BilEventKind::DecisionIssued,
        "PARTNER_ROUTE_OBSERVED" => BilEventKind::VendorRouteObserved,
        _ => BilEventKind::VendorRouteObserved,
    }
}

fn default_sba_express_trace(seed: u64) -> SbaExpressEvidenceTrace {
    let business_ref = format!("hmac_demo_business_{}", seed);
    let loan_ref = format!("hmac_demo_loan_{}", seed);

    SbaExpressEvidenceTrace {
        scenario_id: format!("sba-express-evidence-{}", seed),
        profile: "sba_express_evidence".to_string(),
        borrower_or_business_ref: business_ref.clone(),
        loan_or_account_ref: loan_ref.clone(),
        generated_at: "2026-07-03T12:00:00Z".to_string(),
        events: vec![
            synthetic_event(
                "evt_apiture_consent_001",
                "CONSENT_CAPTURED",
                "APITURE_STYLE_DIGITAL_BANKING",
                "CONSENT_CAPTURED",
                "CONSUMER_USER",
                &business_ref,
                &loan_ref,
                &["SBA_7A_SYNTHETIC_POLICY_V1"],
                &[],
            ),
            synthetic_event(
                "evt_alloy_kyb_001",
                "KYB_CHECK_COMPLETED",
                "ALLOY_STYLE_KYB",
                "KYB_SCREENING",
                "VENDOR_SYSTEM",
                &business_ref,
                &loan_ref,
                &["KYB_FRAUD_SYNTHETIC_POLICY_V1"],
                &["KYB_RISK_RULESET_SYNTHETIC_V1"],
            ),
            synthetic_event(
                "evt_casca_intake_001",
                "APPLICATION_INTAKE",
                "CASCA_STYLE_ORIGINATION",
                "APPLICATION_SUBMITTED",
                "CONSUMER_USER",
                &business_ref,
                &loan_ref,
                &[
                    "SBA_7A_SYNTHETIC_POLICY_V1",
                    "LIVE_OAK_STYLE_CREDIT_POLICY_SYNTHETIC_V1",
                ],
                &["APPLICATION_COMPLETENESS_RULE_V1"],
            ),
            synthetic_event(
                "evt_casca_extract_001",
                "AI_DOCUMENT_EXTRACTION",
                "CASCA_STYLE_AI_EXTRACTION",
                "DOCUMENT_EXTRACTION",
                "AI_ASSISTANT",
                &business_ref,
                &loan_ref,
                &["DOCUMENT_HANDLING_SYNTHETIC_POLICY_V1"],
                &["CASCA_STYLE_EXTRACTION_MODEL_SYNTHETIC_V1"],
            ),
            synthetic_event(
                "evt_casca_credit_001",
                "CREDIT_ANALYSIS",
                "CASCA_STYLE_CREDIT_ANALYSIS",
                "CREDIT_ANALYSIS",
                "AI_ASSISTANT",
                &business_ref,
                &loan_ref,
                &["LIVE_OAK_STYLE_CREDIT_POLICY_SYNTHETIC_V1"],
                &["SBA_EXPRESS_SCORECARD_SYNTHETIC_V1"],
            ),
            synthetic_event(
                "evt_casca_review_001",
                "HUMAN_UNDERWRITING_REVIEW",
                "CASCA_STYLE_UNDERWRITING_REVIEW",
                "HUMAN_REVIEW",
                "HUMAN_UNDERWRITER",
                &business_ref,
                &loan_ref,
                &["HUMAN_REVIEW_SYNTHETIC_POLICY_V1"],
                &[],
            ),
            synthetic_event(
                "evt_infinant_partner_001",
                "PARTNER_ROUTE_OBSERVED",
                "INFINANT_STYLE_EMBEDDED_BANKING",
                "PARTNER_ROUTE",
                "PARTNER_PLATFORM",
                &business_ref,
                &loan_ref,
                &["PARTNER_ROUTING_SYNTHETIC_POLICY_V1"],
                &[],
            ),
            synthetic_event(
                "evt_casca_decision_001",
                "LOAN_DECISION",
                "CASCA_STYLE_DECISIONING",
                "DECISION_ISSUED",
                "HUMAN_UNDERWRITER",
                &business_ref,
                &loan_ref,
                &[
                    "SBA_7A_SYNTHETIC_POLICY_V1",
                    "LIVE_OAK_STYLE_CREDIT_POLICY_SYNTHETIC_V1",
                ],
                &["DECISION_REASONING_RULE_SYNTHETIC_V1"],
            ),
        ],
    }
}

fn synthetic_event(
    event_id: &str,
    event_type: &str,
    vendor_system: &str,
    workflow_stage: &str,
    actor_type: &str,
    business_ref: &str,
    loan_ref: &str,
    policy_refs: &[&str],
    vendor_rule_refs: &[&str],
) -> SyntheticVendorEvent {
    SyntheticVendorEvent {
        event_id: event_id.to_string(),
        event_type: event_type.to_string(),
        vendor_system: vendor_system.to_string(),
        workflow_stage: workflow_stage.to_string(),
        borrower_or_business_ref: Some(business_ref.to_string()),
        loan_or_account_ref: Some(loan_ref.to_string()),
        timestamp: "2026-07-03T12:00:00Z".to_string(),
        actor_type: actor_type.to_string(),
        input_payload_hash: format!("demo_input_hash_{}", event_id),
        output_payload_hash: format!("demo_output_hash_{}", event_id),
        policy_refs: policy_refs.iter().map(|s| s.to_string()).collect(),
        model_refs: vec![],
        vendor_rule_refs: vendor_rule_refs.iter().map(|s| s.to_string()).collect(),
        human_review_refs: vec![],
        exception_refs: vec![],
        consent_refs: if event_type == "CONSENT_CAPTURED" {
            vec![]
        } else {
            vec!["receipt_apiture_consent_001".to_string()]
        },
        source_system_refs: vec![format!("synthetic_{}", normalize_ref(vendor_system))],
    }
}

fn normalize_ref(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

fn write_or_print(out: Option<&str>, contents: &str, label: &str) -> std::io::Result<()> {
    if let Some(out_path) = out {
        write_text(out_path, contents)?;
        println!("Wrote {} to {}", label, out_path);
    } else {
        println!("{}", contents);
    }
    Ok(())
}

fn write_text(path: impl AsRef<Path>, contents: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    if contents.ends_with('\n') {
        fs::write(path, contents)
    } else {
        fs::write(path, format!("{}\n", contents))
    }
}

fn print_pretty_report(report: &bil_verify::VerificationReport) {
    println!("Verification Report:");
    println!("  Status: {:?}", report.status);
    println!("  Receipt ID: {:?}", report.receipt_id);
    println!("  Profile: {:?}", report.profile);
    println!("  Checks:");
    for check in &report.checks {
        println!("    - {:?}: {:?}", check.kind, check.status);
    }
    if !report.findings.is_empty() {
        println!("  Findings:");
        for finding in &report.findings {
            println!("    - [{:?}] {}", finding.priority, finding.message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sba_trace_normalizes_to_issueable_mir_graph() {
        let trace = default_sba_express_trace(2026);
        let graph = trace_to_mir(&trace);

        assert_eq!(graph.profile.0, "sba_express_evidence");
        assert_eq!(graph.events.len(), trace.events.len());
        assert_eq!(graph.evidence.len(), trace.events.len());
        assert!(!graph.authorities.is_empty());
        assert!(!graph.policies.is_empty());

        let artifact = bil_sdk::Bil::issue(
            graph,
            bil_ink::CapabilityCode("assurance-receipt".to_string()),
        )
        .unwrap();
        let report = bil_sdk::Bil::verify(&artifact.receipt).unwrap();

        assert_ne!(report.status, bil_core::BilStatus::Fail);
    }

    #[test]
    fn issue_input_parser_accepts_sba_trace() {
        let trace = default_sba_express_trace(2026);
        let json = serde_json::to_string(&trace).unwrap();
        let graph = parse_issue_input(&json).unwrap();

        assert_eq!(graph.graph_id.0, "sba-express-evidence-2026");
    }

    #[test]
    fn issue_input_parser_accepts_mir_graph() {
        let trace = default_sba_express_trace(2026);
        let graph = trace_to_mir(&trace);
        let json = serde_json::to_string(&graph).unwrap();
        let parsed = parse_issue_input(&json).unwrap();

        assert_eq!(parsed.graph_id.0, graph.graph_id.0);
    }
}
