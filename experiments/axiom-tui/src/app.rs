use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

use chrono::Local;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::{
    exports::{build_export_payload, build_simulation_result, ExportPayload, SimulationResult},
    fixtures::{ASSIGNEES, LOG_POOL, REFERRAL_REASONS, REFERRAL_TARGETS, URGENCIES},
    models::{
        available_actions, filtered_indices, owned_line, panel_order, simulation_scenarios,
        supports_rows, tab_count, view_name, ActionKey, AssignContext, AuditRecord, BiasRecord,
        ComplianceRecord, DetailTarget, DrawerKind, DrawerState, ExportReceipt, FilterState,
        FlashMessage, FlashTone, LogLine, LossRecord, LossSummary, ModelRecord, Panel,
        QuoteEngineState, ReferContext, ReserveSnapshot, RiskRecord, Scenario, SimulateTarget,
        SubmissionRecord, TriageSnapshot, UiMode, ViewId,
    },
    scenario_loader,
    theme::{C_AMBER, C_BLUE, C_CYAN, C_DIM, C_MUTED, C_PURPLE, C_TEXT},
};

pub struct App {
    scenario_id: String,
    scenario_title: String,
    pub(crate) active_panel: Panel,
    pub(crate) tab_index: usize,
    pub(crate) nav_flat: Vec<Panel>,
    pub(crate) ui_mode: UiMode,
    pub(crate) filter_state: FilterState,
    pub(crate) drawer_state: Option<DrawerState>,
    pub(crate) flash: Option<FlashMessage>,
    pub(crate) logs: VecDeque<LogLine>,
    pub(crate) log_seed_index: usize,
    pub(crate) last_log_tick: Instant,
    pub(crate) last_clock_tick: Instant,
    pub(crate) clock_str: String,
    pub(crate) tick_count: u64,
    pub(crate) row_selections: HashMap<ViewId, usize>,
    pub(crate) committed_filters: HashMap<ViewId, String>,
    pub(crate) risks: Vec<RiskRecord>,
    pub(crate) submissions: Vec<SubmissionRecord>,
    pub(crate) loss_rows: Vec<LossRecord>,
    pub(crate) models: Vec<ModelRecord>,
    pub(crate) bias_rows: Vec<BiasRecord>,
    pub(crate) audit_rows: Vec<AuditRecord>,
    pub(crate) compliance_rows: Vec<ComplianceRecord>,
    pub(crate) quote: QuoteEngineState,
    pub(crate) reserves: ReserveSnapshot,
    pub(crate) loss_summary: LossSummary,
    pub(crate) triage: TriageSnapshot,
    pub(crate) last_export: Option<ExportReceipt>,
    pub(crate) export_payloads: Vec<ExportPayload>,
    pub(crate) simulation_results: Vec<SimulationResult>,
    pub should_quit: bool,
}

impl App {
    pub fn from_scenario(scenario: Scenario) -> Self {
        let mut app = Self {
            scenario_id: scenario.scenario_id,
            scenario_title: scenario.title,
            active_panel: Panel::Dashboard,
            tab_index: 0,
            nav_flat: panel_order().to_vec(),
            ui_mode: UiMode::Normal,
            filter_state: FilterState::default(),
            drawer_state: None,
            flash: None,
            logs: VecDeque::new(),
            log_seed_index: 0,
            last_log_tick: Instant::now(),
            last_clock_tick: Instant::now(),
            clock_str: String::new(),
            tick_count: 0,
            row_selections: HashMap::new(),
            committed_filters: HashMap::new(),
            risks: scenario.risks,
            submissions: scenario.submissions,
            loss_rows: scenario.loss_rows,
            models: scenario.models,
            bias_rows: scenario.bias_rows,
            audit_rows: scenario.audit_rows,
            compliance_rows: scenario.compliance_rows,
            quote: scenario.quote,
            reserves: scenario.reserves,
            loss_summary: scenario.loss_summary,
            triage: scenario.triage,
            last_export: None,
            export_payloads: Vec::new(),
            simulation_results: Vec::new(),
            should_quit: false,
        };

        app.update_clock();
        for _ in 0..6 {
            app.seed_log();
        }
        app.ensure_selection(app.active_view());
        app
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn scenario_title(&self) -> &str {
        &self.scenario_title
    }

    pub fn active_view(&self) -> ViewId {
        match self.active_panel {
            Panel::Dashboard => ViewId::Dashboard,
            Panel::RiskRegistry => ViewId::RiskRegistry,
            Panel::Submissions => match self.tab_index {
                0 => ViewId::SubmissionsQueue,
                1 => ViewId::SubmissionsForm,
                _ => ViewId::SubmissionsTriage,
            },
            Panel::PolicyConfig => ViewId::PolicyConfig,
            Panel::QuoteEngine => ViewId::QuoteEngine,
            Panel::LossModels => match self.tab_index {
                0 => ViewId::LossModelsTriangles,
                _ => ViewId::LossModelsTrend,
            },
            Panel::ExposureAnalysis => ViewId::ExposureAnalysis,
            Panel::Reserves => ViewId::Reserves,
            Panel::Retrospective => ViewId::Retrospective,
            Panel::ModelRegistry => match self.tab_index {
                0 => ViewId::ModelRegistryModels,
                _ => ViewId::ModelRegistryGates,
            },
            Panel::BiasMonitor => ViewId::BiasMonitor,
            Panel::DriftDetection => ViewId::DriftDetection,
            Panel::Explainability => ViewId::Explainability,
            Panel::AuditTrail => ViewId::AuditTrail,
            Panel::Compliance => ViewId::Compliance,
            Panel::Reporting => ViewId::Reporting,
            Panel::Config => ViewId::Config,
            Panel::LiveLogs => ViewId::LiveLogs,
        }
    }

    pub fn current_filter(&self, view: ViewId) -> &str {
        self.committed_filters
            .get(&view)
            .map(String::as_str)
            .unwrap_or("")
    }

    pub fn visible_indices(&self, view: ViewId) -> Vec<usize> {
        let query = self.current_filter(view);
        match view {
            ViewId::RiskRegistry => filtered_indices(&self.risks, query, RiskRecord::search_text),
            ViewId::SubmissionsQueue => {
                filtered_indices(&self.submissions, query, SubmissionRecord::search_text)
            }
            ViewId::LossModelsTriangles => {
                filtered_indices(&self.loss_rows, query, LossRecord::search_text)
            }
            ViewId::ModelRegistryModels => {
                filtered_indices(&self.models, query, ModelRecord::search_text)
            }
            ViewId::BiasMonitor => {
                filtered_indices(&self.bias_rows, query, BiasRecord::search_text)
            }
            ViewId::AuditTrail => {
                filtered_indices(&self.audit_rows, query, AuditRecord::search_text)
            }
            ViewId::Compliance => {
                filtered_indices(&self.compliance_rows, query, ComplianceRecord::search_text)
            }
            _ => Vec::new(),
        }
    }

    pub fn visible_row_count_for(&self, view: ViewId) -> usize {
        self.visible_indices(view).len()
    }

    pub fn visible_row_count(&self) -> usize {
        self.visible_row_count_for(self.active_view())
    }

    pub fn supports_rows(&self, view: ViewId) -> bool {
        supports_rows(view)
    }

    pub fn selected_visible_position(&self, view: ViewId) -> Option<usize> {
        if !supports_rows(view) {
            return None;
        }
        let count = self.visible_row_count_for(view);
        if count == 0 {
            None
        } else {
            Some(
                self.row_selections
                    .get(&view)
                    .copied()
                    .unwrap_or(0)
                    .min(count - 1),
            )
        }
    }

    pub fn hint_text_for(&self, view: ViewId) -> String {
        let mut parts = Vec::new();
        parts.push("J/K: panels".to_string());
        if supports_rows(view) && self.visible_row_count_for(view) > 0 {
            parts.push("j/k: rows".to_string());
        }
        if tab_count(self.active_panel) > 1 {
            parts.push("Tab/Shift+Tab: tabs".to_string());
        }
        for action in available_actions(view) {
            parts.push(format!("{}: {}", action.key_label(), action.short_hint()));
        }
        parts.push("?: help".to_string());
        parts.push("q: quit".to_string());
        let filter = self.current_filter(view);
        if !filter.is_empty() {
            parts.push(format!("filter={filter}"));
        }
        format!("  {}", parts.join("  "))
    }

    pub fn last_export_for(&self, view: ViewId) -> Option<&ExportReceipt> {
        self.last_export
            .as_ref()
            .filter(|receipt| receipt.view == view)
    }

    pub fn detail_title(&self, target: DetailTarget) -> String {
        match target {
            DetailTarget::Risk(idx) => format!("RISK DETAIL - {}", self.risks[idx].id),
            DetailTarget::Submission(idx) => {
                format!("SUBMISSION DETAIL - {}", self.submissions[idx].ref_id)
            }
            DetailTarget::Loss(idx) => format!("LOSS DETAIL - AY {}", self.loss_rows[idx].ay),
            DetailTarget::Model(idx) => format!("MODEL DETAIL - {}", self.models[idx].id),
            DetailTarget::Bias(idx) => {
                format!(
                    "BIAS DETAIL - {} {}",
                    self.bias_rows[idx].attribute, self.bias_rows[idx].group
                )
            }
            DetailTarget::Audit(idx) => format!("AUDIT DETAIL - {}", self.audit_rows[idx].hash),
            DetailTarget::Compliance(idx) => {
                format!(
                    "COMPLIANCE DETAIL - {}",
                    self.compliance_rows[idx].framework
                )
            }
        }
    }

    pub fn detail_lines(&self, target: DetailTarget) -> Vec<Line<'static>> {
        match target {
            DetailTarget::Risk(idx) => {
                let row = &self.risks[idx];
                vec![
                    Line::from(""),
                    owned_line("  Category", &row.category, row.category_color),
                    owned_line("  Severity", &row.severity, row.severity_color),
                    owned_line("  Frequency", &row.frequency, C_MUTED),
                    owned_line("  Description", &row.description, C_TEXT),
                    owned_line("  Mitigation", &row.mitigation, C_CYAN),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        "  Enter or Esc closes this drawer.",
                        Style::default().fg(C_DIM),
                    )]),
                ]
            }
            DetailTarget::Submission(idx) => {
                let row = &self.submissions[idx];
                let referral = row
                    .referred_to
                    .clone()
                    .unwrap_or_else(|| "None".to_string());
                let reason = row
                    .referral_reason
                    .clone()
                    .unwrap_or_else(|| "-".to_string());
                vec![
                    Line::from(""),
                    owned_line("  Insured", &row.insured, C_TEXT),
                    owned_line("  Line", &row.line, C_MUTED),
                    owned_line("  Limit", &row.limit, C_TEXT),
                    owned_line("  AI score", &row.score_text(), row.score_color),
                    owned_line("  Flag", &row.flag, row.flag_color),
                    owned_line("  Status", &row.status, row.status_color),
                    owned_line("  Owner", &row.owner, C_BLUE),
                    owned_line("  Urgency", &row.urgency, C_AMBER),
                    owned_line("  Referral target", &referral, C_PURPLE),
                    owned_line("  Referral reason", &reason, C_MUTED),
                    owned_line("  Triage note", &row.triage_note, C_DIM),
                ]
            }
            DetailTarget::Loss(idx) => {
                let row = &self.loss_rows[idx];
                vec![
                    Line::from(""),
                    owned_line("  Accident year", &row.ay, C_TEXT),
                    owned_line("  12 months", &row.m12, C_MUTED),
                    owned_line("  24 months", &row.m24, C_MUTED),
                    owned_line("  36 months", &row.m36, C_MUTED),
                    owned_line("  48 months", &row.m48, C_MUTED),
                    owned_line("  60 months", &row.m60, C_MUTED),
                    owned_line("  Ultimate", &row.ultimate, row.ultimate_color),
                    owned_line("  Method", &row.method, C_CYAN),
                    owned_line("  Current scenario", &self.loss_summary.scenario, C_BLUE),
                ]
            }
            DetailTarget::Model(idx) => {
                let row = &self.models[idx];
                vec![
                    Line::from(""),
                    owned_line("  Name", &row.name, C_TEXT),
                    owned_line("  Purpose", &row.purpose, C_MUTED),
                    owned_line("  Version", &row.version, C_DIM),
                    owned_line("  Drift", &row.drift, row.drift_color),
                    owned_line("  Bias", &row.bias, row.bias_color),
                    owned_line(
                        "  Explainability",
                        &row.explainability,
                        row.explainability_color,
                    ),
                    owned_line("  Status", &row.status, row.status_color),
                    owned_line("  Gate summary", "Validation 88% / Audit 95%", C_CYAN),
                ]
            }
            DetailTarget::Bias(idx) => {
                let row = &self.bias_rows[idx];
                vec![
                    Line::from(""),
                    owned_line("  Attribute", &row.attribute, C_TEXT),
                    owned_line("  Group", &row.group, C_MUTED),
                    owned_line("  Approval rate", &row.approval_rate, C_BLUE),
                    owned_line("  DI ratio", &row.di_ratio, C_AMBER),
                    owned_line("  False positive rate", &row.fpr, C_MUTED),
                    owned_line("  False negative rate", &row.fnr, C_MUTED),
                    owned_line("  Status", &row.status, row.status_color),
                    owned_line(
                        "  Response",
                        "Keep human review active for high-stakes cases",
                        C_CYAN,
                    ),
                ]
            }
            DetailTarget::Audit(idx) => {
                let row = &self.audit_rows[idx];
                vec![
                    Line::from(""),
                    owned_line("  Timestamp", &row.ts, C_MUTED),
                    owned_line("  Actor", &row.actor, C_TEXT),
                    owned_line("  Action", &row.action, C_BLUE),
                    owned_line("  Model", &row.model, C_MUTED),
                    owned_line("  Outcome", &row.outcome, row.outcome_color),
                    owned_line("  Signature hash", &row.hash, C_DIM),
                    owned_line("  Signed detail", &row.detail, C_CYAN),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        "  Use e to export the current signed packet.",
                        Style::default().fg(C_DIM),
                    )]),
                ]
            }
            DetailTarget::Compliance(idx) => {
                let row = &self.compliance_rows[idx];
                vec![
                    Line::from(""),
                    owned_line("  Framework", &row.framework, C_TEXT),
                    owned_line("  Scope", &row.scope, C_MUTED),
                    owned_line("  Last audit", &row.last_audit, C_MUTED),
                    owned_line("  Findings", &row.findings, row.findings_color),
                    owned_line("  Status", &row.status, row.status_color),
                    owned_line("  Next step", &row.next_step, C_CYAN),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        "  Use e to export the compliance pack.",
                        Style::default().fg(C_DIM),
                    )]),
                ]
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        if self.last_clock_tick.elapsed() >= Duration::from_secs(1) {
            self.update_clock();
            self.last_clock_tick = Instant::now();
        }
        if self.last_log_tick.elapsed() >= Duration::from_millis(1_200) {
            self.seed_log();
            self.last_log_tick = Instant::now();
        }
        if let Some(flash) = &mut self.flash {
            if flash.ticks_left == 0 {
                self.flash = None;
            } else {
                flash.ticks_left -= 1;
            }
        }
    }

    pub fn nav_down(&mut self) {
        let current = self
            .nav_flat
            .iter()
            .position(|panel| *panel == self.active_panel)
            .unwrap_or(0);
        let next = (current + 1) % self.nav_flat.len();
        self.active_panel = self.nav_flat[next];
        self.tab_index = 0;
        self.ensure_selection(self.active_view());
    }

    pub fn nav_up(&mut self) {
        let current = self
            .nav_flat
            .iter()
            .position(|panel| *panel == self.active_panel)
            .unwrap_or(0);
        let prev = if current == 0 {
            self.nav_flat.len() - 1
        } else {
            current - 1
        };
        self.active_panel = self.nav_flat[prev];
        self.tab_index = 0;
        self.ensure_selection(self.active_view());
    }

    pub fn move_row(&mut self, delta: i32) {
        let view = self.active_view();
        if !supports_rows(view) {
            return;
        }
        let count = self.visible_row_count_for(view);
        if count == 0 {
            return;
        }
        let current = self.row_selections.get(&view).copied().unwrap_or(0);
        self.row_selections
            .insert(view, crate::models::cycle_index(current, count, delta));
    }

    pub fn next_tab(&mut self) {
        let count = tab_count(self.active_panel);
        if count <= 1 {
            return;
        }
        self.tab_index = (self.tab_index + 1) % count;
        self.ensure_selection(self.active_view());
    }

    pub fn prev_tab(&mut self) {
        let count = tab_count(self.active_panel);
        if count <= 1 {
            return;
        }
        self.tab_index = if self.tab_index == 0 {
            count - 1
        } else {
            self.tab_index - 1
        };
        self.ensure_selection(self.active_view());
    }

    pub fn open_help(&mut self) {
        self.ui_mode = UiMode::Help;
    }

    pub fn close_help(&mut self) {
        self.ui_mode = UiMode::Normal;
    }

    pub fn open_drawer(&mut self, drawer: DrawerState) {
        self.drawer_state = Some(drawer);
        self.ui_mode = UiMode::Drawer;
    }

    pub fn close_drawer(&mut self) {
        self.drawer_state = None;
        self.ui_mode = UiMode::Normal;
    }

    pub fn start_filter(&mut self) {
        let view = self.active_view();
        self.filter_state = FilterState {
            view,
            input: self.current_filter(view).to_string(),
        };
        self.ui_mode = UiMode::Filter;
    }

    pub fn commit_filter(&mut self) {
        let view = self.filter_state.view;
        let query = self.filter_state.input.trim().to_string();
        if query.is_empty() {
            self.committed_filters.remove(&view);
            self.flash(
                FlashTone::Info,
                format!("Cleared filter for {}", view_name(view)),
            );
        } else {
            self.committed_filters.insert(view, query.clone());
            self.flash(
                FlashTone::Success,
                format!("Filtered {} by '{}'", view_name(view), query),
            );
        }
        self.ensure_selection(view);
        self.ui_mode = UiMode::Normal;
    }

    pub fn cancel_filter(&mut self) {
        self.ui_mode = UiMode::Normal;
        self.flash(FlashTone::Info, "Filter cancelled");
    }

    pub fn push_filter_char(&mut self, ch: char) {
        self.filter_state.input.push(ch);
    }

    pub fn pop_filter_char(&mut self) {
        self.filter_state.input.pop();
    }

    pub fn focus_next_in_drawer(&mut self) {
        if let Some(drawer) = &mut self.drawer_state {
            drawer.focus_next();
        }
    }

    pub fn focus_prev_in_drawer(&mut self) {
        if let Some(drawer) = &mut self.drawer_state {
            drawer.focus_prev();
        }
    }

    pub fn adjust_drawer(&mut self, delta: i32) {
        if let Some(drawer) = &mut self.drawer_state {
            drawer.adjust(delta);
        }
    }

    pub fn handle_action_request(&mut self, action: ActionKey) {
        let view = self.active_view();
        if !available_actions(view).contains(&action) {
            self.flash(
                FlashTone::Warning,
                format!(
                    "{} is not available on {}",
                    action.key_label(),
                    view_name(view)
                ),
            );
            return;
        }

        match action {
            ActionKey::Inspect => {
                if let Some(drawer) = self.inspect_drawer_for(view) {
                    self.open_drawer(drawer);
                } else {
                    self.flash(FlashTone::Warning, "No visible row to inspect");
                }
            }
            ActionKey::Assign => {
                let drawer = match view {
                    ViewId::SubmissionsQueue => self
                        .selected_row_index(view)
                        .map(AssignContext::Submission)
                        .map(DrawerState::assign),
                    ViewId::QuoteEngine => Some(DrawerState::assign(AssignContext::Quote)),
                    _ => None,
                };
                if let Some(drawer) = drawer {
                    self.open_drawer(drawer);
                }
            }
            ActionKey::Refer => {
                let drawer = match view {
                    ViewId::SubmissionsQueue => self
                        .selected_row_index(view)
                        .map(ReferContext::Submission)
                        .map(DrawerState::refer),
                    ViewId::QuoteEngine => Some(DrawerState::refer(ReferContext::Quote)),
                    _ => None,
                };
                if let Some(drawer) = drawer {
                    self.open_drawer(drawer);
                }
            }
            ActionKey::Export => self.open_drawer(DrawerState::export(view)),
            ActionKey::Simulate => {
                let target = match view {
                    ViewId::Reserves => Some(SimulateTarget::Reserves),
                    ViewId::LossModelsTriangles | ViewId::LossModelsTrend => {
                        Some(SimulateTarget::LossModels)
                    }
                    ViewId::SubmissionsTriage => Some(SimulateTarget::Triage),
                    _ => None,
                };
                if let Some(target) = target {
                    self.open_drawer(DrawerState::simulate(target));
                }
            }
            ActionKey::Filter => self.start_filter(),
        }
    }

    pub fn submit_drawer(&mut self) {
        let Some(drawer) = self.drawer_state.clone() else {
            return;
        };
        match drawer.kind {
            DrawerKind::Detail(_) => {
                self.close_drawer();
            }
            DrawerKind::Assign {
                context,
                assignee,
                urgency,
                button,
            } => {
                if button == 1 {
                    self.close_drawer();
                    self.flash(FlashTone::Info, "Assignment cancelled");
                    return;
                }
                self.apply_assign(context, assignee, urgency);
                self.close_drawer();
            }
            DrawerKind::Refer {
                context,
                reason,
                target,
                button,
            } => {
                if button == 1 {
                    self.close_drawer();
                    self.flash(FlashTone::Info, "Referral cancelled");
                    return;
                }
                self.apply_referral(context, reason, target);
                self.close_drawer();
            }
            DrawerKind::Export {
                view,
                format,
                audience,
                button,
            } => {
                if button == 1 {
                    self.close_drawer();
                    self.flash(FlashTone::Info, "Export cancelled");
                    return;
                }
                self.apply_export(view, format, audience);
                self.close_drawer();
            }
            DrawerKind::Simulate {
                target,
                scenario,
                iterations,
                button,
            } => {
                if button == 1 {
                    self.close_drawer();
                    self.flash(FlashTone::Info, "Simulation cancelled");
                    return;
                }
                self.apply_simulation(target, scenario, iterations);
                self.close_drawer();
            }
        }
    }

    pub fn set_should_quit(&mut self) {
        self.should_quit = true;
    }

    fn update_clock(&mut self) {
        self.clock_str = Local::now().format("%H:%M:%S  %Y-%m-%d").to_string();
    }

    fn flash(&mut self, tone: FlashTone, text: impl Into<String>) {
        self.flash = Some(FlashMessage {
            text: text.into(),
            tone,
            ticks_left: 40,
        });
    }

    fn push_log(&mut self, level: char, source: impl Into<String>, msg: impl Into<String>) {
        self.logs.push_back(LogLine {
            ts: Local::now().format("%H:%M:%S").to_string(),
            level,
            source: source.into(),
            msg: msg.into(),
        });
        if self.logs.len() > 200 {
            self.logs.pop_front();
        }
    }

    fn seed_log(&mut self) {
        let (level, source, msg) = LOG_POOL[self.log_seed_index % LOG_POOL.len()];
        self.log_seed_index += 1;
        self.push_log(level.chars().next().unwrap_or('I'), source, msg);
    }

    fn ensure_selection(&mut self, view: ViewId) {
        if !supports_rows(view) {
            return;
        }
        let count = self.visible_row_count_for(view);
        let selection = self.row_selections.entry(view).or_insert(0);
        if count == 0 {
            *selection = 0;
        } else if *selection >= count {
            *selection = count - 1;
        }
    }

    fn selected_row_index(&self, view: ViewId) -> Option<usize> {
        let indices = self.visible_indices(view);
        let pos = self.selected_visible_position(view)?;
        indices.get(pos).copied()
    }

    fn inspect_drawer_for(&self, view: ViewId) -> Option<DrawerState> {
        let target = match view {
            ViewId::RiskRegistry => self.selected_row_index(view).map(DetailTarget::Risk),
            ViewId::SubmissionsQueue => self.selected_row_index(view).map(DetailTarget::Submission),
            ViewId::LossModelsTriangles => self.selected_row_index(view).map(DetailTarget::Loss),
            ViewId::ModelRegistryModels => self.selected_row_index(view).map(DetailTarget::Model),
            ViewId::BiasMonitor => self.selected_row_index(view).map(DetailTarget::Bias),
            ViewId::AuditTrail => self.selected_row_index(view).map(DetailTarget::Audit),
            ViewId::Compliance => self.selected_row_index(view).map(DetailTarget::Compliance),
            _ => None,
        }?;
        Some(DrawerState::detail(target))
    }

    fn apply_assign(&mut self, context: AssignContext, assignee_idx: usize, urgency_idx: usize) {
        let assignee = ASSIGNEES[assignee_idx].to_string();
        let urgency = URGENCIES[urgency_idx].to_string();
        match context {
            AssignContext::Submission(idx) => {
                let submission = &mut self.submissions[idx];
                submission.owner = assignee.clone();
                submission.urgency = urgency.clone();
                submission.status = "Assigned".to_string();
                submission.status_color = C_BLUE;
                submission.referred_to = None;
                submission.referral_reason = None;
                let ref_id = submission.ref_id.clone();
                self.push_log(
                    'I',
                    "Assignments",
                    format!("{ref_id} assigned to {assignee} ({urgency})"),
                );
                self.flash(
                    FlashTone::Success,
                    format!("{ref_id} assigned to {assignee}"),
                );
            }
            AssignContext::Quote => {
                self.quote.owner = assignee.clone();
                self.quote.urgency = urgency.clone();
                self.quote.routing_status = "Assigned to quote owner".to_string();
                self.push_log(
                    'I',
                    "QuoteEngine",
                    format!("Quote routed to {assignee} ({urgency})"),
                );
                self.flash(FlashTone::Success, format!("Quote assigned to {assignee}"));
            }
        }
    }

    fn apply_referral(&mut self, context: ReferContext, reason_idx: usize, target_idx: usize) {
        let reason = REFERRAL_REASONS[reason_idx].to_string();
        let target = REFERRAL_TARGETS[target_idx].to_string();
        match context {
            ReferContext::Submission(idx) => {
                let submission = &mut self.submissions[idx];
                submission.status = "Referred".to_string();
                submission.status_color = C_AMBER;
                submission.referred_to = Some(target.clone());
                submission.referral_reason = Some(reason.clone());
                let ref_id = submission.ref_id.clone();
                self.push_log(
                    'W',
                    "Underwriting",
                    format!("{ref_id} referred to {target} ({reason})"),
                );
                self.flash(FlashTone::Success, format!("{ref_id} referred to {target}"));
            }
            ReferContext::Quote => {
                self.quote.routing_status = "Referred for sign-off".to_string();
                self.quote.referral_target = Some(target.clone());
                self.quote.referral_reason = Some(reason.clone());
                self.push_log(
                    'W',
                    "QuoteEngine",
                    format!("Quote referred to {target} ({reason})"),
                );
                self.flash(FlashTone::Success, format!("Quote referred to {target}"));
            }
        }
    }

    fn apply_export(&mut self, view: ViewId, format_idx: usize, audience_idx: usize) {
        let format = crate::models::export_formats(view)[format_idx].to_string();
        let audience = crate::fixtures::EXPORT_AUDIENCES[audience_idx].to_string();
        let payload = build_export_payload(self, view, &format, &audience);
        let created_at = payload.generated_at.clone();
        self.last_export = Some(ExportReceipt {
            view,
            format: format.clone(),
            audience: audience.clone(),
            created_at: created_at.clone(),
        });
        self.export_payloads.push(payload);
        self.push_log(
            'I',
            "Export",
            format!("{} exported as {format} for {audience}", view_name(view)),
        );
        self.flash(
            FlashTone::Success,
            format!("{} export ready: {format}", view_name(view)),
        );
    }

    fn apply_simulation(
        &mut self,
        target: SimulateTarget,
        scenario_idx: usize,
        iteration_idx: usize,
    ) {
        let iterations = crate::fixtures::SIM_ITERATIONS[iteration_idx];
        let scenario = simulation_scenarios(target)[scenario_idx].to_string();
        match target {
            SimulateTarget::Reserves => match scenario_idx {
                0 => {
                    self.reserves = ReserveSnapshot {
                        scenario: scenario.clone(),
                        iterations,
                        case_reserves: 840_000,
                        selected_ibnr: 600_000,
                        ulae: 86_400,
                        total_reserves: 1_526_400,
                        confidence_pct: 91,
                        cov: 0.18,
                        risk_margin: 152_640,
                        status: "ADEQUATE".to_string(),
                    };
                }
                1 => {
                    self.reserves = ReserveSnapshot {
                        scenario: scenario.clone(),
                        iterations,
                        case_reserves: 840_000,
                        selected_ibnr: 715_000,
                        ulae: 99_700,
                        total_reserves: 1_654_700,
                        confidence_pct: 95,
                        cov: 0.24,
                        risk_margin: 211_500,
                        status: "WATCH".to_string(),
                    };
                }
                _ => {
                    self.reserves = ReserveSnapshot {
                        scenario: scenario.clone(),
                        iterations,
                        case_reserves: 840_000,
                        selected_ibnr: 540_000,
                        ulae: 80_000,
                        total_reserves: 1_460_000,
                        confidence_pct: 88,
                        cov: 0.14,
                        risk_margin: 128_400,
                        status: "ADEQUATE".to_string(),
                    };
                }
            },
            SimulateTarget::LossModels => match scenario_idx {
                0 => {
                    self.loss_summary = LossSummary {
                        scenario: scenario.clone(),
                        iterations,
                        expected_loss_ratio_pct: 63.2,
                        ultimate_loss: 2_600_000,
                        tail_factor: 1.42,
                        severity_trend_pct: 8.4,
                        frequency_trend_pct: 4.1,
                        pure_premium_trend_pct: 12.9,
                        note: "AI liability trend is accelerating due to scaling LLM deployments and expanding regulatory exposure.".to_string(),
                    };
                }
                1 => {
                    self.loss_summary = LossSummary {
                        scenario: scenario.clone(),
                        iterations,
                        expected_loss_ratio_pct: 68.9,
                        ultimate_loss: 2_930_000,
                        tail_factor: 1.51,
                        severity_trend_pct: 10.6,
                        frequency_trend_pct: 5.3,
                        pure_premium_trend_pct: 16.5,
                        note: "Stress run assumes slower claim closure, adverse vendor concentration, and elevated severity tails.".to_string(),
                    };
                }
                _ => {
                    self.loss_summary = LossSummary {
                        scenario: scenario.clone(),
                        iterations,
                        expected_loss_ratio_pct: 60.8,
                        ultimate_loss: 2_410_000,
                        tail_factor: 1.36,
                        severity_trend_pct: 7.1,
                        frequency_trend_pct: 3.7,
                        pure_premium_trend_pct: 10.8,
                        note: "Faster settlement scenario assumes earlier intervention, tighter guardrails, and lower tail emergence.".to_string(),
                    };
                }
            },
            SimulateTarget::Triage => match scenario_idx {
                0 => {
                    self.triage = TriageSnapshot {
                        scenario: scenario.clone(),
                        iterations,
                        model_score: 0.72,
                        data_score: 0.61,
                        fairness_score: 0.44,
                        explainability_score: 0.88,
                        operational_score: 0.55,
                        composite_score: 0.87,
                        composite_label: "HIGH".to_string(),
                        recommendation: "Refer to Senior Underwriter".to_string(),
                    };
                }
                1 => {
                    self.triage = TriageSnapshot {
                        scenario: scenario.clone(),
                        iterations,
                        model_score: 0.64,
                        data_score: 0.68,
                        fairness_score: 0.58,
                        explainability_score: 0.89,
                        operational_score: 0.49,
                        composite_score: 0.74,
                        composite_label: "MED".to_string(),
                        recommendation: "Conditional review with fairness controls".to_string(),
                    };
                }
                _ => {
                    self.triage = TriageSnapshot {
                        scenario: scenario.clone(),
                        iterations,
                        model_score: 0.81,
                        data_score: 0.57,
                        fairness_score: 0.41,
                        explainability_score: 0.81,
                        operational_score: 0.63,
                        composite_score: 0.92,
                        composite_label: "HIGH".to_string(),
                        recommendation: "Immediate governance escalation".to_string(),
                    };
                }
            },
        }
        self.simulation_results
            .push(build_simulation_result(self, target, &scenario, iterations));
        self.push_log(
            'I',
            "Simulation",
            format!(
                "{} scenario '{}' completed with {} iterations",
                target.label(),
                scenario,
                iterations
            ),
        );
        self.flash(
            FlashTone::Success,
            format!("{} simulation complete", target.label()),
        );
    }
}

impl Default for App {
    fn default() -> Self {
        let scenario = scenario_loader::load_default_scenario()
            .expect("bundled default scenario should deserialize");
        Self::from_scenario(scenario)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_default_uses_default_scenario() {
        let app = App::default();
        assert_eq!(app.scenario_id(), "default");
        assert!(!app.risks.is_empty());
    }

    #[test]
    fn active_view_tracks_panels_and_tabs() {
        let mut app = App::default();
        assert_eq!(app.active_view(), ViewId::Dashboard);
        app.active_panel = Panel::Submissions;
        app.tab_index = 2;
        assert_eq!(app.active_view(), ViewId::SubmissionsTriage);
        app.active_panel = Panel::ModelRegistry;
        app.tab_index = 1;
        assert_eq!(app.active_view(), ViewId::ModelRegistryGates);
    }

    #[test]
    fn actions_are_contextual() {
        assert_eq!(
            available_actions(ViewId::SubmissionsQueue),
            vec![
                ActionKey::Inspect,
                ActionKey::Assign,
                ActionKey::Refer,
                ActionKey::Filter
            ]
        );
        assert_eq!(
            available_actions(ViewId::Reporting),
            vec![ActionKey::Export]
        );
        assert!(available_actions(ViewId::Dashboard).is_empty());
    }

    #[test]
    fn hidden_tabs_do_not_expose_row_navigation() {
        let mut app = App::default();
        app.active_panel = Panel::Submissions;
        app.tab_index = 1;
        assert_eq!(app.active_view(), ViewId::SubmissionsForm);
        assert_eq!(app.visible_row_count(), 0);
        app.active_panel = Panel::LossModels;
        app.tab_index = 1;
        assert_eq!(app.visible_row_count(), 0);
    }

    #[test]
    fn filters_are_case_insensitive_and_scoped() {
        let mut app = App::default();
        app.committed_filters
            .insert(ViewId::RiskRegistry, "fairness".to_string());
        app.committed_filters
            .insert(ViewId::Compliance, "naic".to_string());
        assert_eq!(app.visible_row_count_for(ViewId::RiskRegistry), 1);
        assert_eq!(app.visible_row_count_for(ViewId::Compliance), 1);
        assert_eq!(app.visible_row_count_for(ViewId::SubmissionsQueue), 5);
    }

    #[test]
    fn assign_workflow_updates_submission_state() {
        let mut app = App::default();
        app.apply_assign(AssignContext::Submission(0), 1, 2);
        let row = &app.submissions[0];
        assert_eq!(row.owner, "Priya Rao");
        assert_eq!(row.urgency, "Critical");
        assert_eq!(row.status, "Assigned");
        assert!(app
            .flash
            .as_ref()
            .is_some_and(|flash| flash.text.contains("assigned")));
    }

    #[test]
    fn referral_workflow_updates_submission_state() {
        let mut app = App::default();
        app.apply_referral(ReferContext::Submission(0), 0, 2);
        let row = &app.submissions[0];
        assert_eq!(row.status, "Referred");
        assert_eq!(row.referred_to.as_deref(), Some("AI Governance"));
        assert_eq!(row.referral_reason.as_deref(), Some("High model risk"));
    }

    #[test]
    fn export_workflow_records_receipt() {
        let mut app = App::default();
        app.apply_export(ViewId::Reporting, 1, 0);
        let receipt = app.last_export.expect("export receipt");
        assert_eq!(receipt.view, ViewId::Reporting);
        assert_eq!(receipt.format, "Board pack");
        assert_eq!(receipt.audience, "Board");
        assert_eq!(app.export_payloads.len(), 1);
    }

    #[test]
    fn simulation_workflow_updates_reserve_snapshot() {
        let mut app = App::default();
        app.apply_simulation(SimulateTarget::Reserves, 1, 1);
        assert_eq!(app.reserves.scenario, "Adverse large-loss shock");
        assert_eq!(app.reserves.iterations, 5_000);
        assert_eq!(app.reserves.status, "WATCH");
        assert_eq!(app.simulation_results.len(), 1);
    }

    #[test]
    fn unsupported_actions_emit_flash_message() {
        let mut app = App::default();
        app.handle_action_request(ActionKey::Assign);
        assert!(app
            .flash
            .as_ref()
            .is_some_and(|flash| flash.text.contains("not available")));
    }
}
