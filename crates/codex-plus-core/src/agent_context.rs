use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelConfig {
    #[serde(default = "default_auto_context")]
    pub auto_context: bool,
    #[serde(default = "default_scope")]
    pub scope: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_agent_context_root")]
    pub agent_context_root: String,
    #[serde(default = "default_agent_context_bin")]
    pub agent_context_bin: String,
}

impl Default for AgentContextPanelConfig {
    fn default() -> Self {
        Self {
            auto_context: default_auto_context(),
            scope: default_scope(),
            mode: default_mode(),
            agent_context_root: default_agent_context_root(),
            agent_context_bin: default_agent_context_bin(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelStatus {
    #[serde(default = "default_status")]
    pub last_status: String,
    #[serde(default)]
    pub last_message: String,
    #[serde(default)]
    pub last_session_id: String,
    #[serde(default)]
    pub last_goal: String,
    #[serde(default)]
    pub last_scope: String,
    #[serde(default)]
    pub last_mode: String,
    #[serde(default)]
    pub last_generated_pack: String,
    #[serde(default)]
    pub last_sources_jsonl: String,
    #[serde(default)]
    pub last_manifest_json: String,
    #[serde(default)]
    pub last_resolution_plan_json: String,
    #[serde(default)]
    pub last_codex_preflight_md: String,
    #[serde(default)]
    pub last_model_input_md: String,
    #[serde(default)]
    pub last_runtime_task_md: String,
    #[serde(default)]
    pub last_review_file: String,
    #[serde(default)]
    pub last_review_client_html: String,
    #[serde(default)]
    pub last_review_launch_md: String,
    #[serde(default)]
    pub last_generated_at_ms: u128,
    #[serde(default)]
    pub access_audit: AgentContextPanelAccessAudit,
    #[serde(default)]
    pub semantic_launchd: AgentContextPanelSemanticLaunchd,
    #[serde(default)]
    pub semantic_readiness: AgentContextPanelSemanticReadiness,
    #[serde(default)]
    pub v1_acceptance: AgentContextPanelV1Acceptance,
    #[serde(default)]
    pub v1_stage_status: AgentContextPanelV1StageStatus,
    #[serde(default)]
    pub feedback_replay_trend: AgentContextPanelFeedbackReplayTrend,
}

impl Default for AgentContextPanelStatus {
    fn default() -> Self {
        Self {
            last_status: default_status(),
            last_message: String::new(),
            last_session_id: String::new(),
            last_goal: String::new(),
            last_scope: String::new(),
            last_mode: String::new(),
            last_generated_pack: String::new(),
            last_sources_jsonl: String::new(),
            last_manifest_json: String::new(),
            last_resolution_plan_json: String::new(),
            last_codex_preflight_md: String::new(),
            last_model_input_md: String::new(),
            last_runtime_task_md: String::new(),
            last_review_file: String::new(),
            last_review_client_html: String::new(),
            last_review_launch_md: String::new(),
            last_generated_at_ms: 0,
            access_audit: AgentContextPanelAccessAudit::default(),
            semantic_launchd: AgentContextPanelSemanticLaunchd::default(),
            semantic_readiness: AgentContextPanelSemanticReadiness::default(),
            v1_acceptance: AgentContextPanelV1Acceptance::default(),
            v1_stage_status: AgentContextPanelV1StageStatus::default(),
            feedback_replay_trend: AgentContextPanelFeedbackReplayTrend::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelFeedbackReplayTrend {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub health: String,
    #[serde(default)]
    pub reports: usize,
    #[serde(default)]
    pub cases: usize,
    #[serde(default)]
    pub latest_expected_top1_rate: String,
    #[serde(default)]
    pub trend_rank_improvements: usize,
    #[serde(default)]
    pub trend_rank_regressions: usize,
    #[serde(default)]
    pub latest_replay_report_path: String,
    #[serde(default)]
    pub latest_trend_report_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelSemanticLaunchd {
    #[serde(default)]
    pub health: String,
    #[serde(default)]
    pub installed: bool,
    #[serde(default)]
    pub plist_path: String,
    #[serde(default)]
    pub script_path: String,
    #[serde(default)]
    pub latest_maintain_report: String,
    #[serde(default)]
    pub latest_prune_report: String,
    #[serde(default)]
    pub monitor_path: String,
    #[serde(default)]
    pub latest_launchd_activity_at: String,
    #[serde(default)]
    pub next_expected_run_after: String,
    #[serde(default)]
    pub natural_run_due: bool,
    #[serde(default)]
    pub natural_run_overdue: bool,
    #[serde(default)]
    pub seconds_overdue: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelSemanticReadiness {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub semantic_chunks: usize,
    #[serde(default)]
    pub trend_days_observed: usize,
    #[serde(default)]
    pub trend_days_remaining: usize,
    #[serde(default)]
    pub monitor_snapshots: usize,
    #[serde(default)]
    pub next_monitor_due_at: String,
    #[serde(default)]
    pub earliest_multi_day_check_after: String,
    #[serde(default)]
    pub next_action: String,
    #[serde(default)]
    pub report_path: String,
    #[serde(default)]
    pub report_markdown_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelV1Acceptance {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub decision: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub report_path: String,
    #[serde(default)]
    pub report_markdown_path: String,
    #[serde(default)]
    pub next_commands: Vec<String>,
    #[serde(default)]
    pub followup_json_path: String,
    #[serde(default)]
    pub followup_markdown_path: String,
    #[serde(default)]
    pub can_recheck_now: bool,
    #[serde(default)]
    pub earliest_recheck_after: String,
    #[serde(default)]
    pub next_monitor_due_at: String,
    #[serde(default)]
    pub trend_days_remaining: usize,
    #[serde(default)]
    pub wait_reason: String,
    #[serde(default)]
    pub next_gate_at: String,
    #[serde(default)]
    pub seconds_until_next_gate: u64,
    #[serde(default)]
    pub next_evidence_gate_reason: String,
    #[serde(default)]
    pub next_evidence_gate_at: String,
    #[serde(default)]
    pub seconds_until_next_evidence_gate: u64,
    #[serde(default)]
    pub acceptance_wait_reason: String,
    #[serde(default)]
    pub acceptance_gate_at: String,
    #[serde(default)]
    pub seconds_until_acceptance_gate: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelV1StageStatus {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub decision: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub report_path: String,
    #[serde(default)]
    pub report_markdown_path: String,
    #[serde(default)]
    pub stages_total: usize,
    #[serde(default)]
    pub ok: usize,
    #[serde(default)]
    pub waiting_for_time: usize,
    #[serde(default)]
    pub warning: usize,
    #[serde(default)]
    pub failed: usize,
    #[serde(default)]
    pub wait_reason: String,
    #[serde(default)]
    pub next_gate_at: String,
    #[serde(default)]
    pub seconds_until_next_gate: u64,
    #[serde(default)]
    pub next_evidence_gate_reason: String,
    #[serde(default)]
    pub next_evidence_gate_at: String,
    #[serde(default)]
    pub seconds_until_next_evidence_gate: u64,
    #[serde(default)]
    pub acceptance_wait_reason: String,
    #[serde(default)]
    pub acceptance_gate_at: String,
    #[serde(default)]
    pub seconds_until_acceptance_gate: u64,
    #[serde(default)]
    pub trend_days_remaining: usize,
    #[serde(default)]
    pub stages: Vec<AgentContextPanelV1StageRow>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelV1StageRow {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub progress: u8,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelAccessAudit {
    #[serde(default)]
    pub audit_path: String,
    #[serde(default)]
    pub events_total: usize,
    #[serde(default)]
    pub recent_allowed: usize,
    #[serde(default)]
    pub recent_denied: usize,
    #[serde(default)]
    pub recent_filtered: usize,
    #[serde(default)]
    pub recent_consent_required: usize,
    #[serde(default)]
    pub recent_events: Vec<AgentContextAccessAuditEvent>,
    #[serde(default)]
    pub last_denied: Option<AgentContextAccessAuditEvent>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextAccessAuditEvent {
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub decision: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub source_id: String,
    #[serde(default)]
    pub source_chunk_id: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextPanelState {
    pub config_path: String,
    pub status_path: String,
    pub feedback_path: String,
    pub config: AgentContextPanelConfig,
    pub status: AgentContextPanelStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextFeedbackEntry {
    #[serde(default)]
    pub winner: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub created_at_ms: u128,
    #[serde(default)]
    pub status_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextLaunchPrewarm {
    pub status: String,
    pub message: String,
    pub goal: String,
    pub scope: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextTaskPreflight {
    pub status: String,
    pub message: String,
    pub goal: String,
    pub scope: String,
    pub mode: String,
    pub sources_included: usize,
    pub codex_preflight_md: String,
    pub model_input_md: String,
    pub context_md: String,
    pub sources_jsonl: String,
    pub manifest_json: String,
    pub resolution_plan_json: String,
    pub session_id: String,
    pub runtime_task_md: String,
    pub runtime_task_json: String,
    pub review_file: String,
    pub agent_preflight_md: String,
    pub review_launch_md: String,
    pub review_client_html: String,
    pub review_server_url: String,
    pub start_server_command: String,
    pub open_client_command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextV1FollowupResult {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub ready: bool,
    #[serde(default, alias = "can_recheck_now")]
    pub can_recheck_now: bool,
    #[serde(default, alias = "earliest_recheck_after")]
    pub earliest_recheck_after: String,
    #[serde(default, alias = "next_monitor_due_at")]
    pub next_monitor_due_at: String,
    #[serde(default, alias = "trend_days_remaining")]
    pub trend_days_remaining: usize,
    #[serde(default, alias = "wait_reason")]
    pub wait_reason: String,
    #[serde(default, alias = "next_gate_at")]
    pub next_gate_at: String,
    #[serde(default, alias = "seconds_until_next_gate")]
    pub seconds_until_next_gate: u64,
    #[serde(default, alias = "next_evidence_gate_reason")]
    pub next_evidence_gate_reason: String,
    #[serde(default, alias = "next_evidence_gate_at")]
    pub next_evidence_gate_at: String,
    #[serde(default, alias = "seconds_until_next_evidence_gate")]
    pub seconds_until_next_evidence_gate: u64,
    #[serde(default, alias = "acceptance_wait_reason")]
    pub acceptance_wait_reason: String,
    #[serde(default, alias = "acceptance_gate_at")]
    pub acceptance_gate_at: String,
    #[serde(default, alias = "seconds_until_acceptance_gate")]
    pub seconds_until_acceptance_gate: u64,
    #[serde(default, alias = "followup_plan_latest_md_path")]
    pub followup_plan_latest_md_path: String,
    #[serde(default, alias = "acceptance_latest_md_path")]
    pub acceptance_latest_md_path: String,
    #[serde(default, alias = "latest_md_path")]
    pub latest_md_path: String,
    #[serde(default, alias = "latest_json_path")]
    pub latest_json_path: String,
    #[serde(default, alias = "next_command")]
    pub next_command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextAccessPolicy {
    #[serde(default, alias = "allow_providers")]
    pub allow_providers: Vec<String>,
    #[serde(default, alias = "deny_providers")]
    pub deny_providers: Vec<String>,
    #[serde(default, alias = "deny_path_patterns")]
    pub deny_path_patterns: Vec<String>,
    #[serde(default, alias = "require_consent_providers")]
    pub require_consent_providers: Vec<String>,
    #[serde(default, alias = "require_consent_path_patterns")]
    pub require_consent_path_patterns: Vec<String>,
    #[serde(default, alias = "audit_max_bytes")]
    pub audit_max_bytes: u64,
    #[serde(default, alias = "audit_max_rotated_files")]
    pub audit_max_rotated_files: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextAccessPolicyPatch {
    #[serde(default)]
    pub allow_providers: Vec<String>,
    #[serde(default)]
    pub remove_allow_providers: Vec<String>,
    #[serde(default)]
    pub deny_providers: Vec<String>,
    #[serde(default)]
    pub remove_deny_providers: Vec<String>,
    #[serde(default)]
    pub deny_path_patterns: Vec<String>,
    #[serde(default)]
    pub remove_deny_path_patterns: Vec<String>,
    #[serde(default)]
    pub require_consent_providers: Vec<String>,
    #[serde(default)]
    pub remove_require_consent_providers: Vec<String>,
    #[serde(default)]
    pub require_consent_path_patterns: Vec<String>,
    #[serde(default)]
    pub remove_require_consent_path_patterns: Vec<String>,
    #[serde(default)]
    pub audit_max_bytes: Option<u64>,
    #[serde(default)]
    pub audit_max_rotated_files: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextAccessPolicyState {
    pub policy_path: String,
    pub updated: bool,
    pub changes: Vec<String>,
    pub policy: AgentContextAccessPolicy,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextAccessPolicyCommandOutput {
    #[serde(default, alias = "policyPath")]
    policy_path: String,
    #[serde(default)]
    updated: bool,
    #[serde(default)]
    changes: Vec<String>,
    #[serde(default)]
    policy: AgentContextAccessPolicy,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextAccessConsentGrant {
    #[serde(default, alias = "created_at")]
    pub created_at: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default, alias = "source_id")]
    pub source_id: String,
    #[serde(default, alias = "source_chunk_id")]
    pub source_chunk_id: String,
    #[serde(default)]
    pub path: String,
    #[serde(default, alias = "relative_path")]
    pub relative_path: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentContextAccessConsentState {
    pub consent_path: String,
    pub written: bool,
    pub grants_total: usize,
    pub grant: AgentContextAccessConsentGrant,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextAccessConsentCommandOutput {
    #[serde(default, alias = "consentPath")]
    consent_path: String,
    #[serde(default)]
    written: bool,
    #[serde(default, alias = "grantsTotal")]
    grants_total: usize,
    #[serde(default)]
    grant: AgentContextAccessConsentGrant,
}

#[derive(Debug, Clone, Deserialize)]
struct AgentContextRuntimePanelStatus {
    #[serde(default)]
    auto_context: bool,
    #[serde(default)]
    goal: Option<String>,
    #[serde(default)]
    scope: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    preflight: Option<AgentContextRuntimePreflight>,
    #[serde(default)]
    last_generated_pack: Option<String>,
    #[serde(default)]
    last_sources_jsonl: Option<String>,
    #[serde(default)]
    last_manifest_json: Option<String>,
    #[serde(default)]
    last_resolution_plan_json: Option<String>,
    #[serde(default)]
    last_codex_preflight_md: Option<String>,
    #[serde(default)]
    last_runtime_task_md: Option<String>,
    #[serde(default)]
    last_review_file: Option<String>,
    #[serde(default)]
    last_review_client_html: Option<String>,
    #[serde(default)]
    last_review_launch_md: Option<String>,
    #[serde(default)]
    access_audit: Option<AgentContextRuntimeAccessAudit>,
    #[serde(default)]
    semantic_launchd: Option<AgentContextRuntimeSemanticLaunchd>,
    #[serde(default)]
    semantic_readiness: Option<AgentContextRuntimeSemanticReadiness>,
    #[serde(default)]
    v1_acceptance: Option<AgentContextRuntimeV1Acceptance>,
    #[serde(default)]
    v1_stage_status: Option<AgentContextRuntimeV1StageStatus>,
    #[serde(default)]
    feedback: Option<AgentContextRuntimeFeedback>,
}

#[derive(Debug, Clone, Deserialize)]
struct AgentContextRuntimePreflight {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    sources_included: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeAccessAudit {
    #[serde(default)]
    audit_path: String,
    #[serde(default)]
    events_total: usize,
    #[serde(default)]
    recent_events: Vec<AgentContextRuntimeAccessAuditEvent>,
    #[serde(default)]
    summary: AgentContextRuntimeAccessAuditSummary,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeAccessAuditSummary {
    #[serde(default)]
    decisions: std::collections::BTreeMap<String, usize>,
    #[serde(default)]
    last_denied: Option<AgentContextRuntimeAccessAuditEvent>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeAccessAuditEvent {
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    action: String,
    #[serde(default)]
    decision: String,
    #[serde(default)]
    identifier: String,
    #[serde(default)]
    provider: String,
    #[serde(default)]
    source_id: String,
    #[serde(default)]
    source_chunk_id: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    reason: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeFeedback {
    #[serde(default)]
    replay_trend: AgentContextRuntimeFeedbackReplayTrend,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeFeedbackReplayTrend {
    #[serde(default)]
    exists: bool,
    #[serde(default)]
    health: String,
    #[serde(default)]
    latest_replay_report_path: String,
    #[serde(default)]
    latest_trend_report_path: String,
    #[serde(default)]
    summary: AgentContextRuntimeFeedbackReplayTrendSummary,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeFeedbackReplayTrendSummary {
    #[serde(default)]
    reports: usize,
    #[serde(default)]
    cases: usize,
    #[serde(default)]
    latest_expected_top1_rate: f64,
    #[serde(default)]
    trend_rank_improvements: usize,
    #[serde(default)]
    trend_rank_regressions: usize,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticLaunchd {
    #[serde(default)]
    health: String,
    #[serde(default)]
    installed: bool,
    #[serde(default)]
    plist_path: String,
    #[serde(default)]
    script_path: String,
    #[serde(default)]
    reports: AgentContextRuntimeSemanticLaunchdReports,
    #[serde(default)]
    monitor: AgentContextRuntimeSemanticLaunchdMonitor,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticLaunchdMonitor {
    #[serde(default)]
    path: String,
    #[serde(default)]
    summary: AgentContextRuntimeSemanticLaunchdMonitorSummary,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticLaunchdMonitorSummary {
    #[serde(default)]
    latest_launchd_activity_at: String,
    #[serde(default)]
    next_expected_run_after: String,
    #[serde(default)]
    natural_run_due: bool,
    #[serde(default)]
    natural_run_overdue: bool,
    #[serde(default)]
    seconds_overdue: i64,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticLaunchdReports {
    #[serde(default)]
    semantic_maintain: AgentContextRuntimeSemanticLaunchdReport,
    #[serde(default)]
    semantic_ann_prune: AgentContextRuntimeSemanticLaunchdReport,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticLaunchdReport {
    #[serde(default)]
    path: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticReadiness {
    #[serde(default)]
    exists: bool,
    #[serde(default)]
    path: String,
    #[serde(default)]
    latest_md_path: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    ready: bool,
    #[serde(default)]
    next_action: String,
    #[serde(default)]
    summary: AgentContextRuntimeSemanticReadinessSummary,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1Acceptance {
    #[serde(default)]
    exists: bool,
    #[serde(default)]
    path: String,
    #[serde(default)]
    latest_md_path: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    ready: bool,
    #[serde(default)]
    decision: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    next_commands: Vec<String>,
    #[serde(default)]
    latest_followup_json_path: String,
    #[serde(default)]
    latest_followup_md_path: String,
    #[serde(default)]
    followup_plan: AgentContextRuntimeV1FollowupPlan,
    #[serde(default)]
    followup_check: AgentContextRuntimeV1FollowupCheck,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1FollowupPlan {
    #[serde(default)]
    can_recheck_now: bool,
    #[serde(default)]
    earliest_recheck_after: String,
    #[serde(default)]
    next_monitor_due_at: String,
    #[serde(default)]
    trend_days_remaining: usize,
    #[serde(default)]
    latest_json_path: String,
    #[serde(default)]
    latest_md_path: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1FollowupCheck {
    #[serde(default)]
    wait_reason: String,
    #[serde(default)]
    next_gate_at: String,
    #[serde(default)]
    seconds_until_next_gate: u64,
    #[serde(default)]
    next_evidence_gate_reason: String,
    #[serde(default)]
    next_evidence_gate_at: String,
    #[serde(default)]
    seconds_until_next_evidence_gate: u64,
    #[serde(default)]
    acceptance_wait_reason: String,
    #[serde(default)]
    acceptance_gate_at: String,
    #[serde(default)]
    seconds_until_acceptance_gate: u64,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1StageStatus {
    #[serde(default)]
    exists: bool,
    #[serde(default)]
    path: String,
    #[serde(default)]
    latest_md_path: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    ready: bool,
    #[serde(default)]
    decision: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    summary: AgentContextRuntimeV1StageSummary,
    #[serde(default)]
    next_gates: AgentContextRuntimeV1StageGates,
    #[serde(default)]
    stages: Vec<AgentContextRuntimeV1StageRow>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1StageSummary {
    #[serde(default)]
    stages_total: usize,
    #[serde(default)]
    ok: usize,
    #[serde(default)]
    waiting_for_time: usize,
    #[serde(default)]
    warning: usize,
    #[serde(default)]
    failed: usize,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1StageGates {
    #[serde(default)]
    wait_reason: String,
    #[serde(default)]
    next_gate_at: String,
    #[serde(default)]
    seconds_until_next_gate: u64,
    #[serde(default)]
    next_evidence_gate_reason: String,
    #[serde(default)]
    next_evidence_gate_at: String,
    #[serde(default)]
    seconds_until_next_evidence_gate: u64,
    #[serde(default)]
    acceptance_wait_reason: String,
    #[serde(default)]
    acceptance_gate_at: String,
    #[serde(default)]
    seconds_until_acceptance_gate: u64,
    #[serde(default)]
    trend_days_remaining: usize,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeV1StageRow {
    #[serde(default)]
    id: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    progress: u8,
    #[serde(default)]
    summary: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeSemanticReadinessSummary {
    #[serde(default)]
    reason: String,
    #[serde(default)]
    semantic_chunks: usize,
    #[serde(default)]
    trend_days_observed: usize,
    #[serde(default)]
    trend_days_remaining: usize,
    #[serde(default)]
    monitor_snapshots: usize,
    #[serde(default)]
    next_monitor_due_at: String,
    #[serde(default)]
    earliest_multi_day_check_after: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeCodexPreflight {
    #[serde(default)]
    status: String,
    #[serde(default)]
    error: String,
    #[serde(default)]
    goal: String,
    #[serde(default)]
    session_id: String,
    #[serde(default)]
    stage: String,
    #[serde(default)]
    source_scope: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    sources_included: usize,
    #[serde(default)]
    preflight_markdown_path: String,
    #[serde(default)]
    context_md_path: Option<String>,
    #[serde(default)]
    sources_jsonl_path: Option<String>,
    #[serde(default)]
    manifest_json_path: Option<String>,
    #[serde(default)]
    resolution_plan_json_path: Option<String>,
    #[serde(default)]
    runtime_task_md_path: String,
    #[serde(default)]
    runtime_task_json_path: String,
    #[serde(default)]
    review_file: String,
    #[serde(default)]
    agent_preflight: Option<AgentContextRuntimeTaskAgentPreflight>,
    #[serde(default)]
    review_launch: Option<AgentContextRuntimeTaskReviewLaunch>,
    #[serde(default)]
    client_html_path: String,
    #[serde(default)]
    review_server_url: String,
    #[serde(default)]
    start_server_command: String,
    #[serde(default)]
    open_client_command: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeAgentPreflight {
    #[serde(default)]
    status: String,
    #[serde(default)]
    next_message: String,
    #[serde(default)]
    session_id: String,
    #[serde(default)]
    source_scope: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    review_file: String,
    #[serde(default)]
    agent_preflight_md_path: String,
    #[serde(default)]
    files: AgentContextRuntimeAgentPreflightFiles,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeAgentPreflightFiles {
    #[serde(default)]
    context_md_path: Option<String>,
    #[serde(default)]
    sources_jsonl_path: Option<String>,
    #[serde(default)]
    model_input_md_path: Option<String>,
    #[serde(default)]
    runtime_task_md_path: Option<String>,
    #[serde(default)]
    runtime_task_json_path: Option<String>,
    #[serde(default)]
    runtime_review_client_html_path: Option<String>,
    #[serde(default)]
    runtime_review_launch_md_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeTaskAgentPreflight {
    #[serde(default)]
    agent_preflight_md_path: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AgentContextRuntimeTaskReviewLaunch {
    #[serde(default)]
    review_launch_md_path: String,
}

pub fn load_agent_context_panel_state() -> anyhow::Result<AgentContextPanelState> {
    Ok(AgentContextPanelState {
        config_path: config_path_string(),
        status_path: status_path_string(),
        feedback_path: feedback_path_string(),
        config: load_agent_context_panel_config()?,
        status: load_agent_context_panel_status().unwrap_or_default(),
    })
}

pub fn run_agent_context_task_preflight(goal: &str) -> anyhow::Result<AgentContextTaskPreflight> {
    let config = load_agent_context_panel_config()?;
    let goal = goal.trim();
    if goal.is_empty() {
        return Err(anyhow!("任务目标不能为空。"));
    }
    if !config.auto_context {
        let preflight = AgentContextTaskPreflight {
            status: "disabled".to_string(),
            message: "Auto Context 已关闭，未启动 Doctor runtime-task。".to_string(),
            goal: goal.to_string(),
            scope: config.scope.clone(),
            mode: config.mode.clone(),
            ..AgentContextTaskPreflight::default()
        };
        save_task_preflight_status(&preflight)?;
        return Ok(preflight);
    }
    let output = run_preflight_command(&config, goal)?;
    if !output.status.success() {
        let message = command_failure_message(&output);
        let previous = load_agent_context_panel_status().unwrap_or_default();
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_goal: goal.to_string(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let runtime: AgentContextRuntimeCodexPreflight = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context runtime-task output")?;
    let preflight = runtime_preflight_to_task_preflight(runtime, &config, goal);
    save_task_preflight_status(&preflight)?;
    Ok(preflight)
}

pub fn run_agent_context_model_input_review() -> anyhow::Result<AgentContextTaskPreflight> {
    let config = load_agent_context_panel_config()?;
    let previous = load_agent_context_panel_status().unwrap_or_default();
    let session_id = previous.last_session_id.trim();
    if session_id.is_empty() {
        return Err(anyhow!("请先运行任务预检，生成 Doctor runtime session。"));
    }
    let output = run_agent_preflight_context_command(&config, session_id)?;
    if !output.status.success() {
        let message = command_failure_message(&output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let runtime: AgentContextRuntimeAgentPreflight = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context agent-preflight context output")?;
    let preflight = agent_preflight_to_task_preflight(runtime, &config, &previous);
    save_task_preflight_status(&preflight)?;
    Ok(preflight)
}

pub fn run_agent_context_answer_review_prepare(reason: &str) -> anyhow::Result<Value> {
    let config = load_agent_context_panel_config()?;
    let previous = load_agent_context_panel_status().unwrap_or_default();
    let session_id = previous.last_session_id.trim().to_string();
    if session_id.is_empty() {
        return Err(anyhow!("请先生成并审查 Doctor model_input.md。"));
    }
    let reason = if reason.trim().is_empty() {
        "approved from Codex++ live task flow"
    } else {
        reason.trim()
    };
    let approve_output =
        run_context_review_decision_command(&config, &session_id, "approve", reason)?;
    if !approve_output.status.success() {
        let message = command_failure_message(&approve_output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let answer_output = run_agent_preflight_answer_command(&config, &session_id, reason)?;
    if !answer_output.status.success() {
        let message = command_failure_message(&answer_output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let stdout = String::from_utf8_lossy(&answer_output.stdout);
    let raw: Value = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context agent-preflight answer output")?;
    let result = answer_preflight_to_bridge_value(raw);
    save_agent_context_panel_status(&AgentContextPanelStatus {
        last_status: string_at(&result, "/status", "awaiting_answer_output"),
        last_message: string_at(
            &result,
            "/message",
            "Doctor 已批准 model_input.md，并生成 answer_packet.md。",
        ),
        last_session_id: previous.last_session_id.clone(),
        last_goal: previous.last_goal.clone(),
        last_scope: config.scope.clone(),
        last_mode: config.mode.clone(),
        last_generated_at_ms: now_ms(),
        ..previous
    })?;
    Ok(result)
}

pub fn run_agent_context_execution_review_prepare(
    reason: &str,
    answer_text: &str,
) -> anyhow::Result<Value> {
    let answer_text = answer_text.trim();
    if answer_text.is_empty() {
        return Ok(json!({
            "status": "failed",
            "message": "未能读取上一条模型答案，无法写入 Doctor answer_review。请复制答案文本后再批准执行审查。"
        }));
    }
    let config = load_agent_context_panel_config()?;
    let previous = load_agent_context_panel_status().unwrap_or_default();
    let session_id = previous.last_session_id.trim().to_string();
    if session_id.is_empty() {
        return Err(anyhow!("请先生成并审查 Doctor answer_packet.md。"));
    }
    let reason = if reason.trim().is_empty() {
        "approved answer from Codex++ live task flow"
    } else {
        reason.trim()
    };
    let answer_file = write_answer_review_bridge_file(&session_id, answer_text)?;

    let record_output =
        run_answer_review_record_command(&config, &session_id, &answer_file, reason)?;
    if !record_output.status.success() {
        let message = command_failure_message(&record_output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let approve_output =
        run_answer_review_decision_command(&config, &session_id, "approve", reason)?;
    if !approve_output.status.success() {
        let message = command_failure_message(&approve_output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let execution_output = run_agent_preflight_execution_command(&config, &session_id, reason)?;
    if !execution_output.status.success() {
        let message = command_failure_message(&execution_output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let stdout = String::from_utf8_lossy(&execution_output.stdout);
    let raw: Value = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context agent-preflight execution output")?;
    let result = execution_preflight_to_bridge_value(raw, answer_file);
    save_agent_context_panel_status(&AgentContextPanelStatus {
        last_status: string_at(&result, "/status", "awaiting_execution"),
        last_message: string_at(
            &result,
            "/message",
            "Doctor 已批准答案，并生成 execution review。",
        ),
        last_session_id: previous.last_session_id.clone(),
        last_goal: previous.last_goal.clone(),
        last_scope: config.scope.clone(),
        last_mode: config.mode.clone(),
        last_generated_at_ms: now_ms(),
        ..previous
    })?;
    Ok(result)
}

pub fn run_agent_context_execution_command(
    command: &str,
    cwd: &str,
    reason: &str,
) -> anyhow::Result<Value> {
    let command = command.trim();
    if command.is_empty() {
        return Ok(json!({
            "status": "failed",
            "message": "执行命令不能为空。请使用“执行命令: <command>”明确给出本机命令。"
        }));
    }
    let config = load_agent_context_panel_config()?;
    let previous = load_agent_context_panel_status().unwrap_or_default();
    let session_id = previous.last_session_id.trim().to_string();
    if session_id.is_empty() {
        return Err(anyhow!(
            "请先进入 Doctor execution review，再执行显式命令。"
        ));
    }
    let reason = if reason.trim().is_empty() {
        "approved execution command from Codex++ live task flow"
    } else {
        reason.trim()
    };
    let cwd = cwd.trim();
    let output = run_execution_review_run_command(
        &config,
        &session_id,
        command,
        if cwd.is_empty() { None } else { Some(cwd) },
        120,
        reason,
    )?;
    if !output.status.success() {
        let message = command_failure_message(&output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let raw: Value = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context execution-review run output")?;
    let result = execution_review_to_bridge_value(
        raw,
        "Doctor ran the approved explicit local command and captured artifacts.",
    );
    save_agent_context_panel_status(&AgentContextPanelStatus {
        last_status: string_at(&result, "/status", "executed"),
        last_message: string_at(
            &result,
            "/message",
            "Doctor 已执行显式命令并生成 artifacts。",
        ),
        last_session_id: previous.last_session_id.clone(),
        last_goal: previous.last_goal.clone(),
        last_scope: config.scope.clone(),
        last_mode: config.mode.clone(),
        last_generated_at_ms: now_ms(),
        ..previous
    })?;
    Ok(result)
}

pub fn run_agent_context_execution_review_decision(
    action: &str,
    reason: &str,
) -> anyhow::Result<Value> {
    let action = action.trim();
    if action != "approve" && action != "reject" {
        return Err(anyhow!("execution review action must be approve or reject"));
    }
    let config = load_agent_context_panel_config()?;
    let previous = load_agent_context_panel_status().unwrap_or_default();
    let session_id = previous.last_session_id.trim().to_string();
    if session_id.is_empty() {
        return Err(anyhow!(
            "请先执行或记录 Doctor execution artifact，再批准执行结果。"
        ));
    }
    let reason = if reason.trim().is_empty() {
        "reviewed execution artifacts from Codex++ live task flow"
    } else {
        reason.trim()
    };
    let output = run_execution_review_decision_command(&config, &session_id, action, reason)?;
    if !output.status.success() {
        let message = command_failure_message(&output);
        save_agent_context_panel_status(&AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_session_id: previous.last_session_id.clone(),
            last_goal: previous.last_goal.clone(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..previous
        })?;
        return Err(anyhow!(message));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let raw: Value = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context execution-review decision output")?;
    let default_message = if action == "approve" {
        "Doctor execution artifacts are approved; the four-stage runtime session is complete."
    } else {
        "Doctor execution artifacts were rejected; revise or rerun before completion."
    };
    let result = execution_review_to_bridge_value(raw, default_message);
    save_agent_context_panel_status(&AgentContextPanelStatus {
        last_status: string_at(&result, "/status", action),
        last_message: string_at(&result, "/message", default_message),
        last_session_id: previous.last_session_id.clone(),
        last_goal: previous.last_goal.clone(),
        last_scope: config.scope.clone(),
        last_mode: config.mode.clone(),
        last_generated_at_ms: now_ms(),
        ..previous
    })?;
    Ok(result)
}

pub fn run_agent_context_panel(goal: &str) -> anyhow::Result<AgentContextPanelState> {
    let config = load_agent_context_panel_config()?;
    let goal = goal.trim();
    let output = run_panel_command(&config, goal)?;
    if !output.status.success() {
        let message = command_failure_message(&output);
        let status = AgentContextPanelStatus {
            last_status: "failed".to_string(),
            last_message: message.clone(),
            last_goal: goal.to_string(),
            last_scope: config.scope.clone(),
            last_mode: config.mode.clone(),
            last_generated_at_ms: now_ms(),
            ..load_agent_context_panel_status().unwrap_or_default()
        };
        save_agent_context_panel_status(&status)?;
        return Err(anyhow!(message));
    }

    let previous = load_agent_context_panel_status().unwrap_or_default();
    let mut status = load_runtime_panel_status(&config)?;
    if status.last_session_id.is_empty() {
        status.last_session_id = previous.last_session_id;
    }
    if status.last_model_input_md.is_empty() {
        status.last_model_input_md = previous.last_model_input_md;
    }
    save_agent_context_panel_status(&status)?;
    Ok(AgentContextPanelState {
        config_path: config_path_string(),
        status_path: status_path_string(),
        feedback_path: feedback_path_string(),
        config,
        status,
    })
}

pub fn run_agent_context_v1_followup() -> anyhow::Result<AgentContextV1FollowupResult> {
    let config = load_agent_context_panel_config()?;
    let output = run_v1_refresh_command(&config)?;
    if !output.status.success() {
        return Err(anyhow!(command_failure_message(&output)));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = parse_v1_refresh_or_followup_output(&stdout)?;
    if let Ok(status) = load_runtime_panel_status(&config) {
        save_agent_context_panel_status(&status)?;
    }
    Ok(result)
}

pub fn load_agent_context_access_policy() -> anyhow::Result<AgentContextAccessPolicyState> {
    let config = load_agent_context_panel_config()?;
    let output = run_access_policy_command(&config, &AgentContextAccessPolicyPatch::default())?;
    parse_access_policy_output(output, &config)
}

pub fn update_agent_context_access_policy(
    patch: &AgentContextAccessPolicyPatch,
) -> anyhow::Result<AgentContextAccessPolicyState> {
    let config = load_agent_context_panel_config()?;
    let output = run_access_policy_command(&config, patch)?;
    parse_access_policy_output(output, &config)
}

pub fn grant_agent_context_access_consent(
    identifier: &str,
    reason: &str,
) -> anyhow::Result<AgentContextAccessConsentState> {
    let config = load_agent_context_panel_config()?;
    let output = run_access_consent_command(&config, identifier, reason)?;
    parse_access_consent_output(output, &config)
}

pub fn start_launch_prewarm() -> AgentContextLaunchPrewarm {
    let config = match load_agent_context_panel_config() {
        Ok(config) => config,
        Err(error) => {
            return AgentContextLaunchPrewarm {
                status: "failed".to_string(),
                message: format!("Agent Context 配置读取失败：{error}"),
                goal: String::new(),
                scope: String::new(),
                mode: String::new(),
            };
        }
    };
    if !config.auto_context {
        let message = "Auto Context 已关闭，Codex++ 启动时未预热上下文包。".to_string();
        let _ = save_launch_prewarm_status(&config, "", "skipped", &message);
        return AgentContextLaunchPrewarm {
            status: "skipped".to_string(),
            message,
            goal: String::new(),
            scope: config.scope,
            mode: config.mode,
        };
    }
    let goal = last_known_goal(&config);
    if goal.is_empty() {
        let message = "没有可复用的任务目标，Codex++ 启动时未预热上下文包。".to_string();
        let _ = save_launch_prewarm_status(&config, "", "skipped", &message);
        return AgentContextLaunchPrewarm {
            status: "skipped".to_string(),
            message,
            goal,
            scope: config.scope,
            mode: config.mode,
        };
    }
    if !PathBuf::from(&config.agent_context_bin).exists() {
        let message = format!("agent-context 不存在：{}", config.agent_context_bin);
        let _ = save_launch_prewarm_status(&config, &goal, "failed", &message);
        return AgentContextLaunchPrewarm {
            status: "failed".to_string(),
            message,
            goal,
            scope: config.scope,
            mode: config.mode,
        };
    }
    let message = "Codex++ 启动时已开始预热 Agent Context。".to_string();
    let _ = save_launch_prewarm_status(&config, &goal, "accepted", &message);
    let thread_goal = goal.clone();
    let spawn_result = thread::Builder::new()
        .name("agent-context-launch-prewarm".to_string())
        .spawn(move || {
            let _ = run_agent_context_panel(&thread_goal);
        });
    if let Err(error) = spawn_result {
        let message = format!("Agent Context 预热线程启动失败：{error}");
        let _ = save_launch_prewarm_status(&config, &goal, "failed", &message);
        return AgentContextLaunchPrewarm {
            status: "failed".to_string(),
            message,
            goal,
            scope: config.scope,
            mode: config.mode,
        };
    }
    AgentContextLaunchPrewarm {
        status: "accepted".to_string(),
        message,
        goal,
        scope: config.scope,
        mode: config.mode,
    }
}

pub fn fallback_agent_context_panel_state() -> AgentContextPanelState {
    AgentContextPanelState {
        config_path: config_path_string(),
        status_path: status_path_string(),
        feedback_path: feedback_path_string(),
        config: AgentContextPanelConfig::default(),
        status: AgentContextPanelStatus::default(),
    }
}

fn run_panel_command(
    config: &AgentContextPanelConfig,
    goal: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("panel")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--source-scope")
        .arg(&config.scope)
        .arg("--mode")
        .arg(&config.mode)
        .arg("--limit")
        .arg("8");
    if !goal.is_empty() {
        command.arg("--goal").arg(goal);
    }
    if !config.auto_context || goal.is_empty() {
        command.arg("--no-auto-context");
    }
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_preflight_command(
    config: &AgentContextPanelConfig,
    goal: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("runtime-task")
        .arg("--goal")
        .arg(goal)
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--port")
        .arg("8765");
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_agent_preflight_context_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("agent-preflight")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--advance")
        .arg("context")
        .arg("--source-scope")
        .arg(&config.scope)
        .arg("--mode")
        .arg(&config.mode)
        .arg("--limit")
        .arg("8");
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_context_review_decision_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    action: &str,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("context-review")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--action")
        .arg(action)
        .arg("--reason")
        .arg(reason);
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_agent_preflight_answer_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("agent-preflight")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--advance")
        .arg("answer")
        .arg("--reason")
        .arg(reason);
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_answer_review_record_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    answer_file: &PathBuf,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("answer-review")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--action")
        .arg("record")
        .arg("--answer-file")
        .arg(answer_file)
        .arg("--reason")
        .arg(reason);
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_answer_review_decision_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    action: &str,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("answer-review")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--action")
        .arg(action)
        .arg("--reason")
        .arg(reason);
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_agent_preflight_execution_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("agent-preflight")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--advance")
        .arg("execution")
        .arg("--reason")
        .arg(reason);
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_execution_review_run_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    execution_command: &str,
    cwd: Option<&str>,
    timeout_seconds: u64,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("execution-review")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--action")
        .arg("run")
        .arg("--command")
        .arg(execution_command)
        .arg("--timeout-seconds")
        .arg(timeout_seconds.to_string())
        .arg("--reason")
        .arg(reason);
    if let Some(cwd) = cwd {
        command.arg("--cwd").arg(cwd);
    }
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_execution_review_decision_command(
    config: &AgentContextPanelConfig,
    session_id: &str,
    action: &str,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    command
        .arg("execution-review")
        .arg("--out")
        .arg(&config.agent_context_root)
        .arg("--session-id")
        .arg(session_id)
        .arg("--action")
        .arg(action)
        .arg("--reason")
        .arg(reason);
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_v1_refresh_command(
    config: &AgentContextPanelConfig,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    for arg in v1_refresh_command_args(config) {
        command.arg(arg);
    }
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_access_policy_command(
    config: &AgentContextPanelConfig,
    patch: &AgentContextAccessPolicyPatch,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    for arg in access_policy_command_args(config, patch) {
        command.arg(arg);
    }
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn run_access_consent_command(
    config: &AgentContextPanelConfig,
    identifier: &str,
    reason: &str,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new(&config.agent_context_bin);
    for arg in access_consent_command_args(config, identifier, reason) {
        command.arg(arg);
    }
    command
        .output()
        .with_context(|| format!("failed to execute {}", config.agent_context_bin))
}

fn v1_refresh_command_args(config: &AgentContextPanelConfig) -> Vec<String> {
    vec![
        "v1-refresh".to_string(),
        "--out".to_string(),
        config.agent_context_root.clone(),
        "--with-manager-feedback-smoke".to_string(),
    ]
}

fn parse_v1_refresh_or_followup_output(
    stdout: &str,
) -> anyhow::Result<AgentContextV1FollowupResult> {
    let value: serde_json::Value = serde_json::from_str(stdout)
        .with_context(|| "failed to parse agent-context v1 refresh output")?;
    if let Some(followup) = value.get("followup_check") {
        return serde_json::from_value(followup.clone())
            .with_context(|| "failed to parse agent-context v1-refresh followup_check output");
    }
    serde_json::from_value(value)
        .with_context(|| "failed to parse agent-context v1-followup output")
}

fn access_policy_command_args(
    config: &AgentContextPanelConfig,
    patch: &AgentContextAccessPolicyPatch,
) -> Vec<String> {
    let mut args = vec![
        "access-policy".to_string(),
        "--out".to_string(),
        config.agent_context_root.clone(),
    ];
    append_repeated_arg(&mut args, "--allow-provider", &patch.allow_providers);
    append_repeated_arg(
        &mut args,
        "--remove-allow-provider",
        &patch.remove_allow_providers,
    );
    append_repeated_arg(&mut args, "--deny-provider", &patch.deny_providers);
    append_repeated_arg(
        &mut args,
        "--remove-deny-provider",
        &patch.remove_deny_providers,
    );
    append_repeated_arg(&mut args, "--deny-path", &patch.deny_path_patterns);
    append_repeated_arg(
        &mut args,
        "--remove-deny-path",
        &patch.remove_deny_path_patterns,
    );
    append_repeated_arg(
        &mut args,
        "--require-consent-provider",
        &patch.require_consent_providers,
    );
    append_repeated_arg(
        &mut args,
        "--remove-require-consent-provider",
        &patch.remove_require_consent_providers,
    );
    append_repeated_arg(
        &mut args,
        "--require-consent-path",
        &patch.require_consent_path_patterns,
    );
    append_repeated_arg(
        &mut args,
        "--remove-require-consent-path",
        &patch.remove_require_consent_path_patterns,
    );
    if let Some(value) = patch.audit_max_bytes {
        args.push("--audit-max-bytes".to_string());
        args.push(value.to_string());
    }
    if let Some(value) = patch.audit_max_rotated_files {
        args.push("--audit-max-rotated-files".to_string());
        args.push(value.to_string());
    }
    args
}

fn access_consent_command_args(
    config: &AgentContextPanelConfig,
    identifier: &str,
    reason: &str,
) -> Vec<String> {
    let mut args = vec![
        "access-consent".to_string(),
        "--out".to_string(),
        config.agent_context_root.clone(),
        "--identifier".to_string(),
        identifier.trim().to_string(),
    ];
    let reason = reason.trim();
    if !reason.is_empty() {
        args.push("--reason".to_string());
        args.push(reason.to_string());
    }
    args
}

fn append_repeated_arg(args: &mut Vec<String>, flag: &str, values: &[String]) {
    for value in values {
        let value = value.trim();
        if !value.is_empty() {
            args.push(flag.to_string());
            args.push(value.to_string());
        }
    }
}

fn parse_access_policy_output(
    output: std::process::Output,
    config: &AgentContextPanelConfig,
) -> anyhow::Result<AgentContextAccessPolicyState> {
    if !output.status.success() {
        return Err(anyhow!(command_failure_message(&output)));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: AgentContextAccessPolicyCommandOutput = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context access-policy output")?;
    Ok(access_policy_output_to_state(parsed, config))
}

fn access_policy_output_to_state(
    output: AgentContextAccessPolicyCommandOutput,
    config: &AgentContextPanelConfig,
) -> AgentContextAccessPolicyState {
    let policy_path = non_empty_or_default(&output.policy_path, || {
        PathBuf::from(&config.agent_context_root)
            .join("config/access_policy.json")
            .to_string_lossy()
            .to_string()
    });
    AgentContextAccessPolicyState {
        policy_path,
        updated: output.updated,
        changes: output.changes,
        policy: output.policy,
    }
}

fn parse_access_consent_output(
    output: std::process::Output,
    config: &AgentContextPanelConfig,
) -> anyhow::Result<AgentContextAccessConsentState> {
    if !output.status.success() {
        return Err(anyhow!(command_failure_message(&output)));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: AgentContextAccessConsentCommandOutput = serde_json::from_str(&stdout)
        .with_context(|| "failed to parse agent-context access-consent output")?;
    Ok(access_consent_output_to_state(parsed, config))
}

fn access_consent_output_to_state(
    output: AgentContextAccessConsentCommandOutput,
    config: &AgentContextPanelConfig,
) -> AgentContextAccessConsentState {
    let consent_path = non_empty_or_default(&output.consent_path, || {
        PathBuf::from(&config.agent_context_root)
            .join("config/access_consent.json")
            .to_string_lossy()
            .to_string()
    });
    AgentContextAccessConsentState {
        consent_path,
        written: output.written,
        grants_total: output.grants_total,
        grant: output.grant,
    }
}

fn load_runtime_panel_status(
    config: &AgentContextPanelConfig,
) -> anyhow::Result<AgentContextPanelStatus> {
    let runtime = load_runtime_panel_status_file(config)?;
    Ok(runtime_status_to_panel_status(runtime, config))
}

fn load_runtime_panel_status_file(
    config: &AgentContextPanelConfig,
) -> anyhow::Result<AgentContextRuntimePanelStatus> {
    let path = PathBuf::from(&config.agent_context_root).join("panel/status.json");
    let text =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let runtime: AgentContextRuntimePanelStatus = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(runtime)
}

fn last_known_goal(config: &AgentContextPanelConfig) -> String {
    if let Ok(status) = load_agent_context_panel_status() {
        let goal = status.last_goal.trim();
        if !goal.is_empty() {
            return goal.to_string();
        }
    }
    if let Ok(runtime) = load_runtime_panel_status_file(config) {
        if let Some(goal) = runtime.goal {
            let goal = goal.trim();
            if !goal.is_empty() {
                return goal.to_string();
            }
        }
    }
    String::new()
}

fn runtime_status_to_panel_status(
    runtime: AgentContextRuntimePanelStatus,
    config: &AgentContextPanelConfig,
) -> AgentContextPanelStatus {
    let preflight_status = runtime
        .preflight
        .as_ref()
        .and_then(|preflight| preflight.status.clone());
    let last_status = preflight_status.unwrap_or_else(|| {
        if !runtime.auto_context {
            "not_checked".to_string()
        } else {
            "ok".to_string()
        }
    });
    let last_message = runtime_status_message(&runtime, &last_status);
    AgentContextPanelStatus {
        last_status,
        last_message,
        last_session_id: String::new(),
        last_goal: runtime.goal.unwrap_or_default(),
        last_scope: non_empty_or_default(&runtime.scope, || config.scope.clone()),
        last_mode: non_empty_or_default(&runtime.mode, || config.mode.clone()),
        last_generated_pack: runtime.last_generated_pack.unwrap_or_default(),
        last_sources_jsonl: runtime.last_sources_jsonl.unwrap_or_default(),
        last_manifest_json: runtime.last_manifest_json.unwrap_or_default(),
        last_resolution_plan_json: runtime.last_resolution_plan_json.unwrap_or_default(),
        last_codex_preflight_md: runtime.last_codex_preflight_md.unwrap_or_default(),
        last_model_input_md: String::new(),
        last_runtime_task_md: runtime.last_runtime_task_md.unwrap_or_default(),
        last_review_file: runtime.last_review_file.unwrap_or_default(),
        last_review_client_html: runtime.last_review_client_html.unwrap_or_default(),
        last_review_launch_md: runtime.last_review_launch_md.unwrap_or_default(),
        last_generated_at_ms: now_ms(),
        access_audit: runtime_access_audit_to_panel(runtime.access_audit.unwrap_or_default()),
        semantic_launchd: runtime_semantic_launchd_to_panel(
            runtime.semantic_launchd.unwrap_or_default(),
        ),
        semantic_readiness: runtime_semantic_readiness_to_panel(
            runtime.semantic_readiness.unwrap_or_default(),
        ),
        v1_acceptance: runtime_v1_acceptance_to_panel(runtime.v1_acceptance.unwrap_or_default()),
        v1_stage_status: runtime_v1_stage_status_to_panel(
            runtime.v1_stage_status.unwrap_or_default(),
        ),
        feedback_replay_trend: runtime_feedback_replay_trend_to_panel(
            runtime.feedback.unwrap_or_default().replay_trend,
        ),
    }
}

fn runtime_feedback_replay_trend_to_panel(
    runtime: AgentContextRuntimeFeedbackReplayTrend,
) -> AgentContextPanelFeedbackReplayTrend {
    AgentContextPanelFeedbackReplayTrend {
        exists: runtime.exists,
        health: non_empty_or_default(&runtime.health, || "not_checked".to_string()),
        reports: runtime.summary.reports,
        cases: runtime.summary.cases,
        latest_expected_top1_rate: format_float(runtime.summary.latest_expected_top1_rate),
        trend_rank_improvements: runtime.summary.trend_rank_improvements,
        trend_rank_regressions: runtime.summary.trend_rank_regressions,
        latest_replay_report_path: runtime.latest_replay_report_path,
        latest_trend_report_path: runtime.latest_trend_report_path,
    }
}

fn runtime_semantic_launchd_to_panel(
    runtime: AgentContextRuntimeSemanticLaunchd,
) -> AgentContextPanelSemanticLaunchd {
    AgentContextPanelSemanticLaunchd {
        health: non_empty_or_default(&runtime.health, || "not_checked".to_string()),
        installed: runtime.installed,
        plist_path: runtime.plist_path,
        script_path: runtime.script_path,
        latest_maintain_report: runtime.reports.semantic_maintain.path,
        latest_prune_report: runtime.reports.semantic_ann_prune.path,
        monitor_path: runtime.monitor.path,
        latest_launchd_activity_at: runtime.monitor.summary.latest_launchd_activity_at,
        next_expected_run_after: runtime.monitor.summary.next_expected_run_after,
        natural_run_due: runtime.monitor.summary.natural_run_due,
        natural_run_overdue: runtime.monitor.summary.natural_run_overdue,
        seconds_overdue: runtime.monitor.summary.seconds_overdue,
    }
}

fn runtime_semantic_readiness_to_panel(
    runtime: AgentContextRuntimeSemanticReadiness,
) -> AgentContextPanelSemanticReadiness {
    AgentContextPanelSemanticReadiness {
        exists: runtime.exists,
        status: non_empty_or_default(&runtime.status, || "not_checked".to_string()),
        ready: runtime.ready,
        reason: runtime.summary.reason,
        semantic_chunks: runtime.summary.semantic_chunks,
        trend_days_observed: runtime.summary.trend_days_observed,
        trend_days_remaining: runtime.summary.trend_days_remaining,
        monitor_snapshots: runtime.summary.monitor_snapshots,
        next_monitor_due_at: runtime.summary.next_monitor_due_at,
        earliest_multi_day_check_after: runtime.summary.earliest_multi_day_check_after,
        next_action: runtime.next_action,
        report_path: runtime.path,
        report_markdown_path: runtime.latest_md_path,
    }
}

fn runtime_v1_acceptance_to_panel(
    runtime: AgentContextRuntimeV1Acceptance,
) -> AgentContextPanelV1Acceptance {
    let followup_json_path = non_empty_or_default(&runtime.latest_followup_json_path, || {
        runtime.followup_plan.latest_json_path.clone()
    });
    let followup_markdown_path = non_empty_or_default(&runtime.latest_followup_md_path, || {
        runtime.followup_plan.latest_md_path.clone()
    });
    let next_evidence_gate_reason =
        non_empty_or_default(&runtime.followup_check.next_evidence_gate_reason, || {
            runtime.followup_check.wait_reason.clone()
        });
    let next_evidence_gate_at =
        non_empty_or_default(&runtime.followup_check.next_evidence_gate_at, || {
            runtime.followup_check.next_gate_at.clone()
        });
    let seconds_until_next_evidence_gate =
        if runtime.followup_check.seconds_until_next_evidence_gate > 0 {
            runtime.followup_check.seconds_until_next_evidence_gate
        } else {
            runtime.followup_check.seconds_until_next_gate
        };
    AgentContextPanelV1Acceptance {
        exists: runtime.exists,
        status: non_empty_or_default(&runtime.status, || "not_checked".to_string()),
        ready: runtime.ready,
        decision: runtime.decision,
        created_at: runtime.created_at,
        report_path: runtime.path,
        report_markdown_path: runtime.latest_md_path,
        next_commands: runtime.next_commands,
        followup_json_path,
        followup_markdown_path,
        can_recheck_now: runtime.followup_plan.can_recheck_now,
        earliest_recheck_after: runtime.followup_plan.earliest_recheck_after,
        next_monitor_due_at: runtime.followup_plan.next_monitor_due_at,
        trend_days_remaining: runtime.followup_plan.trend_days_remaining,
        wait_reason: runtime.followup_check.wait_reason,
        next_gate_at: runtime.followup_check.next_gate_at,
        seconds_until_next_gate: runtime.followup_check.seconds_until_next_gate,
        next_evidence_gate_reason,
        next_evidence_gate_at,
        seconds_until_next_evidence_gate,
        acceptance_wait_reason: runtime.followup_check.acceptance_wait_reason,
        acceptance_gate_at: runtime.followup_check.acceptance_gate_at,
        seconds_until_acceptance_gate: runtime.followup_check.seconds_until_acceptance_gate,
    }
}

fn runtime_v1_stage_status_to_panel(
    runtime: AgentContextRuntimeV1StageStatus,
) -> AgentContextPanelV1StageStatus {
    let next_evidence_gate_reason =
        non_empty_or_default(&runtime.next_gates.next_evidence_gate_reason, || {
            runtime.next_gates.wait_reason.clone()
        });
    let next_evidence_gate_at =
        non_empty_or_default(&runtime.next_gates.next_evidence_gate_at, || {
            runtime.next_gates.next_gate_at.clone()
        });
    let seconds_until_next_evidence_gate =
        if runtime.next_gates.seconds_until_next_evidence_gate > 0 {
            runtime.next_gates.seconds_until_next_evidence_gate
        } else {
            runtime.next_gates.seconds_until_next_gate
        };
    AgentContextPanelV1StageStatus {
        exists: runtime.exists,
        status: non_empty_or_default(&runtime.status, || "not_checked".to_string()),
        ready: runtime.ready,
        decision: runtime.decision,
        created_at: runtime.created_at,
        report_path: runtime.path,
        report_markdown_path: runtime.latest_md_path,
        stages_total: runtime.summary.stages_total,
        ok: runtime.summary.ok,
        waiting_for_time: runtime.summary.waiting_for_time,
        warning: runtime.summary.warning,
        failed: runtime.summary.failed,
        wait_reason: runtime.next_gates.wait_reason,
        next_gate_at: runtime.next_gates.next_gate_at,
        seconds_until_next_gate: runtime.next_gates.seconds_until_next_gate,
        next_evidence_gate_reason,
        next_evidence_gate_at,
        seconds_until_next_evidence_gate,
        acceptance_wait_reason: runtime.next_gates.acceptance_wait_reason,
        acceptance_gate_at: runtime.next_gates.acceptance_gate_at,
        seconds_until_acceptance_gate: runtime.next_gates.seconds_until_acceptance_gate,
        trend_days_remaining: runtime.next_gates.trend_days_remaining,
        stages: runtime
            .stages
            .into_iter()
            .map(|stage| AgentContextPanelV1StageRow {
                id: stage.id,
                title: stage.title,
                status: stage.status,
                progress: stage.progress,
                summary: stage.summary,
            })
            .collect(),
    }
}

fn runtime_access_audit_to_panel(
    runtime: AgentContextRuntimeAccessAudit,
) -> AgentContextPanelAccessAudit {
    let recent_events: Vec<AgentContextAccessAuditEvent> = runtime
        .recent_events
        .into_iter()
        .map(runtime_access_event_to_panel)
        .collect();
    let recent_allowed = *runtime.summary.decisions.get("allowed").unwrap_or(&0);
    let recent_denied = *runtime.summary.decisions.get("denied").unwrap_or(&0);
    let recent_filtered = *runtime.summary.decisions.get("filtered").unwrap_or(&0);
    let recent_consent_required = *runtime
        .summary
        .decisions
        .get("consent_required")
        .unwrap_or(&0);
    AgentContextPanelAccessAudit {
        audit_path: runtime.audit_path,
        events_total: runtime.events_total,
        recent_allowed,
        recent_denied,
        recent_filtered,
        recent_consent_required,
        recent_events,
        last_denied: runtime
            .summary
            .last_denied
            .map(runtime_access_event_to_panel),
    }
}

fn runtime_access_event_to_panel(
    event: AgentContextRuntimeAccessAuditEvent,
) -> AgentContextAccessAuditEvent {
    AgentContextAccessAuditEvent {
        created_at: event.created_at,
        action: event.action,
        decision: event.decision,
        identifier: event.identifier,
        provider: event.provider,
        source_id: event.source_id,
        source_chunk_id: event.source_chunk_id,
        path: event.path,
        reason: event.reason,
    }
}

fn runtime_preflight_to_task_preflight(
    runtime: AgentContextRuntimeCodexPreflight,
    config: &AgentContextPanelConfig,
    requested_goal: &str,
) -> AgentContextTaskPreflight {
    let runtime_status = non_empty_or_default(&runtime.status, || "ok".to_string());
    let status = normalize_task_preflight_status(&runtime_status, &runtime.stage);
    let goal = non_empty_or_default(&runtime.goal, || requested_goal.to_string());
    let scope = non_empty_or_default(&runtime.source_scope, || config.scope.clone());
    let mode = non_empty_or_default(&runtime.mode, || config.mode.clone());
    let message = task_preflight_message(
        &status,
        runtime.sources_included,
        &runtime.error,
        &runtime.stage,
        &runtime.review_file,
    );
    let agent_preflight_md = runtime
        .agent_preflight
        .as_ref()
        .map(|preflight| preflight.agent_preflight_md_path.clone())
        .unwrap_or_default();
    let review_launch_md = runtime
        .review_launch
        .as_ref()
        .map(|launch| launch.review_launch_md_path.clone())
        .unwrap_or_default();
    AgentContextTaskPreflight {
        status,
        message,
        goal,
        scope,
        mode,
        sources_included: runtime.sources_included,
        codex_preflight_md: runtime.preflight_markdown_path,
        model_input_md: String::new(),
        context_md: runtime.context_md_path.unwrap_or_default(),
        sources_jsonl: runtime.sources_jsonl_path.unwrap_or_default(),
        manifest_json: runtime.manifest_json_path.unwrap_or_default(),
        resolution_plan_json: runtime.resolution_plan_json_path.unwrap_or_default(),
        session_id: runtime.session_id,
        runtime_task_md: runtime.runtime_task_md_path,
        runtime_task_json: runtime.runtime_task_json_path,
        review_file: runtime.review_file,
        agent_preflight_md,
        review_launch_md,
        review_client_html: runtime.client_html_path,
        review_server_url: runtime.review_server_url,
        start_server_command: runtime.start_server_command,
        open_client_command: runtime.open_client_command,
    }
}

fn agent_preflight_to_task_preflight(
    runtime: AgentContextRuntimeAgentPreflight,
    config: &AgentContextPanelConfig,
    previous: &AgentContextPanelStatus,
) -> AgentContextTaskPreflight {
    let context_md = runtime.files.context_md_path.clone().unwrap_or_default();
    let sources_jsonl = runtime.files.sources_jsonl_path.clone().unwrap_or_default();
    let model_input_md = runtime
        .files
        .model_input_md_path
        .clone()
        .unwrap_or_default();
    let status = non_empty_or_default(&runtime.status, || "awaiting_context_review".to_string());
    let message = if model_input_md.trim().is_empty() {
        non_empty_or_default(&runtime.next_message, || {
            "Doctor 已推进到上下文审查，但没有返回 model_input.md。".to_string()
        })
    } else {
        "Doctor 已生成 model_input.md，请审查后再发给模型。".to_string()
    };
    AgentContextTaskPreflight {
        status,
        message,
        goal: previous.last_goal.clone(),
        scope: non_empty_or_default(&runtime.source_scope, || config.scope.clone()),
        mode: non_empty_or_default(&runtime.mode, || config.mode.clone()),
        sources_included: count_jsonl_rows(&sources_jsonl),
        codex_preflight_md: runtime.agent_preflight_md_path.clone(),
        model_input_md: model_input_md.clone(),
        context_md,
        sources_jsonl,
        manifest_json: String::new(),
        resolution_plan_json: String::new(),
        session_id: non_empty_or_default(&runtime.session_id, || previous.last_session_id.clone()),
        runtime_task_md: runtime.files.runtime_task_md_path.unwrap_or_default(),
        runtime_task_json: runtime.files.runtime_task_json_path.unwrap_or_default(),
        review_file: non_empty_or_default(&runtime.review_file, || model_input_md),
        agent_preflight_md: runtime.agent_preflight_md_path,
        review_launch_md: runtime
            .files
            .runtime_review_launch_md_path
            .unwrap_or_default(),
        review_client_html: runtime
            .files
            .runtime_review_client_html_path
            .unwrap_or_default(),
        review_server_url: String::new(),
        start_server_command: String::new(),
        open_client_command: String::new(),
    }
}

fn answer_preflight_to_bridge_value(raw: Value) -> Value {
    let status = string_at(&raw, "/status", "");
    let session_id = string_at(&raw, "/session_id", "");
    let null = Value::Null;
    let contract = raw.get("client_contract").unwrap_or(&null);
    let files = raw.get("files").unwrap_or(&null);
    let handoff = raw.get("agent_handoff").unwrap_or(&null);
    let action = raw.get("action_result").unwrap_or(&null);
    let approved_model_input = first_non_empty(&[
        string_at(contract, "/approved_model_input_md_path", ""),
        string_at(files, "/model_input_md_path", ""),
        string_at(handoff, "/model_input_md_path", ""),
    ]);
    let agent_handoff = first_non_empty(&[
        string_at(contract, "/agent_handoff_md_path", ""),
        string_at(files, "/agent_handoff_md_path", ""),
        string_at(handoff, "/agent_handoff_md_path", ""),
    ]);
    let answer_packet = first_non_empty(&[
        string_at(contract, "/answer_packet_md_path", ""),
        string_at(files, "/answer_packet_md_path", ""),
        string_at(action, "/answer_packet_md_path", ""),
        string_at(handoff, "/answer_packet_md_path", ""),
    ]);
    let answer_md = first_non_empty(&[
        string_at(contract, "/answer_md_path", ""),
        string_at(files, "/answer_md_path", ""),
        string_at(action, "/answer_md_path", ""),
    ]);
    let message = first_non_empty(&[
        string_at(&raw, "/next_message", ""),
        string_at(contract, "/instruction", ""),
        "Use answer_packet.md with the approved Doctor context, then record and review the answer."
            .to_string(),
    ]);
    json!({
        "status": status,
        "message": message,
        "sessionId": session_id,
        "safeToSendModel": contract.get("safe_to_send_model").and_then(Value::as_bool).unwrap_or(false),
        "approvedModelInputMd": approved_model_input,
        "agentHandoffMd": agent_handoff,
        "answerPacketMd": answer_packet,
        "answerMd": answer_md,
        "contextMd": string_at(files, "/context_md_path", ""),
        "sourcesJsonl": string_at(files, "/sources_jsonl_path", ""),
        "reviewFile": string_at(&raw, "/review_file", ""),
        "agentPreflightMd": string_at(&raw, "/agent_preflight_md_path", ""),
        "nextCommands": raw.get("next_commands").cloned().unwrap_or_else(|| json!([])),
    })
}

fn execution_preflight_to_bridge_value(raw: Value, recorded_answer_file: PathBuf) -> Value {
    let status = string_at(&raw, "/status", "");
    let session_id = string_at(&raw, "/session_id", "");
    let null = Value::Null;
    let contract = raw.get("client_contract").unwrap_or(&null);
    let files = raw.get("files").unwrap_or(&null);
    let action = raw.get("action_result").unwrap_or(&null);
    let execution_report = first_non_empty(&[
        string_at(contract, "/execution_report_md_path", ""),
        string_at(files, "/execution_report_md_path", ""),
        string_at(action, "/execution_report_md_path", ""),
    ]);
    let artifact_index = first_non_empty(&[
        string_at(contract, "/execution_artifact_index_md_path", ""),
        string_at(files, "/execution_artifact_index_md_path", ""),
        string_at(action, "/artifact_index_md_path", ""),
    ]);
    let execution_review = first_non_empty(&[
        string_at(files, "/execution_review_json_path", ""),
        string_at(action, "/execution_review_json_path", ""),
    ]);
    let message = first_non_empty(&[
        string_at(&raw, "/next_message", ""),
        string_at(contract, "/instruction", ""),
        "Doctor has recorded and approved the answer; execution review is ready. Do not run local commands until the user approves an execution command.".to_string(),
    ]);
    json!({
        "status": status,
        "message": message,
        "sessionId": session_id,
        "safeToSendModel": false,
        "recordedAnswerFile": recorded_answer_file.to_string_lossy().to_string(),
        "answerMd": string_at(files, "/answer_md_path", ""),
        "executionReviewJson": execution_review,
        "executionReportMd": execution_report,
        "executionArtifactsJsonl": string_at(files, "/execution_artifact_manifest_jsonl_path", ""),
        "executionArtifactIndexMd": artifact_index,
        "artifactsDir": string_at(files, "/artifacts_dir", ""),
        "reviewFile": string_at(&raw, "/review_file", ""),
        "agentPreflightMd": string_at(&raw, "/agent_preflight_md_path", ""),
        "nextCommands": raw.get("next_commands").cloned().unwrap_or_else(|| json!([])),
    })
}

fn execution_review_to_bridge_value(raw: Value, default_message: &str) -> Value {
    let null = Value::Null;
    let commands = raw
        .get("commands")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let last_command = commands.last().unwrap_or(&null);
    let status = string_at(&raw, "/status", "");
    let execution_report = string_at(&raw, "/execution_report_md_path", "");
    let artifact_index = string_at(&raw, "/artifact_index_md_path", "");
    json!({
        "status": status,
        "message": default_message,
        "sessionId": string_at(&raw, "/session_id", ""),
        "safeToSendModel": false,
        "complete": status == "approved",
        "command": string_at(last_command, "/command", ""),
        "cwd": string_at(last_command, "/cwd", ""),
        "lastRunId": string_at(&raw, "/last_run_id", ""),
        "lastReturncode": last_command.get("returncode").cloned().unwrap_or_else(|| raw.get("last_returncode").cloned().unwrap_or(Value::Null)),
        "lastTimedOut": last_command.get("timed_out").and_then(Value::as_bool).unwrap_or_else(|| raw.get("last_timed_out").and_then(Value::as_bool).unwrap_or(false)),
        "stdoutPath": string_at(last_command, "/stdout_path", ""),
        "stderrPath": string_at(last_command, "/stderr_path", ""),
        "resultJsonPath": string_at(last_command, "/result_json_path", ""),
        "executionReviewJson": string_at(&raw, "/execution_review_json_path", ""),
        "executionReportMd": execution_report,
        "executionArtifactsJsonl": string_at(&raw, "/artifact_manifest_jsonl_path", ""),
        "executionArtifactIndexMd": artifact_index,
        "artifactsDir": string_at(&raw, "/artifacts_dir", ""),
        "reviewFile": execution_report,
        "artifactCount": raw.get("artifact_count").and_then(Value::as_u64).unwrap_or(0),
        "commands": raw.get("commands").cloned().unwrap_or_else(|| json!([])),
        "externalArtifacts": raw.get("external_artifacts").cloned().unwrap_or_else(|| json!([])),
        "nextCommands": json!([]),
    })
}

fn write_answer_review_bridge_file(session_id: &str, answer_text: &str) -> anyhow::Result<PathBuf> {
    let dir = crate::paths::default_app_state_dir().join("agent-context-answer-review");
    fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;
    let filename = format!("{}-answer.md", sanitize_filename_component(session_id));
    let path = dir.join(filename);
    fs::write(&path, answer_text).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
}

fn sanitize_filename_component(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();
    let trimmed = sanitized.trim_matches('-');
    if trimmed.is_empty() {
        "doctor-session".to_string()
    } else {
        trimmed.to_string()
    }
}

fn first_non_empty(values: &[String]) -> String {
    values
        .iter()
        .find(|value| !value.trim().is_empty())
        .cloned()
        .unwrap_or_default()
}

fn string_at(value: &Value, pointer: &str, fallback: &str) -> String {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn normalize_task_preflight_status(status: &str, stage: &str) -> String {
    if status == "awaiting_context_generation" || stage == "clarify_review" {
        "ok".to_string()
    } else {
        status.to_string()
    }
}

fn count_jsonl_rows(path: &str) -> usize {
    if path.trim().is_empty() {
        return 0;
    }
    fs::read_to_string(path)
        .map(|contents| {
            contents
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count()
        })
        .unwrap_or(0)
}

fn task_preflight_message(
    status: &str,
    sources_included: usize,
    error: &str,
    stage: &str,
    review_file: &str,
) -> String {
    match status {
        "ok" | "awaiting_context_generation" if !review_file.trim().is_empty() => {
            "Doctor 已启动第一阶段审查，请先确认 refined_prompt.md，再生成上下文。".to_string()
        }
        "ok" | "awaiting_context_generation" if stage == "clarify_review" => {
            "Doctor 已启动第一阶段审查，请先确认需求归一化。".to_string()
        }
        "ok" => format!("任务预检已生成，包含 {sources_included} 条来源。"),
        "disabled" => "Auto Context 已关闭，任务预检只生成禁用说明。".to_string(),
        "resolver_failed" => {
            if error.trim().is_empty() {
                "任务预检失败：resolver 未生成上下文包。".to_string()
            } else {
                format!("任务预检失败：{}", error.trim())
            }
        }
        other => format!("任务预检状态：{other}。"),
    }
}

fn save_task_preflight_status(preflight: &AgentContextTaskPreflight) -> anyhow::Result<()> {
    let previous = load_agent_context_panel_status().unwrap_or_default();
    save_agent_context_panel_status(&AgentContextPanelStatus {
        last_status: preflight.status.clone(),
        last_message: preflight.message.clone(),
        last_session_id: if preflight.session_id.is_empty() {
            previous.last_session_id
        } else {
            preflight.session_id.clone()
        },
        last_goal: preflight.goal.clone(),
        last_scope: preflight.scope.clone(),
        last_mode: preflight.mode.clone(),
        last_generated_pack: if preflight.context_md.is_empty() {
            previous.last_generated_pack
        } else {
            preflight.context_md.clone()
        },
        last_sources_jsonl: if preflight.sources_jsonl.is_empty() {
            previous.last_sources_jsonl
        } else {
            preflight.sources_jsonl.clone()
        },
        last_manifest_json: if preflight.manifest_json.is_empty() {
            previous.last_manifest_json
        } else {
            preflight.manifest_json.clone()
        },
        last_resolution_plan_json: if preflight.resolution_plan_json.is_empty() {
            previous.last_resolution_plan_json
        } else {
            preflight.resolution_plan_json.clone()
        },
        last_codex_preflight_md: if !preflight.codex_preflight_md.is_empty() {
            preflight.codex_preflight_md.clone()
        } else if !preflight.review_file.is_empty() {
            preflight.review_file.clone()
        } else if !preflight.agent_preflight_md.is_empty() {
            preflight.agent_preflight_md.clone()
        } else {
            previous.last_codex_preflight_md
        },
        last_model_input_md: if preflight.model_input_md.is_empty() {
            previous.last_model_input_md
        } else {
            preflight.model_input_md.clone()
        },
        last_runtime_task_md: if preflight.runtime_task_md.is_empty() {
            previous.last_runtime_task_md
        } else {
            preflight.runtime_task_md.clone()
        },
        last_review_file: if preflight.review_file.is_empty() {
            previous.last_review_file
        } else {
            preflight.review_file.clone()
        },
        last_review_client_html: if preflight.review_client_html.is_empty() {
            previous.last_review_client_html
        } else {
            preflight.review_client_html.clone()
        },
        last_review_launch_md: if preflight.review_launch_md.is_empty() {
            previous.last_review_launch_md
        } else {
            preflight.review_launch_md.clone()
        },
        access_audit: previous.access_audit,
        semantic_launchd: previous.semantic_launchd,
        semantic_readiness: previous.semantic_readiness,
        v1_acceptance: previous.v1_acceptance,
        v1_stage_status: previous.v1_stage_status,
        feedback_replay_trend: previous.feedback_replay_trend,
        last_generated_at_ms: now_ms(),
    })?;
    Ok(())
}

fn runtime_status_message(runtime: &AgentContextRuntimePanelStatus, status: &str) -> String {
    if status == "ok" {
        if let Some(sources) = runtime
            .preflight
            .as_ref()
            .and_then(|preflight| preflight.sources_included)
        {
            return format!("已生成上下文包，包含 {sources} 条来源。");
        }
        return "Agent Context 面板状态已刷新。".to_string();
    }
    if !runtime.auto_context {
        return "Auto Context 已关闭，只刷新面板状态。".to_string();
    }
    "Agent Context 未生成新的上下文包。".to_string()
}

fn save_agent_context_panel_status(status: &AgentContextPanelStatus) -> anyhow::Result<()> {
    let path = crate::paths::default_agent_context_panel_status_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(status)?;
    fs::write(&path, bytes).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn save_launch_prewarm_status(
    config: &AgentContextPanelConfig,
    goal: &str,
    status: &str,
    message: &str,
) -> anyhow::Result<()> {
    let previous = load_agent_context_panel_status().unwrap_or_default();
    let last_goal = if goal.trim().is_empty() {
        previous.last_goal.clone()
    } else {
        goal.trim().to_string()
    };
    save_agent_context_panel_status(&AgentContextPanelStatus {
        last_status: status.to_string(),
        last_message: message.to_string(),
        last_goal,
        last_scope: config.scope.clone(),
        last_mode: config.mode.clone(),
        last_generated_at_ms: now_ms(),
        ..previous
    })
}

fn command_failure_message(output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        format!("agent-context exited with status {}", output.status)
    }
}

pub fn save_agent_context_panel_config(
    config: &AgentContextPanelConfig,
) -> anyhow::Result<AgentContextPanelState> {
    let config = normalize_config(config.clone());
    let path = crate::paths::default_agent_context_panel_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(&config)?;
    fs::write(&path, bytes).with_context(|| format!("failed to write {}", path.display()))?;
    load_agent_context_panel_state()
}

pub fn record_agent_context_feedback(
    feedback: AgentContextFeedbackEntry,
) -> anyhow::Result<PathBuf> {
    let path = crate::paths::default_agent_context_feedback_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let entry = AgentContextFeedbackEntry {
        created_at_ms: if feedback.created_at_ms == 0 {
            now_ms()
        } else {
            feedback.created_at_ms
        },
        status_path: if feedback.status_path.trim().is_empty() {
            status_path_string()
        } else {
            feedback.status_path.trim().to_string()
        },
        winner: feedback.winner.trim().to_string(),
        reason: feedback.reason.trim().to_string(),
    };
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("failed to open {}", path.display()))?;
    writeln!(file, "{}", serde_json::to_string(&entry)?)
        .with_context(|| format!("failed to append {}", path.display()))?;
    Ok(path)
}

fn load_agent_context_panel_config() -> anyhow::Result<AgentContextPanelConfig> {
    let path = crate::paths::default_agent_context_panel_config_path();
    if !path.exists() {
        return Ok(AgentContextPanelConfig::default());
    }
    let text =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let config: AgentContextPanelConfig = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(normalize_config(config))
}

fn load_agent_context_panel_status() -> anyhow::Result<AgentContextPanelStatus> {
    let path = crate::paths::default_agent_context_panel_status_path();
    if !path.exists() {
        return Ok(AgentContextPanelStatus::default());
    }
    let text =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let status: AgentContextPanelStatus = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(status)
}

fn normalize_config(mut config: AgentContextPanelConfig) -> AgentContextPanelConfig {
    config.scope = match config.scope.trim() {
        "downloads" | "gitProjects" | "codexSessions" | "agentSessions" | "workflowDocs"
        | "all" => config.scope.trim().to_string(),
        _ => default_scope(),
    };
    config.mode = match config.mode.trim() {
        "fast" | "deep" | "arena" => config.mode.trim().to_string(),
        _ => default_mode(),
    };
    config.agent_context_root =
        non_empty_or_default(&config.agent_context_root, default_agent_context_root);
    config.agent_context_bin =
        non_empty_or_default(&config.agent_context_bin, default_agent_context_bin);
    config
}

fn non_empty_or_default<F>(value: &str, default_value: F) -> String
where
    F: FnOnce() -> String,
{
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default_value()
    } else {
        trimmed.to_string()
    }
}

fn format_float(value: f64) -> String {
    if !value.is_finite() {
        return String::new();
    }
    if value.fract().abs() < f64::EPSILON {
        return format!("{value:.1}");
    }
    let formatted = format!("{value:.6}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

fn default_scope() -> String {
    "all".to_string()
}

fn default_mode() -> String {
    "fast".to_string()
}

fn default_auto_context() -> bool {
    PathBuf::from(default_agent_context_bin()).exists()
}

fn default_status() -> String {
    "not_checked".to_string()
}

fn default_agent_context_root() -> String {
    home_dir()
        .join("agent-context-system")
        .to_string_lossy()
        .to_string()
}

fn default_agent_context_bin() -> String {
    let file = if cfg!(windows) {
        "agent-context.exe"
    } else {
        "agent-context"
    };
    home_dir()
        .join("agent-context-system")
        .join(file)
        .to_string_lossy()
        .to_string()
}

fn home_dir() -> PathBuf {
    directories::BaseDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn config_path_string() -> String {
    crate::paths::default_agent_context_panel_config_path()
        .to_string_lossy()
        .to_string()
}

fn status_path_string() -> String {
    crate::paths::default_agent_context_panel_status_path()
        .to_string_lossy()
        .to_string()
}

fn feedback_path_string() -> String {
    crate::paths::default_agent_context_feedback_path()
        .to_string_lossy()
        .to_string()
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_uses_smart_auto_context() {
        let config = AgentContextPanelConfig::default();

        assert_eq!(config.auto_context, default_auto_context());
        assert_eq!(config.scope, "all");
        assert_eq!(config.mode, "fast");
    }

    #[test]
    fn normalize_config_clamps_unknown_scope_and_mode() {
        let config = normalize_config(AgentContextPanelConfig {
            auto_context: true,
            scope: "unknown".to_string(),
            mode: "slow".to_string(),
            agent_context_root: "  ".to_string(),
            agent_context_bin: "  ".to_string(),
        });

        assert!(config.auto_context);
        assert_eq!(config.scope, "all");
        assert_eq!(config.mode, "fast");
        assert!(!config.agent_context_root.trim().is_empty());
        assert!(!config.agent_context_bin.trim().is_empty());
    }

    #[test]
    fn normalize_config_accepts_all_runtime_scopes() {
        for scope in [
            "downloads",
            "gitProjects",
            "codexSessions",
            "agentSessions",
            "workflowDocs",
            "all",
        ] {
            let config = normalize_config(AgentContextPanelConfig {
                auto_context: true,
                scope: scope.to_string(),
                mode: "fast".to_string(),
                agent_context_root: "/tmp/agent-context-system".to_string(),
                agent_context_bin: "/tmp/agent-context".to_string(),
            });
            assert_eq!(config.scope, scope);
        }
    }

    #[test]
    fn access_policy_command_args_include_patch_values() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };
        let patch = AgentContextAccessPolicyPatch {
            allow_providers: vec!["custom_provider".to_string()],
            remove_allow_providers: vec![],
            deny_providers: vec!["claude_session".to_string()],
            remove_deny_providers: vec!["old_provider".to_string()],
            deny_path_patterns: vec!["*/Secrets/*".to_string()],
            remove_deny_path_patterns: vec!["*.tmp".to_string()],
            require_consent_providers: vec!["codex_session".to_string()],
            remove_require_consent_providers: vec!["legacy_session".to_string()],
            require_consent_path_patterns: vec!["*/PrivateNotes/*".to_string()],
            remove_require_consent_path_patterns: vec!["*/OldNotes/*".to_string()],
            audit_max_bytes: Some(1234),
            audit_max_rotated_files: Some(4),
        };

        let args = access_policy_command_args(&config, &patch);

        assert_eq!(args[0], "access-policy");
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--out", "/tmp/agent-context-system"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--allow-provider", "custom_provider"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--deny-provider", "claude_session"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--remove-deny-provider", "old_provider"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--deny-path", "*/Secrets/*"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--remove-deny-path", "*.tmp"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--require-consent-provider", "codex_session"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--remove-require-consent-provider", "legacy_session"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--require-consent-path", "*/PrivateNotes/*"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--remove-require-consent-path", "*/OldNotes/*"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--audit-max-bytes", "1234"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--audit-max-rotated-files", "4"])
        );
    }

    #[test]
    fn access_policy_output_maps_snake_case_contract() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };
        let output: AgentContextAccessPolicyCommandOutput = serde_json::from_str(
            r#"{
                "policy_path": "/tmp/access_policy.json",
                "updated": true,
                "changes": ["add:deny_path_patterns:*/Secrets/*"],
                "policy": {
                    "allow_providers": ["git_project"],
                    "deny_providers": ["claude_session"],
                    "deny_path_patterns": ["*/Secrets/*"],
                    "require_consent_providers": ["codex_session"],
                    "require_consent_path_patterns": ["*/PrivateNotes/*"],
                    "audit_max_bytes": 1234,
                    "audit_max_rotated_files": 4
                }
            }"#,
        )
        .unwrap();

        let state = access_policy_output_to_state(output, &config);

        assert_eq!(state.policy_path, "/tmp/access_policy.json");
        assert!(state.updated);
        assert_eq!(state.changes.len(), 1);
        assert_eq!(state.policy.allow_providers, vec!["git_project"]);
        assert_eq!(state.policy.deny_providers, vec!["claude_session"]);
        assert_eq!(state.policy.deny_path_patterns, vec!["*/Secrets/*"]);
        assert_eq!(
            state.policy.require_consent_providers,
            vec!["codex_session"]
        );
        assert_eq!(
            state.policy.require_consent_path_patterns,
            vec!["*/PrivateNotes/*"]
        );
        assert_eq!(state.policy.audit_max_bytes, 1234);
        assert_eq!(state.policy.audit_max_rotated_files, 4);
    }

    #[test]
    fn access_consent_command_args_include_identifier_and_reason() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };

        let args =
            access_consent_command_args(&config, " project:sensitive ", " approved from panel ");

        assert_eq!(args[0], "access-consent");
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--out", "/tmp/agent-context-system"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--identifier", "project:sensitive"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--reason", "approved from panel"])
        );
    }

    #[test]
    fn access_consent_output_maps_snake_case_contract() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };
        let output: AgentContextAccessConsentCommandOutput = serde_json::from_str(
            r#"{
                "consent_path": "/tmp/access_consent.json",
                "written": true,
                "grants_total": 2,
                "grant": {
                    "created_at": "2026-06-16T00:00:00+08:00",
                    "key": "source_id:project:sensitive",
                    "identifier": "project:sensitive",
                    "reason": "approved from panel",
                    "provider": "git_project",
                    "source_id": "project:sensitive",
                    "source_chunk_id": "",
                    "path": "/tmp/project",
                    "relative_path": "project"
                }
            }"#,
        )
        .unwrap();

        let state = access_consent_output_to_state(output, &config);

        assert_eq!(state.consent_path, "/tmp/access_consent.json");
        assert!(state.written);
        assert_eq!(state.grants_total, 2);
        assert_eq!(state.grant.identifier, "project:sensitive");
        assert_eq!(state.grant.provider, "git_project");
        assert_eq!(state.grant.source_id, "project:sensitive");
    }

    #[test]
    fn v1_refresh_command_args_use_unified_refresh_entrypoint() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };

        let args = v1_refresh_command_args(&config);

        assert_eq!(args[0], "v1-refresh");
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--out", "/tmp/agent-context-system"])
        );
        assert!(
            args.iter()
                .any(|arg| arg == "--with-manager-feedback-smoke")
        );
    }

    #[test]
    fn v1_followup_output_maps_snake_case_contract() {
        let output: AgentContextV1FollowupResult = serde_json::from_str(
            r#"{
                "status": "waiting_for_time",
                "action": "wait",
                "ready": false,
                "can_recheck_now": false,
                "earliest_recheck_after": "2026-06-17T00:00:00+08:00",
                "next_monitor_due_at": "2026-06-16T12:41:23+08:00",
                "trend_days_remaining": 1,
                "wait_reason": "monitor_not_due",
                "next_gate_at": "2026-06-16T12:41:23+08:00",
                "seconds_until_next_gate": 1000,
                "next_evidence_gate_reason": "monitor_not_due",
                "next_evidence_gate_at": "2026-06-16T12:41:23+08:00",
                "seconds_until_next_evidence_gate": 1000,
                "acceptance_wait_reason": "multi_day_not_due",
                "acceptance_gate_at": "2026-06-17T00:00:00+08:00",
                "seconds_until_acceptance_gate": 40920,
                "followup_plan_latest_md_path": "/tmp/reports/v1-followup-latest.md",
                "acceptance_latest_md_path": "/tmp/reports/v1-acceptance-latest.md",
                "latest_md_path": "/tmp/reports/v1-followup-check-latest.md",
                "latest_json_path": "/tmp/reports/v1-followup-check-latest.json",
                "next_command": "agent-context v1-acceptance --refresh-evidence"
            }"#,
        )
        .unwrap();

        assert_eq!(output.status, "waiting_for_time");
        assert_eq!(output.action, "wait");
        assert!(!output.can_recheck_now);
        assert_eq!(output.trend_days_remaining, 1);
        assert_eq!(output.wait_reason, "monitor_not_due");
        assert_eq!(output.seconds_until_next_gate, 1000);
        assert_eq!(output.next_evidence_gate_reason, "monitor_not_due");
        assert_eq!(output.next_evidence_gate_at, "2026-06-16T12:41:23+08:00");
        assert_eq!(output.seconds_until_next_evidence_gate, 1000);
        assert_eq!(output.acceptance_wait_reason, "multi_day_not_due");
        assert_eq!(output.acceptance_gate_at, "2026-06-17T00:00:00+08:00");
        assert!(
            output
                .latest_md_path
                .ends_with("v1-followup-check-latest.md")
        );
    }

    #[test]
    fn v1_refresh_output_maps_nested_followup_check_contract() {
        let output = parse_v1_refresh_or_followup_output(
            r#"{
                "status": "waiting_for_time",
                "ready": false,
                "semantic_evidence": {
                    "refreshed": false,
                    "reason": "monitor_not_due"
                },
                "followup_check": {
                    "status": "waiting_for_time",
                    "action": "wait",
                    "ready": false,
                    "can_recheck_now": false,
                    "wait_reason": "monitor_not_due",
                    "next_gate_at": "2026-06-16T15:41:57+08:00",
                    "seconds_until_next_gate": 1200,
                    "acceptance_wait_reason": "multi_day_not_due",
                    "acceptance_gate_at": "2026-06-17T00:00:00+08:00",
                    "latest_md_path": "/tmp/reports/v1-followup-check-latest.md",
                    "latest_json_path": "/tmp/reports/v1-followup-check-latest.json",
                    "next_command": "agent-context v1-refresh --out /tmp/agent-context-system"
                }
            }"#,
        )
        .unwrap();

        assert_eq!(output.status, "waiting_for_time");
        assert_eq!(output.action, "wait");
        assert_eq!(output.wait_reason, "monitor_not_due");
        assert_eq!(output.next_gate_at, "2026-06-16T15:41:57+08:00");
        assert_eq!(output.seconds_until_next_gate, 1200);
        assert_eq!(output.acceptance_gate_at, "2026-06-17T00:00:00+08:00");
        assert!(output.next_command.starts_with("agent-context v1-refresh"));
    }

    #[test]
    fn runtime_status_maps_generated_pack_paths() {
        let config = AgentContextPanelConfig::default();
        let status = runtime_status_to_panel_status(
            AgentContextRuntimePanelStatus {
                auto_context: true,
                goal: Some("研究本地推荐系统".to_string()),
                scope: "gitProjects".to_string(),
                mode: "fast".to_string(),
                preflight: Some(AgentContextRuntimePreflight {
                    status: Some("ok".to_string()),
                    sources_included: Some(8),
                }),
                last_generated_pack: Some("/tmp/context.md".to_string()),
                last_sources_jsonl: Some("/tmp/sources.jsonl".to_string()),
                last_manifest_json: Some("/tmp/manifest.json".to_string()),
                last_resolution_plan_json: Some("/tmp/resolution_plan.json".to_string()),
                last_codex_preflight_md: Some("/tmp/codex_preflight.md".to_string()),
                last_runtime_task_md: Some("/tmp/runtime_task.md".to_string()),
                last_review_file: Some("/tmp/refined_prompt.md".to_string()),
                last_review_client_html: Some("/tmp/doctor-runtime-review-client.html".to_string()),
                last_review_launch_md: Some("/tmp/review_launch.md".to_string()),
                access_audit: Some(AgentContextRuntimeAccessAudit {
                    audit_path: "/tmp/access_audit.jsonl".to_string(),
                    events_total: 3,
                    recent_events: vec![AgentContextRuntimeAccessAuditEvent {
                        created_at: "2026-06-16T00:00:00Z".to_string(),
                        action: "mcp_read_provider".to_string(),
                        decision: "denied".to_string(),
                        identifier: "project:secret".to_string(),
                        provider: "git_project".to_string(),
                        source_id: "project:secret".to_string(),
                        source_chunk_id: String::new(),
                        path: "/tmp/secret".to_string(),
                        reason: "path_denied:secret".to_string(),
                    }],
                    summary: AgentContextRuntimeAccessAuditSummary {
                        decisions: std::collections::BTreeMap::from([
                            ("denied".to_string(), 1),
                            ("consent_required".to_string(), 1),
                        ]),
                        last_denied: Some(AgentContextRuntimeAccessAuditEvent {
                            created_at: "2026-06-16T00:00:00Z".to_string(),
                            action: "mcp_read_provider".to_string(),
                            decision: "denied".to_string(),
                            identifier: "project:secret".to_string(),
                            provider: "git_project".to_string(),
                            source_id: "project:secret".to_string(),
                            source_chunk_id: String::new(),
                            path: "/tmp/secret".to_string(),
                            reason: "path_denied:secret".to_string(),
                        }),
                    },
                }),
                semantic_launchd: Some(AgentContextRuntimeSemanticLaunchd {
                    health: "not_installed".to_string(),
                    installed: false,
                    plist_path:
                        "/tmp/LaunchAgents/com.gengrf.agent-context.semantic-maintenance.plist"
                            .to_string(),
                    script_path:
                        "/tmp/out/scripts/com.gengrf.agent-context.semantic-maintenance.sh"
                            .to_string(),
                    reports: AgentContextRuntimeSemanticLaunchdReports {
                        semantic_maintain: AgentContextRuntimeSemanticLaunchdReport {
                            path: "/tmp/reports/semantic-maintain.json".to_string(),
                        },
                        semantic_ann_prune: AgentContextRuntimeSemanticLaunchdReport {
                            path: "/tmp/reports/semantic-ann-prune.json".to_string(),
                        },
                    },
                    monitor: AgentContextRuntimeSemanticLaunchdMonitor {
                        path: "/tmp/reports/semantic-launchd-monitor-latest.json".to_string(),
                        summary: AgentContextRuntimeSemanticLaunchdMonitorSummary {
                            latest_launchd_activity_at: "2026-06-16T05:39:52+08:00".to_string(),
                            next_expected_run_after: "2026-06-16T06:39:52+08:00".to_string(),
                            natural_run_due: false,
                            natural_run_overdue: false,
                            seconds_overdue: 0,
                        },
                    },
                }),
                semantic_readiness: Some(AgentContextRuntimeSemanticReadiness {
                    exists: true,
                    path: "/tmp/reports/semantic-readiness-latest.json".to_string(),
                    latest_md_path: "/tmp/reports/semantic-readiness-latest.md".to_string(),
                    status: "waiting_for_time".to_string(),
                    ready: false,
                    next_action: "Need 1 more observed day.".to_string(),
                    summary: AgentContextRuntimeSemanticReadinessSummary {
                        reason: "healthy_but_short_window".to_string(),
                        semantic_chunks: 666,
                        trend_days_observed: 1,
                        trend_days_remaining: 1,
                        monitor_snapshots: 56,
                        next_monitor_due_at: "2026-06-16T11:40:58+08:00".to_string(),
                        earliest_multi_day_check_after: "2026-06-17T00:00:00+08:00".to_string(),
                    },
                }),
                v1_acceptance: Some(AgentContextRuntimeV1Acceptance {
                    exists: true,
                    path: "/tmp/reports/v1-acceptance-latest.json".to_string(),
                    latest_md_path: "/tmp/reports/v1-acceptance-latest.md".to_string(),
                    latest_followup_json_path: "/tmp/reports/v1-followup-latest.json".to_string(),
                    latest_followup_md_path: "/tmp/reports/v1-followup-latest.md".to_string(),
                    status: "waiting_for_time".to_string(),
                    ready: false,
                    decision:
                        "Implementation evidence is present, but final v1 acceptance is time-gated."
                            .to_string(),
                    created_at: "2026-06-16T11:12:11+08:00".to_string(),
                    followup_plan: AgentContextRuntimeV1FollowupPlan {
                        can_recheck_now: false,
                        earliest_recheck_after: "2026-06-17T00:00:00+08:00".to_string(),
                        next_monitor_due_at: "2026-06-16T12:41:23+08:00".to_string(),
                        trend_days_remaining: 1,
                        latest_json_path: "/tmp/reports/v1-followup-latest.json".to_string(),
                        latest_md_path: "/tmp/reports/v1-followup-latest.md".to_string(),
                    },
                    followup_check: AgentContextRuntimeV1FollowupCheck {
                        wait_reason: "monitor_not_due".to_string(),
                        next_gate_at: "2026-06-16T12:41:23+08:00".to_string(),
                        seconds_until_next_gate: 1000,
                        next_evidence_gate_reason: "monitor_not_due".to_string(),
                        next_evidence_gate_at: "2026-06-16T12:41:23+08:00".to_string(),
                        seconds_until_next_evidence_gate: 1000,
                        acceptance_wait_reason: "multi_day_not_due".to_string(),
                        acceptance_gate_at: "2026-06-17T00:00:00+08:00".to_string(),
                        seconds_until_acceptance_gate: 40920,
                    },
                    next_commands: vec![
                        "agent-context semantic-readiness --out /tmp/out".to_string(),
                        "agent-context v1-acceptance --out /tmp/out".to_string(),
                    ],
                }),
                v1_stage_status: Some(AgentContextRuntimeV1StageStatus {
                    exists: true,
                    path: "/tmp/reports/v1-stage-status-latest.json".to_string(),
                    latest_md_path: "/tmp/reports/v1-stage-status-latest.md".to_string(),
                    status: "waiting_for_time".to_string(),
                    ready: false,
                    decision: "Implementation evidence is present, but final gate is time-gated."
                        .to_string(),
                    created_at: "2026-06-16T13:05:25+08:00".to_string(),
                    summary: AgentContextRuntimeV1StageSummary {
                        stages_total: 10,
                        ok: 8,
                        waiting_for_time: 2,
                        warning: 0,
                        failed: 0,
                    },
                    next_gates: AgentContextRuntimeV1StageGates {
                        wait_reason: "monitor_not_due".to_string(),
                        next_gate_at: "2026-06-16T13:41:35+08:00".to_string(),
                        seconds_until_next_gate: 1000,
                        next_evidence_gate_reason: "monitor_not_due".to_string(),
                        next_evidence_gate_at: "2026-06-16T13:41:35+08:00".to_string(),
                        seconds_until_next_evidence_gate: 1000,
                        acceptance_wait_reason: "multi_day_not_due".to_string(),
                        acceptance_gate_at: "2026-06-17T00:00:00+08:00".to_string(),
                        seconds_until_acceptance_gate: 40920,
                        trend_days_remaining: 1,
                    },
                    stages: vec![
                        AgentContextRuntimeV1StageRow {
                            id: "downloads_ingestion".to_string(),
                            title: "Downloads ingestion".to_string(),
                            status: "ok".to_string(),
                            progress: 100,
                            summary: "997 documents".to_string(),
                        },
                        AgentContextRuntimeV1StageRow {
                            id: "semantic_background".to_string(),
                            title: "Background semantic index".to_string(),
                            status: "waiting_for_time".to_string(),
                            progress: 90,
                            summary: "days=1/2".to_string(),
                        },
                    ],
                }),
                feedback: Some(AgentContextRuntimeFeedback {
                    replay_trend: AgentContextRuntimeFeedbackReplayTrend {
                        exists: true,
                        health: "ok".to_string(),
                        latest_replay_report_path: "/tmp/reports/feedback_replay.json".to_string(),
                        latest_trend_report_path: "/tmp/reports/feedback_replay_trend.json"
                            .to_string(),
                        summary: AgentContextRuntimeFeedbackReplayTrendSummary {
                            reports: 10,
                            cases: 10,
                            latest_expected_top1_rate: 1.0,
                            trend_rank_improvements: 2,
                            trend_rank_regressions: 0,
                        },
                    },
                }),
            },
            &config,
        );

        assert_eq!(status.last_status, "ok");
        assert_eq!(status.last_message, "已生成上下文包，包含 8 条来源。");
        assert_eq!(status.last_goal, "研究本地推荐系统");
        assert_eq!(status.last_scope, "gitProjects");
        assert_eq!(status.last_generated_pack, "/tmp/context.md");
        assert_eq!(
            status.last_resolution_plan_json,
            "/tmp/resolution_plan.json"
        );
        assert_eq!(status.last_codex_preflight_md, "/tmp/codex_preflight.md");
        assert_eq!(status.access_audit.audit_path, "/tmp/access_audit.jsonl");
        assert_eq!(status.access_audit.events_total, 3);
        assert_eq!(status.access_audit.recent_denied, 1);
        assert_eq!(status.access_audit.recent_consent_required, 1);
        assert_eq!(
            status.access_audit.recent_events[0].identifier,
            "project:secret"
        );
        assert_eq!(
            status.access_audit.last_denied.as_ref().unwrap().reason,
            "path_denied:secret"
        );
        assert_eq!(status.semantic_launchd.health, "not_installed");
        assert_eq!(
            status.semantic_launchd.latest_maintain_report,
            "/tmp/reports/semantic-maintain.json"
        );
        assert_eq!(
            status.semantic_launchd.monitor_path,
            "/tmp/reports/semantic-launchd-monitor-latest.json"
        );
        assert_eq!(
            status.semantic_launchd.next_expected_run_after,
            "2026-06-16T06:39:52+08:00"
        );
        assert!(!status.semantic_launchd.natural_run_overdue);
        assert_eq!(status.semantic_readiness.status, "waiting_for_time");
        assert!(!status.semantic_readiness.ready);
        assert_eq!(status.semantic_readiness.reason, "healthy_but_short_window");
        assert_eq!(status.semantic_readiness.semantic_chunks, 666);
        assert_eq!(status.semantic_readiness.trend_days_remaining, 1);
        assert_eq!(
            status.semantic_readiness.report_path,
            "/tmp/reports/semantic-readiness-latest.json"
        );
        assert_eq!(status.v1_acceptance.status, "waiting_for_time");
        assert!(!status.v1_acceptance.ready);
        assert!(status.v1_acceptance.decision.contains("time-gated"));
        assert_eq!(
            status.v1_acceptance.report_path,
            "/tmp/reports/v1-acceptance-latest.json"
        );
        assert_eq!(
            status.v1_acceptance.followup_markdown_path,
            "/tmp/reports/v1-followup-latest.md"
        );
        assert!(!status.v1_acceptance.can_recheck_now);
        assert_eq!(
            status.v1_acceptance.earliest_recheck_after,
            "2026-06-17T00:00:00+08:00"
        );
        assert_eq!(status.v1_acceptance.trend_days_remaining, 1);
        assert_eq!(status.v1_acceptance.wait_reason, "monitor_not_due");
        assert_eq!(
            status.v1_acceptance.next_gate_at,
            "2026-06-16T12:41:23+08:00"
        );
        assert_eq!(status.v1_acceptance.seconds_until_next_gate, 1000);
        assert_eq!(
            status.v1_acceptance.next_evidence_gate_reason,
            "monitor_not_due"
        );
        assert_eq!(
            status.v1_acceptance.next_evidence_gate_at,
            "2026-06-16T12:41:23+08:00"
        );
        assert_eq!(status.v1_acceptance.seconds_until_next_evidence_gate, 1000);
        assert_eq!(
            status.v1_acceptance.acceptance_wait_reason,
            "multi_day_not_due"
        );
        assert_eq!(
            status.v1_acceptance.acceptance_gate_at,
            "2026-06-17T00:00:00+08:00"
        );
        assert_eq!(status.v1_acceptance.seconds_until_acceptance_gate, 40920);
        assert_eq!(status.v1_acceptance.next_commands.len(), 2);
        assert_eq!(status.v1_stage_status.status, "waiting_for_time");
        assert!(!status.v1_stage_status.ready);
        assert_eq!(
            status.v1_stage_status.report_markdown_path,
            "/tmp/reports/v1-stage-status-latest.md"
        );
        assert_eq!(status.v1_stage_status.ok, 8);
        assert_eq!(status.v1_stage_status.waiting_for_time, 2);
        assert_eq!(status.v1_stage_status.wait_reason, "monitor_not_due");
        assert_eq!(status.v1_stage_status.seconds_until_next_gate, 1000);
        assert_eq!(
            status.v1_stage_status.next_evidence_gate_reason,
            "monitor_not_due"
        );
        assert_eq!(
            status.v1_stage_status.next_evidence_gate_at,
            "2026-06-16T13:41:35+08:00"
        );
        assert_eq!(
            status.v1_stage_status.seconds_until_next_evidence_gate,
            1000
        );
        assert_eq!(
            status.v1_stage_status.acceptance_gate_at,
            "2026-06-17T00:00:00+08:00"
        );
        assert_eq!(status.v1_stage_status.seconds_until_acceptance_gate, 40920);
        assert_eq!(status.v1_stage_status.stages.len(), 2);
        assert_eq!(status.v1_stage_status.stages[0].id, "downloads_ingestion");
        assert!(status.feedback_replay_trend.exists);
        assert_eq!(status.feedback_replay_trend.health, "ok");
        assert_eq!(status.feedback_replay_trend.reports, 10);
        assert_eq!(
            status.feedback_replay_trend.latest_expected_top1_rate,
            "1.0"
        );
        assert_eq!(status.feedback_replay_trend.trend_rank_improvements, 2);
        assert_eq!(status.feedback_replay_trend.trend_rank_regressions, 0);
    }

    #[test]
    fn runtime_preflight_maps_task_preflight_contract() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "deep".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };
        let preflight = runtime_preflight_to_task_preflight(
            AgentContextRuntimeCodexPreflight {
                status: "ok".to_string(),
                error: String::new(),
                goal: "本地推荐系统".to_string(),
                source_scope: "gitProjects".to_string(),
                mode: "deep".to_string(),
                sources_included: 8,
                preflight_markdown_path: "/tmp/codex_preflight.md".to_string(),
                context_md_path: Some("/tmp/context.md".to_string()),
                sources_jsonl_path: Some("/tmp/sources.jsonl".to_string()),
                manifest_json_path: Some("/tmp/manifest.json".to_string()),
                resolution_plan_json_path: Some("/tmp/resolution_plan.json".to_string()),
                ..AgentContextRuntimeCodexPreflight::default()
            },
            &config,
            "ignored",
        );

        assert_eq!(preflight.status, "ok");
        assert_eq!(preflight.message, "任务预检已生成，包含 8 条来源。");
        assert_eq!(preflight.scope, "gitProjects");
        assert_eq!(preflight.mode, "deep");
        assert_eq!(preflight.codex_preflight_md, "/tmp/codex_preflight.md");
        assert_eq!(preflight.context_md, "/tmp/context.md");
    }

    #[test]
    fn runtime_task_maps_first_review_gate_contract() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };
        let preflight = runtime_preflight_to_task_preflight(
            AgentContextRuntimeCodexPreflight {
                status: "awaiting_context_generation".to_string(),
                goal: "开源往事如何在番茄爆火".to_string(),
                session_id: "runtime-task-test".to_string(),
                stage: "clarify_review".to_string(),
                review_file: "/tmp/refined_prompt.md".to_string(),
                runtime_task_md_path: "/tmp/runtime_task.md".to_string(),
                runtime_task_json_path: "/tmp/runtime_task.json".to_string(),
                agent_preflight: Some(AgentContextRuntimeTaskAgentPreflight {
                    agent_preflight_md_path: "/tmp/agent_preflight.md".to_string(),
                }),
                review_launch: Some(AgentContextRuntimeTaskReviewLaunch {
                    review_launch_md_path: "/tmp/review_launch.md".to_string(),
                }),
                client_html_path: "/tmp/doctor-runtime-review-client.html".to_string(),
                review_server_url: "http://127.0.0.1:8765/".to_string(),
                start_server_command: "doctor runtime-review-server --session-id runtime-task-test"
                    .to_string(),
                open_client_command: "open /tmp/doctor-runtime-review-client.html".to_string(),
                ..AgentContextRuntimeCodexPreflight::default()
            },
            &config,
            "ignored",
        );

        assert_eq!(preflight.status, "ok");
        assert!(preflight.message.contains("第一阶段审查"));
        assert_eq!(preflight.sources_included, 0);
        assert_eq!(preflight.context_md, "");
        assert_eq!(preflight.review_file, "/tmp/refined_prompt.md");
        assert_eq!(preflight.runtime_task_md, "/tmp/runtime_task.md");
        assert_eq!(
            preflight.review_client_html,
            "/tmp/doctor-runtime-review-client.html"
        );
    }

    #[test]
    fn agent_preflight_context_maps_model_input_review_contract() {
        let config = AgentContextPanelConfig {
            auto_context: true,
            scope: "all".to_string(),
            mode: "fast".to_string(),
            agent_context_root: "/tmp/agent-context-system".to_string(),
            agent_context_bin: "/tmp/agent-context".to_string(),
        };
        let previous = AgentContextPanelStatus {
            last_session_id: "runtime-task-test".to_string(),
            last_goal: "审查模型输入".to_string(),
            ..AgentContextPanelStatus::default()
        };
        let preflight = agent_preflight_to_task_preflight(
            AgentContextRuntimeAgentPreflight {
                status: "awaiting_context_review".to_string(),
                next_message: "Review model_input.md before any model consumes it.".to_string(),
                session_id: "runtime-task-test".to_string(),
                source_scope: "all".to_string(),
                mode: "fast".to_string(),
                review_file: "/tmp/pack/model_input.md".to_string(),
                agent_preflight_md_path: "/tmp/runtime/agent_preflight.md".to_string(),
                files: AgentContextRuntimeAgentPreflightFiles {
                    context_md_path: Some("/tmp/pack/context.md".to_string()),
                    sources_jsonl_path: Some("/tmp/pack/sources.jsonl".to_string()),
                    model_input_md_path: Some("/tmp/pack/model_input.md".to_string()),
                    runtime_task_md_path: Some("/tmp/runtime/runtime_task.md".to_string()),
                    runtime_task_json_path: Some("/tmp/runtime/runtime_task.json".to_string()),
                    runtime_review_client_html_path: Some(
                        "/tmp/runtime/doctor-runtime-review-client.html".to_string(),
                    ),
                    runtime_review_launch_md_path: Some(
                        "/tmp/runtime/review_launch.md".to_string(),
                    ),
                },
            },
            &config,
            &previous,
        );

        assert_eq!(preflight.status, "awaiting_context_review");
        assert_eq!(preflight.goal, "审查模型输入");
        assert_eq!(preflight.model_input_md, "/tmp/pack/model_input.md");
        assert_eq!(preflight.review_file, "/tmp/pack/model_input.md");
        assert_eq!(preflight.context_md, "/tmp/pack/context.md");
        assert_eq!(
            preflight.agent_preflight_md,
            "/tmp/runtime/agent_preflight.md"
        );
        assert!(preflight.message.contains("model_input.md"));
    }

    #[test]
    fn agent_preflight_answer_maps_answer_review_bridge_contract() {
        let bridge = answer_preflight_to_bridge_value(json!({
            "status": "awaiting_answer_output",
            "next_message": "Use answer_packet.md with a model or local answer command, then review the answer.",
            "session_id": "runtime-task-test",
            "review_file": "/tmp/pack/answer_packet.md",
            "agent_preflight_md_path": "/tmp/runtime/agent_preflight.md",
            "client_contract": {
                "safe_to_send_model": true,
                "approved_model_input_md_path": "/tmp/pack/model_input.md",
                "agent_handoff_md_path": "/tmp/pack/agent_handoff.md",
                "answer_packet_md_path": "/tmp/pack/answer_packet.md",
                "answer_md_path": "/tmp/pack/answer.md"
            },
            "files": {
                "context_md_path": "/tmp/pack/context.md",
                "sources_jsonl_path": "/tmp/pack/sources.jsonl"
            },
            "next_commands": ["agent-context answer-review --action approve"]
        }));

        assert_eq!(bridge["status"], "awaiting_answer_output");
        assert_eq!(bridge["sessionId"], "runtime-task-test");
        assert_eq!(bridge["safeToSendModel"], true);
        assert_eq!(bridge["approvedModelInputMd"], "/tmp/pack/model_input.md");
        assert_eq!(bridge["agentHandoffMd"], "/tmp/pack/agent_handoff.md");
        assert_eq!(bridge["answerPacketMd"], "/tmp/pack/answer_packet.md");
        assert_eq!(bridge["answerMd"], "/tmp/pack/answer.md");
        assert_eq!(bridge["contextMd"], "/tmp/pack/context.md");
        assert_eq!(bridge["sourcesJsonl"], "/tmp/pack/sources.jsonl");
        assert_eq!(bridge["reviewFile"], "/tmp/pack/answer_packet.md");
        assert_eq!(
            bridge["agentPreflightMd"],
            "/tmp/runtime/agent_preflight.md"
        );
        assert!(
            bridge["message"]
                .as_str()
                .unwrap()
                .contains("answer_packet.md")
        );
    }

    #[test]
    fn agent_preflight_execution_maps_execution_review_bridge_contract() {
        let bridge = execution_preflight_to_bridge_value(
            json!({
                "status": "awaiting_execution",
                "next_message": "Prepare an explicit local command for user review.",
                "session_id": "runtime-task-test",
                "review_file": "/tmp/pack/execution_report.md",
                "agent_preflight_md_path": "/tmp/runtime/agent_preflight.md",
                "client_contract": {
                    "safe_to_send_model": false,
                    "execution_report_md_path": "/tmp/pack/execution_report.md",
                    "execution_artifact_index_md_path": "/tmp/pack/execution_artifacts.md"
                },
                "files": {
                    "answer_md_path": "/tmp/pack/answer.md",
                    "execution_review_json_path": "/tmp/pack/execution_review.json",
                    "execution_artifact_manifest_jsonl_path": "/tmp/pack/execution_artifacts.jsonl",
                    "artifacts_dir": "/tmp/pack/artifacts"
                },
                "next_commands": ["agent-context execution-review --action run"]
            }),
            PathBuf::from("/tmp/codex-plus-answer.md"),
        );

        assert_eq!(bridge["status"], "awaiting_execution");
        assert_eq!(bridge["sessionId"], "runtime-task-test");
        assert_eq!(bridge["safeToSendModel"], false);
        assert_eq!(bridge["recordedAnswerFile"], "/tmp/codex-plus-answer.md");
        assert_eq!(bridge["answerMd"], "/tmp/pack/answer.md");
        assert_eq!(
            bridge["executionReviewJson"],
            "/tmp/pack/execution_review.json"
        );
        assert_eq!(bridge["executionReportMd"], "/tmp/pack/execution_report.md");
        assert_eq!(
            bridge["executionArtifactsJsonl"],
            "/tmp/pack/execution_artifacts.jsonl"
        );
        assert_eq!(
            bridge["executionArtifactIndexMd"],
            "/tmp/pack/execution_artifacts.md"
        );
        assert_eq!(bridge["artifactsDir"], "/tmp/pack/artifacts");
        assert_eq!(bridge["reviewFile"], "/tmp/pack/execution_report.md");
        assert_eq!(
            bridge["agentPreflightMd"],
            "/tmp/runtime/agent_preflight.md"
        );
    }

    #[test]
    fn execution_review_maps_command_run_bridge_contract() {
        let bridge = execution_review_to_bridge_value(
            json!({
                "status": "executed",
                "session_id": "runtime-task-test",
                "last_run_id": "run-test",
                "last_returncode": 0,
                "last_timed_out": false,
                "execution_review_json_path": "/tmp/pack/execution_review.json",
                "execution_report_md_path": "/tmp/pack/execution_report.md",
                "artifact_manifest_jsonl_path": "/tmp/pack/execution_artifacts.jsonl",
                "artifact_index_md_path": "/tmp/pack/execution_artifacts.md",
                "artifacts_dir": "/tmp/pack/artifacts",
                "artifact_count": 3,
                "commands": [{
                    "run_id": "run-test",
                    "command": "python -c \"print('runtime artifact')\"",
                    "cwd": "/tmp",
                    "returncode": 0,
                    "timed_out": false,
                    "stdout_path": "/tmp/pack/artifacts/run-test.stdout.txt",
                    "stderr_path": "/tmp/pack/artifacts/run-test.stderr.txt",
                    "result_json_path": "/tmp/pack/artifacts/run-test.json"
                }]
            }),
            "Doctor ran the approved explicit local command and captured artifacts.",
        );

        assert_eq!(bridge["status"], "executed");
        assert_eq!(bridge["sessionId"], "runtime-task-test");
        assert_eq!(bridge["command"], "python -c \"print('runtime artifact')\"");
        assert_eq!(bridge["lastRunId"], "run-test");
        assert_eq!(bridge["lastReturncode"], 0);
        assert_eq!(bridge["lastTimedOut"], false);
        assert_eq!(
            bridge["stdoutPath"],
            "/tmp/pack/artifacts/run-test.stdout.txt"
        );
        assert_eq!(
            bridge["stderrPath"],
            "/tmp/pack/artifacts/run-test.stderr.txt"
        );
        assert_eq!(
            bridge["resultJsonPath"],
            "/tmp/pack/artifacts/run-test.json"
        );
        assert_eq!(
            bridge["executionArtifactIndexMd"],
            "/tmp/pack/execution_artifacts.md"
        );
        assert_eq!(bridge["artifactCount"], 3);
        assert_eq!(bridge["complete"], false);
    }
}
