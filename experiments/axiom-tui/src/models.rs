use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use serde::{Deserialize, Serialize};

use crate::{
    fixtures::{
        default_loss_summary, default_quote_state, default_reserve_snapshot, default_scenario_id,
        default_scenario_title, default_triage_snapshot, seed_audit_rows, seed_bias_rows,
        seed_compliance_rows, seed_loss_rows, seed_models, seed_risks, seed_submissions, ASSIGNEES,
        EXPORT_AUDIENCES, REFERRAL_REASONS, REFERRAL_TARGETS, SIM_ITERATIONS, URGENCIES,
    },
    theme::{color_serde, C_AMBER, C_CYAN, C_GREEN, C_MUTED},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Panel {
    Dashboard,
    RiskRegistry,
    Submissions,
    PolicyConfig,
    QuoteEngine,
    LossModels,
    ExposureAnalysis,
    Reserves,
    Retrospective,
    ModelRegistry,
    BiasMonitor,
    DriftDetection,
    Explainability,
    AuditTrail,
    Compliance,
    Reporting,
    Config,
    LiveLogs,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum ViewId {
    Dashboard,
    RiskRegistry,
    SubmissionsQueue,
    SubmissionsForm,
    SubmissionsTriage,
    PolicyConfig,
    QuoteEngine,
    LossModelsTriangles,
    LossModelsTrend,
    ExposureAnalysis,
    Reserves,
    Retrospective,
    ModelRegistryModels,
    ModelRegistryGates,
    BiasMonitor,
    DriftDetection,
    Explainability,
    AuditTrail,
    Compliance,
    Reporting,
    Config,
    LiveLogs,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum UiMode {
    Normal,
    Help,
    Filter,
    Drawer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ActionKey {
    Inspect,
    Assign,
    Refer,
    Export,
    Simulate,
    Filter,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlashTone {
    Info,
    Success,
    Warning,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DetailTarget {
    Risk(usize),
    Submission(usize),
    Loss(usize),
    Model(usize),
    Bias(usize),
    Audit(usize),
    Compliance(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AssignContext {
    Submission(usize),
    Quote,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ReferContext {
    Submission(usize),
    Quote,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SimulateTarget {
    Reserves,
    LossModels,
    Triage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DrawerKind {
    Detail(DetailTarget),
    Assign {
        context: AssignContext,
        assignee: usize,
        urgency: usize,
        button: usize,
    },
    Refer {
        context: ReferContext,
        reason: usize,
        target: usize,
        button: usize,
    },
    Export {
        view: ViewId,
        format: usize,
        audience: usize,
        button: usize,
    },
    Simulate {
        target: SimulateTarget,
        scenario: usize,
        iterations: usize,
        button: usize,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrawerState {
    pub kind: DrawerKind,
    pub focus: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilterState {
    pub view: ViewId,
    pub input: String,
}

#[derive(Clone, Debug)]
pub struct FlashMessage {
    pub text: String,
    pub tone: FlashTone,
    pub ticks_left: u8,
}

#[derive(Clone, Copy)]
pub struct NavEntry {
    pub label: &'static str,
    pub panel: Panel,
    pub badge: Option<(&'static str, Color)>,
}

#[derive(Clone, Debug)]
pub struct LogLine {
    pub ts: String,
    pub level: char,
    pub source: String,
    pub msg: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskRecord {
    pub id: String,
    pub category: String,
    #[serde(with = "color_serde")]
    pub category_color: Color,
    pub description: String,
    pub frequency: String,
    pub severity: String,
    #[serde(with = "color_serde")]
    pub severity_color: Color,
    pub mitigation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmissionRecord {
    pub ref_id: String,
    pub insured: String,
    pub line: String,
    pub limit: String,
    pub risk_band: String,
    pub risk_score: f64,
    #[serde(with = "color_serde")]
    pub score_color: Color,
    pub flag: String,
    #[serde(with = "color_serde")]
    pub flag_color: Color,
    pub status: String,
    #[serde(with = "color_serde")]
    pub status_color: Color,
    pub owner: String,
    pub urgency: String,
    pub triage_note: String,
    pub referred_to: Option<String>,
    pub referral_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LossRecord {
    pub ay: String,
    pub m12: String,
    pub m24: String,
    pub m36: String,
    pub m48: String,
    pub m60: String,
    pub ultimate: String,
    #[serde(with = "color_serde")]
    pub ultimate_color: Color,
    pub method: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelRecord {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub version: String,
    pub drift: String,
    #[serde(with = "color_serde")]
    pub drift_color: Color,
    pub bias: String,
    #[serde(with = "color_serde")]
    pub bias_color: Color,
    pub explainability: String,
    #[serde(with = "color_serde")]
    pub explainability_color: Color,
    pub status: String,
    #[serde(with = "color_serde")]
    pub status_color: Color,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiasRecord {
    pub attribute: String,
    pub group: String,
    pub approval_rate: String,
    pub di_ratio: String,
    pub fpr: String,
    pub fnr: String,
    pub status: String,
    #[serde(with = "color_serde")]
    pub status_color: Color,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditRecord {
    pub ts: String,
    pub actor: String,
    pub action: String,
    pub model: String,
    pub outcome: String,
    #[serde(with = "color_serde")]
    pub outcome_color: Color,
    pub hash: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceRecord {
    pub framework: String,
    pub scope: String,
    pub last_audit: String,
    pub findings: String,
    #[serde(with = "color_serde")]
    pub findings_color: Color,
    pub status: String,
    #[serde(with = "color_serde")]
    pub status_color: Color,
    pub next_step: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuoteEngineState {
    pub owner: String,
    pub urgency: String,
    pub routing_status: String,
    pub referral_target: Option<String>,
    pub referral_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReserveSnapshot {
    pub scenario: String,
    pub iterations: usize,
    pub case_reserves: u32,
    pub selected_ibnr: u32,
    pub ulae: u32,
    pub total_reserves: u32,
    pub confidence_pct: u16,
    pub cov: f64,
    pub risk_margin: u32,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LossSummary {
    pub scenario: String,
    pub iterations: usize,
    pub expected_loss_ratio_pct: f64,
    pub ultimate_loss: u32,
    pub tail_factor: f64,
    pub severity_trend_pct: f64,
    pub frequency_trend_pct: f64,
    pub pure_premium_trend_pct: f64,
    pub note: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriageSnapshot {
    pub scenario: String,
    pub iterations: usize,
    pub model_score: f64,
    pub data_score: f64,
    pub fairness_score: f64,
    pub explainability_score: f64,
    pub operational_score: f64,
    pub composite_score: f64,
    pub composite_label: String,
    pub recommendation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportReceipt {
    pub view: ViewId,
    pub format: String,
    pub audience: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scenario {
    #[serde(default = "default_scenario_id")]
    pub scenario_id: String,
    #[serde(default = "default_scenario_title")]
    pub title: String,
    #[serde(default = "seed_risks")]
    pub risks: Vec<RiskRecord>,
    #[serde(default = "seed_submissions")]
    pub submissions: Vec<SubmissionRecord>,
    #[serde(default = "seed_loss_rows")]
    pub loss_rows: Vec<LossRecord>,
    #[serde(default = "seed_models")]
    pub models: Vec<ModelRecord>,
    #[serde(default = "seed_bias_rows")]
    pub bias_rows: Vec<BiasRecord>,
    #[serde(default = "seed_audit_rows")]
    pub audit_rows: Vec<AuditRecord>,
    #[serde(default = "seed_compliance_rows")]
    pub compliance_rows: Vec<ComplianceRecord>,
    #[serde(default = "default_quote_state")]
    pub quote: QuoteEngineState,
    #[serde(default = "default_reserve_snapshot")]
    pub reserves: ReserveSnapshot,
    #[serde(default = "default_loss_summary")]
    pub loss_summary: LossSummary,
    #[serde(default = "default_triage_snapshot")]
    pub triage: TriageSnapshot,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            view: ViewId::RiskRegistry,
            input: String::new(),
        }
    }
}

impl ActionKey {
    pub fn key_label(self) -> &'static str {
        match self {
            Self::Inspect => "Enter",
            Self::Assign => "a",
            Self::Refer => "r",
            Self::Export => "e",
            Self::Simulate => "s",
            Self::Filter => "/",
        }
    }

    pub fn short_hint(self) -> &'static str {
        match self {
            Self::Inspect => "detail",
            Self::Assign => "assign",
            Self::Refer => "refer",
            Self::Export => "export",
            Self::Simulate => "simulate",
            Self::Filter => "filter",
        }
    }

    pub fn help_text(self) -> &'static str {
        match self {
            Self::Inspect => "Open row detail drawer",
            Self::Assign => "Assign workflow",
            Self::Refer => "Refer / escalate",
            Self::Export => "Export pack",
            Self::Simulate => "Run scenario simulation",
            Self::Filter => "Filter visible rows",
        }
    }
}

impl FlashTone {
    pub fn color(self) -> Color {
        match self {
            Self::Info => C_CYAN,
            Self::Success => C_GREEN,
            Self::Warning => C_AMBER,
        }
    }
}

impl RiskRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.id, self.category, self.description, self.frequency, self.mitigation
        )
    }
}

impl SubmissionRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {}",
            self.ref_id,
            self.insured,
            self.line,
            self.limit,
            self.risk_band,
            self.flag,
            self.status,
            self.owner
        )
    }

    pub fn score_text(&self) -> String {
        format!("{} {:.2}", self.risk_band, self.risk_score)
    }
}

impl LossRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {}",
            self.ay, self.m12, self.m24, self.m36, self.m48, self.m60, self.ultimate, self.method
        )
    }
}

impl ModelRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {}",
            self.id,
            self.name,
            self.purpose,
            self.version,
            self.drift,
            self.bias,
            self.explainability,
            self.status
        )
    }
}

impl BiasRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {}",
            self.attribute,
            self.group,
            self.approval_rate,
            self.di_ratio,
            self.fpr,
            self.fnr,
            self.status
        )
    }
}

impl AuditRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.ts, self.actor, self.action, self.model, self.outcome, self.hash
        )
    }
}

impl ComplianceRecord {
    pub fn search_text(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.framework, self.scope, self.last_audit, self.findings, self.status, self.next_step
        )
    }
}

impl DrawerState {
    pub fn detail(target: DetailTarget) -> Self {
        Self {
            kind: DrawerKind::Detail(target),
            focus: 0,
        }
    }

    pub fn assign(context: AssignContext) -> Self {
        Self {
            kind: DrawerKind::Assign {
                context,
                assignee: 0,
                urgency: 0,
                button: 0,
            },
            focus: 0,
        }
    }

    pub fn refer(context: ReferContext) -> Self {
        Self {
            kind: DrawerKind::Refer {
                context,
                reason: 0,
                target: 0,
                button: 0,
            },
            focus: 0,
        }
    }

    pub fn export(view: ViewId) -> Self {
        Self {
            kind: DrawerKind::Export {
                view,
                format: 0,
                audience: 0,
                button: 0,
            },
            focus: 0,
        }
    }

    pub fn simulate(target: SimulateTarget) -> Self {
        Self {
            kind: DrawerKind::Simulate {
                target,
                scenario: 0,
                iterations: 0,
                button: 0,
            },
            focus: 0,
        }
    }

    pub fn field_count(&self) -> usize {
        match self.kind {
            DrawerKind::Detail(_) => 1,
            _ => 3,
        }
    }

    pub fn is_detail(&self) -> bool {
        matches!(self.kind, DrawerKind::Detail(_))
    }

    pub fn focus_next(&mut self) {
        self.focus = (self.focus + 1) % self.field_count();
    }

    pub fn focus_prev(&mut self) {
        self.focus = if self.focus == 0 {
            self.field_count() - 1
        } else {
            self.focus - 1
        };
    }

    pub fn adjust(&mut self, delta: i32) {
        match &mut self.kind {
            DrawerKind::Detail(_) => {}
            DrawerKind::Assign {
                assignee,
                urgency,
                button,
                ..
            } => match self.focus {
                0 => *assignee = cycle_index(*assignee, ASSIGNEES.len(), delta),
                1 => *urgency = cycle_index(*urgency, URGENCIES.len(), delta),
                _ => *button = cycle_index(*button, 2, delta),
            },
            DrawerKind::Refer {
                reason,
                target,
                button,
                ..
            } => match self.focus {
                0 => *reason = cycle_index(*reason, REFERRAL_REASONS.len(), delta),
                1 => *target = cycle_index(*target, REFERRAL_TARGETS.len(), delta),
                _ => *button = cycle_index(*button, 2, delta),
            },
            DrawerKind::Export {
                view,
                format,
                audience,
                button,
            } => match self.focus {
                0 => *format = cycle_index(*format, export_formats(*view).len(), delta),
                1 => *audience = cycle_index(*audience, EXPORT_AUDIENCES.len(), delta),
                _ => *button = cycle_index(*button, 2, delta),
            },
            DrawerKind::Simulate {
                target,
                scenario,
                iterations,
                button,
            } => match self.focus {
                0 => *scenario = cycle_index(*scenario, simulation_scenarios(*target).len(), delta),
                1 => *iterations = cycle_index(*iterations, SIM_ITERATIONS.len(), delta),
                _ => *button = cycle_index(*button, 2, delta),
            },
        }
    }
}

impl SimulateTarget {
    pub fn label(self) -> &'static str {
        match self {
            Self::Reserves => "Reserves",
            Self::LossModels => "Loss Models",
            Self::Triage => "AI Triage",
        }
    }
}

pub fn panel_order() -> &'static [Panel] {
    &[
        Panel::Dashboard,
        Panel::RiskRegistry,
        Panel::Submissions,
        Panel::PolicyConfig,
        Panel::QuoteEngine,
        Panel::LossModels,
        Panel::ExposureAnalysis,
        Panel::Reserves,
        Panel::Retrospective,
        Panel::ModelRegistry,
        Panel::BiasMonitor,
        Panel::DriftDetection,
        Panel::Explainability,
        Panel::AuditTrail,
        Panel::Compliance,
        Panel::Reporting,
        Panel::Config,
        Panel::LiveLogs,
    ]
}

pub fn view_name(view: ViewId) -> &'static str {
    match view {
        ViewId::Dashboard => "Dashboard",
        ViewId::RiskRegistry => "Risk Registry",
        ViewId::SubmissionsQueue => "Submissions / Queue",
        ViewId::SubmissionsForm => "Submissions / New Submission",
        ViewId::SubmissionsTriage => "Submissions / AI Triage",
        ViewId::PolicyConfig => "Policy Config",
        ViewId::QuoteEngine => "Quote Engine",
        ViewId::LossModelsTriangles => "Loss Models / Triangles",
        ViewId::LossModelsTrend => "Loss Models / Trend",
        ViewId::ExposureAnalysis => "Exposure Analysis",
        ViewId::Reserves => "Reserves",
        ViewId::Retrospective => "Retrospective",
        ViewId::ModelRegistryModels => "Model Registry / Models",
        ViewId::ModelRegistryGates => "Model Registry / Governance Gates",
        ViewId::BiasMonitor => "Bias Monitor",
        ViewId::DriftDetection => "Drift Detection",
        ViewId::Explainability => "Explainability",
        ViewId::AuditTrail => "Audit Trail",
        ViewId::Compliance => "Compliance",
        ViewId::Reporting => "Reporting",
        ViewId::Config => "Config",
        ViewId::LiveLogs => "Live Logs",
    }
}

pub fn supports_rows(view: ViewId) -> bool {
    matches!(
        view,
        ViewId::RiskRegistry
            | ViewId::SubmissionsQueue
            | ViewId::LossModelsTriangles
            | ViewId::ModelRegistryModels
            | ViewId::BiasMonitor
            | ViewId::AuditTrail
            | ViewId::Compliance
    )
}

pub fn available_actions(view: ViewId) -> Vec<ActionKey> {
    match view {
        ViewId::RiskRegistry => vec![ActionKey::Inspect, ActionKey::Filter],
        ViewId::SubmissionsQueue => vec![
            ActionKey::Inspect,
            ActionKey::Assign,
            ActionKey::Refer,
            ActionKey::Filter,
        ],
        ViewId::SubmissionsTriage => vec![ActionKey::Simulate],
        ViewId::QuoteEngine => vec![ActionKey::Assign, ActionKey::Refer],
        ViewId::LossModelsTriangles => {
            vec![ActionKey::Inspect, ActionKey::Simulate, ActionKey::Filter]
        }
        ViewId::LossModelsTrend => vec![ActionKey::Simulate],
        ViewId::Reserves => vec![ActionKey::Simulate],
        ViewId::ModelRegistryModels => vec![ActionKey::Inspect, ActionKey::Filter],
        ViewId::BiasMonitor => vec![ActionKey::Inspect, ActionKey::Filter],
        ViewId::AuditTrail => vec![ActionKey::Inspect, ActionKey::Export, ActionKey::Filter],
        ViewId::Compliance => vec![ActionKey::Inspect, ActionKey::Export, ActionKey::Filter],
        ViewId::Reporting => vec![ActionKey::Export],
        _ => Vec::new(),
    }
}

pub fn tab_count(panel: Panel) -> usize {
    match panel {
        Panel::Submissions | Panel::LossModels | Panel::ModelRegistry => {
            2 + usize::from(matches!(panel, Panel::Submissions))
        }
        _ => 1,
    }
}

pub fn export_formats(view: ViewId) -> &'static [&'static str] {
    match view {
        ViewId::AuditTrail => &[
            "Audit packet",
            "Signed decision digest",
            "XBRL evidence map",
        ],
        ViewId::Compliance => &["Compliance pack", "Gap summary", "XBRL appendix"],
        ViewId::Reporting => &["XBRL filing", "Board pack", "Actuarial opinion"],
        _ => &["Local export"],
    }
}

pub fn simulation_scenarios(target: SimulateTarget) -> &'static [&'static str] {
    match target {
        SimulateTarget::Reserves => &[
            "Baseline reserve refresh",
            "Adverse large-loss shock",
            "Favorable close-out",
        ],
        SimulateTarget::LossModels => &["Baseline trend", "Stress trend", "Faster settlement"],
        SimulateTarget::Triage => &[
            "Portfolio baseline",
            "Bias remediation",
            "High-alert escalation",
        ],
    }
}

pub fn filtered_indices<T, F>(items: &[T], query: &str, mut search_text: F) -> Vec<usize>
where
    F: FnMut(&T) -> String,
{
    let needle = query.trim().to_ascii_lowercase();
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            if needle.is_empty() || matches_filter(&search_text(item), &needle) {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

pub fn matches_filter(text: &str, lower_needle: &str) -> bool {
    text.to_ascii_lowercase()
        .contains(&lower_needle.to_ascii_lowercase())
}

pub fn cycle_index(current: usize, len: usize, delta: i32) -> usize {
    if len == 0 {
        return 0;
    }
    let len = len as i32;
    let current = current as i32;
    (((current + delta) % len) + len) as usize % len as usize
}

pub fn owned_line(label: &str, value: &str, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<18}: "), Style::default().fg(C_MUTED)),
        Span::styled(value.to_string(), Style::default().fg(color)),
    ])
}
