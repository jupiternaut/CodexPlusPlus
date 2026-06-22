#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const repoRoot = path.resolve(new URL("..", import.meta.url).pathname);
const agentContextRoot = process.env.AGENT_CONTEXT_ROOT || path.join(os.homedir(), "agent-context-system");
const agentContextBin = process.env.AGENT_CONTEXT_BIN || path.join(agentContextRoot, "agent-context");
const statusPath = process.env.AGENT_CONTEXT_PANEL_STATUS || path.join(agentContextRoot, "panel", "status.json");
const skipPanelRefresh = process.env.CODEX_PLUS_SMOKE_SKIP_AGENT_CONTEXT_PANEL === "1";

function fail(message, details = {}) {
  console.error(JSON.stringify({ status: "failed", message, ...details }, null, 2));
  process.exit(1);
}

function assertIncludes(filePath, needles) {
  const text = fs.readFileSync(filePath, "utf8");
  const missing = needles.filter((needle) => !text.includes(needle));
  if (missing.length) {
    fail("Codex++ source does not consume expected panel status fields", {
      filePath,
      missing,
    });
  }
}

function refreshPanelStatus() {
  if (skipPanelRefresh) return;
  if (!fs.existsSync(agentContextBin)) {
    fail("agent-context binary not found", { agentContextBin });
  }
  const result = spawnSync(
    agentContextBin,
    ["panel", "--out", agentContextRoot, "--no-auto-context", "--mode", "fast", "--source-scope", "all"],
    {
      cwd: agentContextRoot,
      encoding: "utf8",
    }
  );
  if (result.status !== 0) {
    fail("agent-context panel refresh failed", {
      agentContextBin,
      stderr: result.stderr,
      stdout: result.stdout,
    });
  }
}

function readStatus() {
  if (!fs.existsSync(statusPath)) {
    fail("panel/status.json not found", { statusPath });
  }
  try {
    return JSON.parse(fs.readFileSync(statusPath, "utf8"));
  } catch (error) {
    fail("panel/status.json is not valid JSON", { statusPath, error: String(error?.message || error) });
  }
}

function validateSemanticLaunchd(status) {
  const semanticLaunchd = status.semantic_launchd;
  if (!semanticLaunchd || typeof semanticLaunchd !== "object") {
    fail("panel/status.json is missing semantic_launchd", { statusPath });
  }
  const allowedHealth = new Set(["not_installed", "ok", "degraded", "not_checked"]);
  if (!allowedHealth.has(String(semanticLaunchd.health || ""))) {
    fail("semantic_launchd.health is not a recognized status", {
      statusPath,
      health: semanticLaunchd.health,
    });
  }
  if (typeof semanticLaunchd.installed !== "boolean") {
    fail("semantic_launchd.installed must be boolean", {
      statusPath,
      installed: semanticLaunchd.installed,
    });
  }
  for (const key of ["plist_path", "script_path", "reports", "monitor"]) {
    if (!(key in semanticLaunchd)) {
      fail(`semantic_launchd.${key} is missing`, { statusPath });
    }
  }
  if (typeof semanticLaunchd.monitor !== "object" || semanticLaunchd.monitor === null) {
    fail("semantic_launchd.monitor must be an object", { statusPath });
  }
  return semanticLaunchd;
}

function validateFeedbackReplayTrend(status) {
  const feedback = status.feedback;
  if (!feedback || typeof feedback !== "object") {
    fail("panel/status.json is missing feedback", { statusPath });
  }
  const replayTrend = feedback.replay_trend;
  if (!replayTrend || typeof replayTrend !== "object") {
    fail("panel/status.json is missing feedback.replay_trend", { statusPath });
  }
  const allowedHealth = new Set(["not_checked", "ok", "warning", "alert"]);
  if (!allowedHealth.has(String(replayTrend.health || ""))) {
    fail("feedback.replay_trend.health is not a recognized status", {
      statusPath,
      health: replayTrend.health,
    });
  }
  if (typeof replayTrend.exists !== "boolean") {
    fail("feedback.replay_trend.exists must be boolean", {
      statusPath,
      exists: replayTrend.exists,
    });
  }
  if (!replayTrend.summary || typeof replayTrend.summary !== "object") {
    fail("feedback.replay_trend.summary must be an object", { statusPath });
  }
  for (const key of ["reports", "cases", "latest_expected_top1_rate", "trend_rank_improvements", "trend_rank_regressions"]) {
    if (!(key in replayTrend.summary)) {
      fail(`feedback.replay_trend.summary.${key} is missing`, { statusPath });
    }
  }
  return replayTrend;
}

function validateSemanticReadiness(status) {
  const readiness = status.semantic_readiness;
  if (!readiness || typeof readiness !== "object") {
    fail("panel/status.json is missing semantic_readiness", { statusPath });
  }
  const allowedStatus = new Set(["missing", "not_checked", "ready", "waiting_for_time", "attention_required", "failed"]);
  if (!allowedStatus.has(String(readiness.status || ""))) {
    fail("semantic_readiness.status is not a recognized status", {
      statusPath,
      status: readiness.status,
    });
  }
  if (typeof readiness.ready !== "boolean") {
    fail("semantic_readiness.ready must be boolean", {
      statusPath,
      ready: readiness.ready,
    });
  }
  if (!readiness.summary || typeof readiness.summary !== "object") {
    fail("semantic_readiness.summary must be an object", { statusPath });
  }
  for (const key of ["semantic_chunks", "trend_days_observed", "trend_days_remaining", "monitor_snapshots"]) {
    if (!(key in readiness.summary)) {
      fail(`semantic_readiness.summary.${key} is missing`, { statusPath });
    }
  }
  return readiness;
}

function validateV1Acceptance(status) {
  const acceptance = status.v1_acceptance;
  if (!acceptance || typeof acceptance !== "object") {
    fail("panel/status.json is missing v1_acceptance", { statusPath });
  }
  const allowedStatus = new Set(["missing", "not_checked", "ok", "ready", "waiting_for_time", "warning", "failed"]);
  if (!allowedStatus.has(String(acceptance.status || ""))) {
    fail("v1_acceptance.status is not a recognized status", {
      statusPath,
      status: acceptance.status,
    });
  }
  if (typeof acceptance.ready !== "boolean") {
    fail("v1_acceptance.ready must be boolean", {
      statusPath,
      ready: acceptance.ready,
    });
  }
  if (!("decision" in acceptance)) {
    fail("v1_acceptance.decision is missing", { statusPath });
  }
  if (!Array.isArray(acceptance.next_commands)) {
    fail("v1_acceptance.next_commands must be an array", { statusPath });
  }
  if (!("latest_followup_md_path" in acceptance)) {
    fail("v1_acceptance.latest_followup_md_path is missing", { statusPath });
  }
  if (!("latest_followup_json_path" in acceptance)) {
    fail("v1_acceptance.latest_followup_json_path is missing", { statusPath });
  }
  if (!acceptance.followup_plan || typeof acceptance.followup_plan !== "object") {
    fail("v1_acceptance.followup_plan must be an object", { statusPath });
  }
  if (typeof acceptance.followup_plan.can_recheck_now !== "boolean") {
    fail("v1_acceptance.followup_plan.can_recheck_now must be boolean", { statusPath });
  }
  if (!acceptance.followup_check || typeof acceptance.followup_check !== "object") {
    fail("v1_acceptance.followup_check must be an object", { statusPath });
  }
  if (typeof acceptance.followup_check.wait_reason !== "string") {
    fail("v1_acceptance.followup_check.wait_reason must be string", { statusPath });
  }
  if (typeof acceptance.followup_check.acceptance_wait_reason !== "string") {
    fail("v1_acceptance.followup_check.acceptance_wait_reason must be string", { statusPath });
  }
  for (const key of [
    "next_evidence_gate_reason",
    "next_evidence_gate_at",
    "seconds_until_next_evidence_gate",
    "seconds_until_acceptance_gate",
    "acceptance_gate_at",
    "seconds_until_acceptance_gate",
  ]) {
    if (!(key in acceptance.followup_check)) {
      fail(`v1_acceptance.followup_check.${key} is missing`, { statusPath });
    }
  }
  return acceptance;
}

function validateV1StageStatus(status) {
  const stageStatus = status.v1_stage_status;
  if (!stageStatus || typeof stageStatus !== "object") {
    fail("panel/status.json is missing v1_stage_status", { statusPath });
  }
  const allowedStatus = new Set(["missing", "not_checked", "ok", "ready", "waiting_for_time", "warning", "failed"]);
  if (!allowedStatus.has(String(stageStatus.status || ""))) {
    fail("v1_stage_status.status is not a recognized status", {
      statusPath,
      status: stageStatus.status,
    });
  }
  if (typeof stageStatus.ready !== "boolean") {
    fail("v1_stage_status.ready must be boolean", { statusPath, ready: stageStatus.ready });
  }
  if (!stageStatus.summary || typeof stageStatus.summary !== "object") {
    fail("v1_stage_status.summary must be an object", { statusPath });
  }
  for (const key of ["stages_total", "ok", "waiting_for_time", "warning", "failed"]) {
    if (!(key in stageStatus.summary)) {
      fail(`v1_stage_status.summary.${key} is missing`, { statusPath });
    }
  }
  if (!stageStatus.next_gates || typeof stageStatus.next_gates !== "object") {
    fail("v1_stage_status.next_gates must be an object", { statusPath });
  }
  for (const key of [
    "next_evidence_gate_reason",
    "next_evidence_gate_at",
    "seconds_until_next_evidence_gate",
    "acceptance_gate_at",
    "seconds_until_acceptance_gate",
  ]) {
    if (!(key in stageStatus.next_gates)) {
      fail(`v1_stage_status.next_gates.${key} is missing`, { statusPath });
    }
  }
  if (!Array.isArray(stageStatus.stages)) {
    fail("v1_stage_status.stages must be an array", { statusPath });
  }
  return stageStatus;
}

function validateCodexPlusConsumption() {
  assertIncludes(path.join(repoRoot, "crates", "codex-plus-core", "src", "agent_context.rs"), [
    "pub semantic_launchd: AgentContextPanelSemanticLaunchd",
    "pub semantic_readiness: AgentContextPanelSemanticReadiness",
    "pub v1_acceptance: AgentContextPanelV1Acceptance",
    "pub v1_stage_status: AgentContextPanelV1StageStatus",
    "pub feedback_replay_trend: AgentContextPanelFeedbackReplayTrend",
    "pub last_runtime_task_md: String",
    "pub last_review_file: String",
    "pub last_review_client_html: String",
    "pub last_review_launch_md: String",
    "struct AgentContextRuntimeSemanticLaunchd",
    "struct AgentContextRuntimeSemanticReadiness",
    "struct AgentContextRuntimeV1Acceptance",
    "struct AgentContextRuntimeV1FollowupPlan",
    "struct AgentContextRuntimeV1FollowupCheck",
    "struct AgentContextRuntimeV1StageStatus",
    "struct AgentContextRuntimeFeedbackReplayTrend",
    "runtime_semantic_launchd_to_panel",
    "runtime_semantic_readiness_to_panel",
    "runtime_v1_acceptance_to_panel",
    "runtime_v1_stage_status_to_panel",
    "run_agent_context_v1_followup",
    '"v1-refresh".to_string()',
    '"--with-manager-feedback-smoke".to_string()',
    "followup_markdown_path",
    "can_recheck_now",
    "wait_reason",
    "next_gate_at",
    "next_evidence_gate_reason",
    "next_evidence_gate_at",
    "seconds_until_next_evidence_gate",
    "acceptance_wait_reason",
    "acceptance_gate_at",
    "seconds_until_acceptance_gate",
    "runtime_feedback_replay_trend_to_panel",
    "semantic_launchd: Option<AgentContextRuntimeSemanticLaunchd>",
    "semantic_readiness: Option<AgentContextRuntimeSemanticReadiness>",
    "v1_acceptance: Option<AgentContextRuntimeV1Acceptance>",
    "v1_stage_status: Option<AgentContextRuntimeV1StageStatus>",
    "feedback: Option<AgentContextRuntimeFeedback>",
  ]);
  assertIncludes(path.join(repoRoot, "apps", "codex-plus-manager", "src", "App.tsx"), [
    "type AgentContextSemanticLaunchd",
    "type AgentContextSemanticReadiness",
    "type AgentContextV1Acceptance",
    "type AgentContextV1StageStatus",
    "type AgentContextFeedbackReplayTrend",
    "semanticLaunchd: AgentContextSemanticLaunchd",
    "semanticReadiness: AgentContextSemanticReadiness",
    "v1Acceptance: AgentContextV1Acceptance",
    "v1StageStatus: AgentContextV1StageStatus",
    "feedbackReplayTrend: AgentContextFeedbackReplayTrend",
    "lastRuntimeTaskMd: string",
    "lastReviewFile: string",
    "lastReviewClientHtml: string",
    "lastReviewLaunchMd: string",
    "status.semanticLaunchd",
    "status.semanticReadiness",
    "status.v1Acceptance",
    "status.v1StageStatus",
    "status.feedbackReplayTrend",
    "Semantic maintenance",
    "Semantic readiness",
    "V1 acceptance",
    "V1 stage status",
    "followupMarkdownPath",
    "earliestRecheckAfter",
    "canRecheckNow",
    "waitReason",
    "nextGateAt",
    "nextEvidenceGateReason",
    "nextEvidenceGateAt",
    "secondsUntilNextEvidenceGate",
    "acceptanceWaitReason",
    "acceptanceGateAt",
    "secondsUntilAcceptanceGate",
    "runAgentContextV1Followup",
    "运行复验 gate",
    "Feedback replay",
    "runtime_task.md",
    "review client",
    "review_launch.md",
    "nextExpectedRunAfter",
  ]);
}

function main() {
  refreshPanelStatus();
  const status = readStatus();
  const semanticLaunchd = validateSemanticLaunchd(status);
  const semanticReadiness = validateSemanticReadiness(status);
  const v1Acceptance = validateV1Acceptance(status);
  const v1StageStatus = validateV1StageStatus(status);
  const replayTrend = validateFeedbackReplayTrend(status);
  validateCodexPlusConsumption();
  console.log(JSON.stringify({
    status: "ok",
    agentContextRoot,
    statusPath,
    health: semanticLaunchd.health,
    installed: semanticLaunchd.installed,
    plistPath: semanticLaunchd.plist_path || "",
    scriptPath: semanticLaunchd.script_path || "",
    maintainReport: semanticLaunchd.reports?.semantic_maintain?.path || "",
    pruneReport: semanticLaunchd.reports?.semantic_ann_prune?.path || "",
    monitorPath: semanticLaunchd.monitor?.path || "",
    nextExpectedRunAfter: semanticLaunchd.monitor?.summary?.next_expected_run_after || "",
    naturalRunDue: semanticLaunchd.monitor?.summary?.natural_run_due === true,
    naturalRunOverdue: semanticLaunchd.monitor?.summary?.natural_run_overdue === true,
    semanticReadinessStatus: semanticReadiness.status || "",
    semanticReadinessReady: semanticReadiness.ready === true,
    semanticReadinessChunks: semanticReadiness.summary?.semantic_chunks ?? 0,
    semanticReadinessTrendDaysRemaining: semanticReadiness.summary?.trend_days_remaining ?? 0,
    v1AcceptanceStatus: v1Acceptance.status || "",
    v1AcceptanceReady: v1Acceptance.ready === true,
    v1FollowupMarkdownPath: v1Acceptance.latest_followup_md_path || "",
    v1CanRecheckNow: v1Acceptance.followup_plan?.can_recheck_now === true,
    v1WaitReason: v1Acceptance.followup_check?.wait_reason || "",
    v1NextGateAt: v1Acceptance.followup_check?.next_gate_at || "",
    v1NextEvidenceGateAt:
      v1Acceptance.followup_check?.next_evidence_gate_at || v1Acceptance.followup_check?.next_gate_at || "",
    v1AcceptanceWaitReason: v1Acceptance.followup_check?.acceptance_wait_reason || "",
    v1AcceptanceGateAt: v1Acceptance.followup_check?.acceptance_gate_at || "",
    v1StageStatus: v1StageStatus.status || "",
    v1StageOk: v1StageStatus.summary?.ok ?? 0,
    v1StageWaiting: v1StageStatus.summary?.waiting_for_time ?? 0,
    v1StageNextEvidenceGateAt:
      v1StageStatus.next_gates?.next_evidence_gate_at || v1StageStatus.next_gates?.next_gate_at || "",
    v1StageReport: v1StageStatus.latest_md_path || "",
    replayHealth: replayTrend.health,
    replayReports: replayTrend.summary?.reports ?? 0,
    replayLatestExpectedTop1Rate: replayTrend.summary?.latest_expected_top1_rate ?? 0,
    replayRankRegressions: replayTrend.summary?.trend_rank_regressions ?? 0,
    codexPlusConsumesStatus: true,
  }, null, 2));
}

main();
