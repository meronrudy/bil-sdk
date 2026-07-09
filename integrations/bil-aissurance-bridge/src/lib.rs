use anyhow::{anyhow, Context, Result};
use bil_canonical::Hash256;
use bil_core::{
    ActorRef, AuthorityRef, BilId, EventId, EvidenceRef, PolicyRef, ProfileId, SystemRef,
};
use bil_ink::CapabilityCode;
use bil_mir::{
    AuthorityEdge, AuthorityNode, BilEvent, BilEventKind, BilMirGraph, EvidenceRefNode, PolicyNode,
    ReplayState,
};
use bil_sdk::Bil;
use bil_verify::VerificationReport;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const AISSURANCE_PROFILE: &str = "aissurance_platform_alpha_v1";
pub const PLANNER_CAPABILITY: &str = "aissurance.platform.planner.v1";
pub const SAFETY_CAPABILITY: &str = "aissurance.platform.safety.v1";
pub const RISK_CAPABILITY: &str = "aissurance.platform.risk.v1";
pub const RUN_CAPABILITY: &str = "aissurance.platform.aggregate.v1";

const ALPHA_VALID_FROM_MS: i64 = 1_783_123_200_000;

#[derive(Debug, Clone)]
pub struct AissurancePlatformDemoOptions {
    pub config_path: Option<PathBuf>,
    pub data_dir: PathBuf,
    pub out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptArtifactRef {
    pub graph_path: PathBuf,
    pub receipt_path: PathBuf,
    pub receipt_id: String,
    pub verification_report_path: PathBuf,
    pub evidence_manifest_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingArtifacts {
    pub input_path: PathBuf,
    pub report_path: PathBuf,
    pub filing_path: PathBuf,
    pub job_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRunManifest {
    pub run_id: String,
    pub profile: String,
    pub job_id: String,
    pub output_dir: PathBuf,
    pub planner_receipt: ReceiptArtifactRef,
    pub safety_receipt: ReceiptArtifactRef,
    pub risk_receipt: ReceiptArtifactRef,
    pub run_receipt: ReceiptArtifactRef,
    pub underlying_artifacts: UnderlyingArtifacts,
    pub run_context_path: PathBuf,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceManifest {
    pub stage: String,
    pub items: Vec<EvidenceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub id: String,
    pub kind: String,
    pub hash_hex: String,
    pub payload_path: PathBuf,
    pub source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RunContextManifest {
    run_id: String,
    profile: String,
    job_id: String,
    underlying_artifacts: UnderlyingArtifacts,
    planner_receipt_path: PathBuf,
    safety_receipt_path: PathBuf,
    risk_receipt_path: PathBuf,
}

#[derive(Debug, Clone)]
struct StageAuthority {
    id: &'static str,
    actor: &'static str,
    system: &'static str,
    role: &'static str,
    scope: &'static str,
    policy: &'static str,
}

#[derive(Debug)]
struct GeneratedReceipt {
    artifact: ReceiptArtifactRef,
}

pub fn run_platform_demo_and_issue(
    options: AissurancePlatformDemoOptions,
) -> Result<PlatformRunManifest> {
    let result = aissurance_cli::run_platform_demo(options.config_path, options.data_dir)
        .context("failed to run AiSSURANCE platform demo")?;
    let proposal = result
        .planner
        .proposal
        .clone()
        .ok_or_else(|| anyhow!("planner returned no proposal"))?;
    let run_id = hex::encode(proposal.proposal_id.as_bytes());
    let output_dir = options.out_dir.join(&run_id);
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create {}", output_dir.display()))?;

    let planner_input_path =
        write_json_file(output_dir.join("planner/input.json"), &result.planner_input)?;
    let planner_output_path =
        write_json_file(output_dir.join("planner/output.json"), &result.planner)?;
    let planner_evidence = vec![
        evidence_item(
            "planner_input",
            "aissurance.planner.input.v1",
            &planner_input_path,
            None,
        )?,
        evidence_item(
            "planner_output",
            "aissurance.planner.output.v1",
            &planner_output_path,
            None,
        )?,
    ];
    let planner_graph = build_stage_graph(
        &run_id,
        "planner",
        &[
            BilEventKind::WorkflowStarted,
            BilEventKind::ModelInvoked,
            BilEventKind::DecisionIssued,
        ],
        &planner_evidence,
        &[
            file_hash(&planner_input_path)?,
            file_hash(&planner_output_path)?,
        ],
        StageAuthority {
            id: "auth_aissurance_planner_service",
            actor: "actor_aissurance_planner_service",
            system: "system_aissurance_vla_layer",
            role: "planner",
            scope: "platform_demo",
            policy: "POLICY_AISSURANCE_PLANNER_ALPHA_V1",
        },
    );
    let planner_receipt = issue_stage_receipt(
        output_dir.join("planner"),
        "planner",
        PLANNER_CAPABILITY,
        &planner_graph,
        &planner_evidence,
    )?;

    let safety_proposal_path = write_json_file(output_dir.join("safety/proposal.json"), &proposal)?;
    let safety_replay_input_path = write_json_file(
        output_dir.join("safety/replay_input.json"),
        &result.replay_steps,
    )?;
    let safety_report_path =
        write_json_file(output_dir.join("safety/report.json"), &result.safety)?;
    let safety_evidence = vec![
        evidence_item(
            "safety_proposal",
            "aissurance.safety.proposal.v1",
            &safety_proposal_path,
            None,
        )?,
        evidence_item(
            "safety_replay_input",
            "aissurance.safety.replay_input.v1",
            &safety_replay_input_path,
            None,
        )?,
        evidence_item(
            "safety_report",
            "aissurance.safety.report.v1",
            &safety_report_path,
            None,
        )?,
    ];
    let safety_graph = build_stage_graph(
        &run_id,
        "safety",
        &[
            BilEventKind::PolicyEvaluated,
            BilEventKind::DecisionIssued,
            BilEventKind::AuditBundleExported,
        ],
        &safety_evidence,
        &[
            file_hash(&safety_proposal_path)?,
            file_hash(&safety_replay_input_path)?,
            file_hash(&safety_report_path)?,
        ],
        StageAuthority {
            id: "auth_aissurance_safety_service",
            actor: "actor_aissurance_safety_service",
            system: "system_aissurance_safety_layer",
            role: "guardrail",
            scope: "platform_demo",
            policy: "POLICY_AISSURANCE_SAFETY_ALPHA_V1",
        },
    );
    let safety_receipt = issue_stage_receipt(
        output_dir.join("safety"),
        "safety",
        SAFETY_CAPABILITY,
        &safety_graph,
        &safety_evidence,
    )?;

    let job_path = result
        .artifacts
        .report_path
        .parent()
        .ok_or_else(|| anyhow!("missing report parent for job metadata"))?
        .join("job.json");
    let underlying_artifacts = UnderlyingArtifacts {
        input_path: result.artifacts.input_path.clone(),
        report_path: result.artifacts.report_path.clone(),
        filing_path: result.artifacts.filing_path.clone(),
        job_path: job_path.clone(),
    };
    let risk_evidence = vec![
        evidence_item(
            "risk_input",
            "aissurance.risk.input.v1",
            &underlying_artifacts.input_path,
            Some(underlying_artifacts.input_path.clone()),
        )?,
        evidence_item(
            "risk_report",
            "aissurance.risk.report.v1",
            &underlying_artifacts.report_path,
            Some(underlying_artifacts.report_path.clone()),
        )?,
        evidence_item(
            "risk_filing",
            "aissurance.risk.filing.v1",
            &underlying_artifacts.filing_path,
            Some(underlying_artifacts.filing_path.clone()),
        )?,
        evidence_item(
            "risk_job",
            "aissurance.risk.job.v1",
            &underlying_artifacts.job_path,
            Some(underlying_artifacts.job_path.clone()),
        )?,
    ];
    let risk_graph = build_stage_graph(
        &run_id,
        "risk",
        &[
            BilEventKind::DocumentReceived,
            BilEventKind::EvidenceExtracted,
            BilEventKind::ModelInvoked,
            BilEventKind::DecisionIssued,
            BilEventKind::AuditBundleExported,
        ],
        &risk_evidence,
        &[
            file_hash(&underlying_artifacts.input_path)?,
            file_hash(&underlying_artifacts.job_path)?,
            file_hash(&underlying_artifacts.report_path)?,
            file_hash(&underlying_artifacts.filing_path)?,
        ],
        StageAuthority {
            id: "auth_aissurance_risk_service",
            actor: "actor_aissurance_risk_service",
            system: "system_aissurance_control_plane",
            role: "risk_engine",
            scope: "platform_demo",
            policy: "POLICY_AISSURANCE_RISK_ALPHA_V1",
        },
    );
    let risk_receipt = issue_stage_receipt(
        output_dir.join("risk"),
        "risk",
        RISK_CAPABILITY,
        &risk_graph,
        &risk_evidence,
    )?;

    let run_context = RunContextManifest {
        run_id: run_id.clone(),
        profile: AISSURANCE_PROFILE.to_string(),
        job_id: result.job.job_id.clone(),
        underlying_artifacts: underlying_artifacts.clone(),
        planner_receipt_path: planner_receipt.artifact.receipt_path.clone(),
        safety_receipt_path: safety_receipt.artifact.receipt_path.clone(),
        risk_receipt_path: risk_receipt.artifact.receipt_path.clone(),
    };
    let run_context_path =
        write_json_file(output_dir.join("aggregate/run_context.json"), &run_context)?;
    let run_evidence = vec![
        evidence_item(
            "run_planner_receipt",
            "aissurance.aggregate.planner_receipt.v1",
            &planner_receipt.artifact.receipt_path,
            Some(planner_receipt.artifact.receipt_path.clone()),
        )?,
        evidence_item(
            "run_safety_receipt",
            "aissurance.aggregate.safety_receipt.v1",
            &safety_receipt.artifact.receipt_path,
            Some(safety_receipt.artifact.receipt_path.clone()),
        )?,
        evidence_item(
            "run_risk_receipt",
            "aissurance.aggregate.risk_receipt.v1",
            &risk_receipt.artifact.receipt_path,
            Some(risk_receipt.artifact.receipt_path.clone()),
        )?,
        evidence_item(
            "run_context",
            "aissurance.aggregate.run_context.v1",
            &run_context_path,
            None,
        )?,
    ];
    let run_graph = build_stage_graph(
        &run_id,
        "aggregate",
        &[
            BilEventKind::WorkflowStarted,
            BilEventKind::ModelInvoked,
            BilEventKind::PolicyEvaluated,
            BilEventKind::DecisionIssued,
            BilEventKind::AuditBundleExported,
        ],
        &run_evidence,
        &[
            file_hash(&planner_receipt.artifact.receipt_path)?,
            file_hash(&safety_receipt.artifact.receipt_path)?,
            file_hash(&risk_receipt.artifact.receipt_path)?,
            file_hash(&run_context_path)?,
        ],
        StageAuthority {
            id: "auth_aissurance_platform_service",
            actor: "actor_aissurance_platform_service",
            system: "system_aissurance_platform_demo",
            role: "orchestrator",
            scope: "platform_demo",
            policy: "POLICY_AISSURANCE_PLATFORM_AGGREGATE_ALPHA_V1",
        },
    );
    let run_receipt = issue_stage_receipt(
        output_dir.join("aggregate"),
        "aggregate",
        RUN_CAPABILITY,
        &run_graph,
        &run_evidence,
    )?;

    let manifest_path = output_dir.join("manifest.json");
    let manifest = PlatformRunManifest {
        run_id,
        profile: AISSURANCE_PROFILE.to_string(),
        job_id: result.job.job_id,
        output_dir: output_dir.clone(),
        planner_receipt: planner_receipt.artifact,
        safety_receipt: safety_receipt.artifact,
        risk_receipt: risk_receipt.artifact,
        run_receipt: run_receipt.artifact,
        underlying_artifacts,
        run_context_path,
        manifest_path: manifest_path.clone(),
    };
    write_json_file(&manifest_path, &manifest)?;
    Ok(manifest)
}

fn issue_stage_receipt(
    stage_dir: PathBuf,
    stage_name: &str,
    capability: &str,
    graph: &BilMirGraph,
    evidence: &[EvidenceItem],
) -> Result<GeneratedReceipt> {
    fs::create_dir_all(&stage_dir)
        .with_context(|| format!("failed to create {}", stage_dir.display()))?;
    let graph_path = write_json_file(stage_dir.join("graph.json"), graph)?;
    let evidence_manifest_path = write_json_file(
        stage_dir.join("evidence_manifest.json"),
        &EvidenceManifest {
            stage: stage_name.to_string(),
            items: evidence.to_vec(),
        },
    )?;
    let issued = Bil::issue(graph.clone(), CapabilityCode(capability.to_string()))
        .map_err(|err| anyhow!("failed to issue {} receipt: {}", stage_name, err))?;
    let receipt_path = write_json_file(stage_dir.join("receipt.json"), &issued.receipt)?;
    let verification = Bil::verify(&issued.receipt)
        .map_err(|err| anyhow!("failed to verify {} receipt: {}", stage_name, err))?;
    let verification_report_path =
        write_json_file(stage_dir.join("verification_report.json"), &verification)?;

    Ok(GeneratedReceipt {
        artifact: ReceiptArtifactRef {
            graph_path,
            receipt_path,
            receipt_id: issued.receipt.preimage.receipt_id.0.clone(),
            verification_report_path,
            evidence_manifest_path,
        },
    })
}

fn build_stage_graph(
    run_id: &str,
    stage: &str,
    event_kinds: &[BilEventKind],
    evidence: &[EvidenceItem],
    replay_hashes: &[Hash256],
    authority: StageAuthority,
) -> BilMirGraph {
    let policies = vec![PolicyNode {
        id: PolicyRef(authority.policy.to_string()),
    }];
    let events = event_kinds
        .iter()
        .enumerate()
        .map(|(index, kind)| BilEvent {
            id: EventId(format!(
                "evt_{}_{}_{}_{}",
                run_id,
                stage,
                index,
                debug_name(kind)
            )),
            kind: kind.clone(),
        })
        .collect();
    let evidence = evidence
        .iter()
        .map(|item| EvidenceRefNode {
            id: EvidenceRef(item.id.clone()),
            hash: Hash256::from_hex(&item.hash_hex).expect("evidence hashes are generated locally"),
            kind: Some(item.kind.clone()),
        })
        .collect();
    let authorities = vec![AuthorityNode {
        id: AuthorityRef(authority.id.to_string()),
        edge: AuthorityEdge {
            actor: ActorRef(authority.actor.to_string()),
            system: Some(SystemRef(authority.system.to_string())),
            role: authority.role.to_string(),
            scope: authority.scope.to_string(),
            policy_ref: policies[0].id.clone(),
            valid_from_ms: ALPHA_VALID_FROM_MS,
            valid_until_ms: None,
        },
    }];

    BilMirGraph {
        graph_id: BilId(run_id.to_string()),
        profile: ProfileId(AISSURANCE_PROFILE.to_string()),
        events,
        evidence,
        authorities,
        policies,
        replay: replay_state(replay_hashes),
    }
}

fn replay_state(hashes: &[Hash256]) -> ReplayState {
    let initial_state_hash = hashes.first().cloned().unwrap_or_else(Hash256::zero);
    let transition_hashes = hashes.iter().skip(1).cloned().collect::<Vec<_>>();
    let mut final_state_preimage = Vec::new();
    for hash in hashes {
        final_state_preimage.extend_from_slice(&hash.0);
    }

    ReplayState {
        initial_state_hash,
        transition_hashes,
        final_state_hash: Hash256::sha256(&final_state_preimage),
    }
}

fn evidence_item(
    id: &str,
    kind: &str,
    payload_path: &Path,
    source_path: Option<PathBuf>,
) -> Result<EvidenceItem> {
    Ok(EvidenceItem {
        id: format!("evd_{}", id),
        kind: kind.to_string(),
        hash_hex: file_hash(payload_path)?.to_hex(),
        payload_path: payload_path.to_path_buf(),
        source_path,
    })
}

fn file_hash(path: &Path) -> Result<Hash256> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(Hash256::sha256(&bytes))
}

fn write_json_file(path: impl AsRef<Path>, value: &impl Serialize) -> Result<PathBuf> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path.to_path_buf())
}

fn debug_name(kind: &BilEventKind) -> &'static str {
    match kind {
        BilEventKind::WorkflowStarted => "workflow_started",
        BilEventKind::ConsentCaptured => "consent_captured",
        BilEventKind::DocumentReceived => "document_received",
        BilEventKind::EvidenceExtracted => "evidence_extracted",
        BilEventKind::ModelInvoked => "model_invoked",
        BilEventKind::PolicyEvaluated => "policy_evaluated",
        BilEventKind::AuthorityBound => "authority_bound",
        BilEventKind::HumanReviewed => "human_reviewed",
        BilEventKind::DecisionIssued => "decision_issued",
        BilEventKind::AdverseActionReasoned => "adverse_action_reasoned",
        BilEventKind::AuditBundleExported => "audit_bundle_exported",
        BilEventKind::VendorRouteObserved => "vendor_route_observed",
    }
}

pub fn read_verification_report(path: &Path) -> Result<VerificationReport> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(serde_json::from_slice(&bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bil_core::BilStatus;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn run_bridge() -> PlatformRunManifest {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let data_dir = std::env::temp_dir().join(format!("bil-aissurance-data-{suffix}"));
        let out_dir = std::env::temp_dir().join(format!("bil-aissurance-out-{suffix}"));
        fs::create_dir_all(&data_dir).unwrap();
        fs::create_dir_all(&out_dir).unwrap();
        let manifest = run_platform_demo_and_issue(AissurancePlatformDemoOptions {
            config_path: None,
            data_dir,
            out_dir,
        })
        .unwrap();
        assert!(manifest.manifest_path.exists());
        manifest
    }

    #[test]
    fn graph_construction_uses_profile_and_expected_events() {
        let hashes = [Hash256::sha256(b"input"), Hash256::sha256(b"output")];
        let graph = build_stage_graph(
            "run-1",
            "planner",
            &[
                BilEventKind::WorkflowStarted,
                BilEventKind::ModelInvoked,
                BilEventKind::DecisionIssued,
            ],
            &[EvidenceItem {
                id: "evd_planner_input".to_string(),
                kind: "kind".to_string(),
                hash_hex: hashes[0].to_hex(),
                payload_path: PathBuf::from("payload.json"),
                source_path: None,
            }],
            &hashes,
            StageAuthority {
                id: "auth",
                actor: "actor",
                system: "system",
                role: "role",
                scope: "scope",
                policy: "POLICY_AISSURANCE_TEST_V1",
            },
        );

        assert_eq!(graph.profile.0, AISSURANCE_PROFILE);
        assert_eq!(graph.events.len(), 3);
        assert_eq!(graph.replay.initial_state_hash, hashes[0]);
        assert_eq!(graph.replay.transition_hashes, vec![hashes[1].clone()]);
    }

    #[test]
    fn emitted_receipts_pass_verification() {
        let manifest = run_bridge();
        for report_path in [
            &manifest.planner_receipt.verification_report_path,
            &manifest.safety_receipt.verification_report_path,
            &manifest.risk_receipt.verification_report_path,
            &manifest.run_receipt.verification_report_path,
        ] {
            let report = read_verification_report(report_path).unwrap();
            assert_eq!(report.status, BilStatus::Pass);
        }
    }

    #[test]
    fn manifest_keeps_common_run_id_and_job_id() {
        let manifest = run_bridge();
        assert!(!manifest.run_id.is_empty());
        assert!(!manifest.job_id.is_empty());

        let bytes = fs::read(&manifest.run_context_path).unwrap();
        let run_context: RunContextManifest = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(run_context.run_id, manifest.run_id);
        assert_eq!(run_context.job_id, manifest.job_id);
    }

    #[test]
    fn risk_evidence_points_to_persisted_aissurance_artifacts() {
        let manifest = run_bridge();
        let bytes = fs::read(&manifest.risk_receipt.evidence_manifest_path).unwrap();
        let evidence: EvidenceManifest = serde_json::from_slice(&bytes).unwrap();
        let sources = evidence
            .items
            .iter()
            .map(|item| item.source_path.clone().unwrap())
            .collect::<Vec<_>>();

        assert!(sources.contains(&manifest.underlying_artifacts.input_path));
        assert!(sources.contains(&manifest.underlying_artifacts.report_path));
        assert!(sources.contains(&manifest.underlying_artifacts.filing_path));
        assert!(sources.contains(&manifest.underlying_artifacts.job_path));
    }
}
