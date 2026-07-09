use ratatui::{
    style::{Modifier, Style},
    widgets::{Cell, Row},
};

use crate::{
    models::{
        AuditRecord, BiasRecord, ComplianceRecord, LossRecord, ModelRecord, RiskRecord,
        SubmissionRecord,
    },
    theme::{C_DIM, C_MUTED, C_TEXT},
};

pub fn risk_row(row: &RiskRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.id.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.category.clone()).style(
            Style::default()
                .fg(row.category_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.description.clone()).style(Style::default().fg(C_TEXT)),
        Cell::from(row.frequency.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.severity.clone()).style(
            Style::default()
                .fg(row.severity_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.mitigation.clone()).style(Style::default().fg(C_MUTED)),
    ])
}

pub fn submission_row(row: &SubmissionRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.ref_id.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.insured.clone()),
        Cell::from(row.line.clone()),
        Cell::from(row.limit.clone()),
        Cell::from(row.score_text()).style(
            Style::default()
                .fg(row.score_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.flag.clone()).style(Style::default().fg(row.flag_color)),
        Cell::from(row.status.clone()).style(
            Style::default()
                .fg(row.status_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

pub fn loss_row(row: &LossRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.ay.clone()),
        Cell::from(row.m12.clone()),
        Cell::from(row.m24.clone()),
        Cell::from(row.m36.clone()),
        Cell::from(row.m48.clone()).style(Style::default().fg(if row.m48 == "-" {
            C_DIM
        } else {
            C_TEXT
        })),
        Cell::from(row.m60.clone()).style(Style::default().fg(if row.m60 == "-" {
            C_DIM
        } else {
            C_TEXT
        })),
        Cell::from(row.ultimate.clone()).style(
            Style::default()
                .fg(row.ultimate_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.method.clone()).style(Style::default().fg(C_MUTED)),
    ])
}

pub fn model_row(row: &ModelRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.id.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.name.clone())
            .style(Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)),
        Cell::from(row.purpose.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.version.clone()).style(Style::default().fg(C_DIM)),
        Cell::from(row.drift.clone()).style(
            Style::default()
                .fg(row.drift_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.bias.clone()).style(
            Style::default()
                .fg(row.bias_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.explainability.clone()).style(Style::default().fg(row.explainability_color)),
        Cell::from(row.status.clone()).style(
            Style::default()
                .fg(row.status_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

pub fn bias_row(row: &BiasRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.attribute.clone()),
        Cell::from(row.group.clone()),
        Cell::from(row.approval_rate.clone()),
        Cell::from(row.di_ratio.clone()).style(Style::default().fg(row.status_color)),
        Cell::from(row.fpr.clone()),
        Cell::from(row.fnr.clone()),
        Cell::from(row.status.clone()).style(
            Style::default()
                .fg(row.status_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

pub fn audit_row(row: &AuditRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.ts.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.actor.clone()).style(Style::default().fg(C_TEXT)),
        Cell::from(row.action.clone()),
        Cell::from(row.model.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.outcome.clone()).style(
            Style::default()
                .fg(row.outcome_color)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(row.hash.clone()).style(Style::default().fg(C_DIM)),
    ])
}

pub fn compliance_row(row: &ComplianceRecord) -> Row<'static> {
    Row::new(vec![
        Cell::from(row.framework.clone())
            .style(Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)),
        Cell::from(row.scope.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.last_audit.clone()).style(Style::default().fg(C_MUTED)),
        Cell::from(row.findings.clone()).style(Style::default().fg(row.findings_color)),
        Cell::from(row.status.clone()).style(
            Style::default()
                .fg(row.status_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}
