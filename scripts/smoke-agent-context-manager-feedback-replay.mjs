#!/usr/bin/env node

import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const repoRoot = path.resolve(new URL("..", import.meta.url).pathname);
const managerRoot = path.join(repoRoot, "apps", "codex-plus-manager");
const agentContextRoot = process.env.AGENT_CONTEXT_ROOT || path.join(os.homedir(), "agent-context-system");
const statusPath = process.env.AGENT_CONTEXT_PANEL_STATUS || path.join(agentContextRoot, "panel", "status.json");
const evidenceDir = process.env.CODEX_PLUS_SMOKE_SCREENSHOT_DIR || path.join(agentContextRoot, "reports", "screenshots");
const appPath = path.join(managerRoot, "src", "App.tsx");
const stylesPath = path.join(managerRoot, "src", "styles.css");

function fail(message, details = {}) {
  console.error(JSON.stringify({ status: "failed", message, ...details }, null, 2));
  process.exit(1);
}

function readJson(filePath) {
  try {
    return JSON.parse(fs.readFileSync(filePath, "utf8"));
  } catch (error) {
    fail("failed to read JSON", { filePath, error: String(error?.message || error) });
  }
}

function readText(filePath) {
  try {
    return fs.readFileSync(filePath, "utf8");
  } catch (error) {
    fail("failed to read text", { filePath, error: String(error?.message || error) });
  }
}

function camelStatus(status) {
  const semanticReadiness = status.semantic_readiness || {};
  const semanticReadinessSummary = semanticReadiness.summary || {};
  const v1Acceptance = status.v1_acceptance || {};
  const v1Followup = v1Acceptance.followup_plan || {};
  const v1FollowupCheck = v1Acceptance.followup_check || {};
  const v1StageStatus = status.v1_stage_status || {};
  const v1StageSummary = v1StageStatus.summary || {};
  const v1StageGates = v1StageStatus.next_gates || {};
  const replayTrend = status.feedback?.replay_trend || {};
  const replaySummary = replayTrend.summary || {};

  return {
    semanticReadiness: {
      exists: semanticReadiness.exists === true,
      status: semanticReadiness.status || "not_checked",
      ready: semanticReadiness.ready === true,
      reason: semanticReadinessSummary.reason || "",
      semanticChunks: semanticReadinessSummary.semantic_chunks || 0,
      trendDaysObserved: semanticReadinessSummary.trend_days_observed || 0,
      trendDaysRemaining: semanticReadinessSummary.trend_days_remaining || 0,
      reportMarkdownPath: semanticReadiness.latest_md_path || "",
    },
    v1Acceptance: {
      exists: v1Acceptance.exists === true,
      status: v1Acceptance.status || "not_checked",
      ready: v1Acceptance.ready === true,
      decision: v1Acceptance.decision || "",
      reportMarkdownPath: v1Acceptance.latest_md_path || "",
      followupMarkdownPath: v1Acceptance.latest_followup_md_path || v1Followup.latest_md_path || "",
      canRecheckNow: v1Followup.can_recheck_now === true,
      earliestRecheckAfter: v1Followup.earliest_recheck_after || "",
      nextMonitorDueAt: v1Followup.next_monitor_due_at || "",
      trendDaysRemaining: v1Followup.trend_days_remaining || 0,
      waitReason: v1FollowupCheck.wait_reason || "",
      nextEvidenceGateReason: v1FollowupCheck.next_evidence_gate_reason || v1FollowupCheck.wait_reason || "",
      nextEvidenceGateAt: v1FollowupCheck.next_evidence_gate_at || v1FollowupCheck.next_gate_at || "",
      acceptanceWaitReason: v1FollowupCheck.acceptance_wait_reason || "",
      acceptanceGateAt: v1FollowupCheck.acceptance_gate_at || "",
      nextCommands: Array.isArray(v1Acceptance.next_commands) ? v1Acceptance.next_commands : [],
    },
    v1StageStatus: {
      exists: v1StageStatus.exists === true,
      status: v1StageStatus.status || "not_checked",
      ready: v1StageStatus.ready === true,
      decision: v1StageStatus.decision || "",
      reportMarkdownPath: v1StageStatus.latest_md_path || "",
      ok: v1StageSummary.ok || 0,
      waitingForTime: v1StageSummary.waiting_for_time || 0,
      warning: v1StageSummary.warning || 0,
      failed: v1StageSummary.failed || 0,
      nextEvidenceGateReason: v1StageGates.next_evidence_gate_reason || v1StageGates.wait_reason || "",
      nextEvidenceGateAt: v1StageGates.next_evidence_gate_at || v1StageGates.next_gate_at || "",
      acceptanceWaitReason: v1StageGates.acceptance_wait_reason || "",
      acceptanceGateAt: v1StageGates.acceptance_gate_at || "",
      trendDaysRemaining: v1StageGates.trend_days_remaining || 0,
      stages: Array.isArray(v1StageStatus.stages) ? v1StageStatus.stages : [],
    },
    feedbackReplayTrend: {
      exists: replayTrend.exists === true,
      health: replayTrend.health || "not_checked",
      reports: replaySummary.reports || 0,
      cases: replaySummary.cases || 0,
      latestExpectedTop1Rate:
        replaySummary.latest_expected_top1_rate === undefined ? "" : String(replaySummary.latest_expected_top1_rate),
      trendRankImprovements: replaySummary.trend_rank_improvements || 0,
      trendRankRegressions: replaySummary.trend_rank_regressions || 0,
      latestReplayReportPath: replayTrend.latest_replay_report_path || "",
      latestTrendReportPath: replayTrend.latest_trend_report_path || "",
    },
  };
}

function assertRequiredFields(panelStatus) {
  const missing = [];
  if (!panelStatus.feedbackReplayTrend.exists) missing.push("feedbackReplayTrend.exists");
  if (!panelStatus.v1Acceptance.exists) missing.push("v1Acceptance.exists");
  if (!panelStatus.v1StageStatus.exists) missing.push("v1StageStatus.exists");
  if (!panelStatus.semanticReadiness.exists) missing.push("semanticReadiness.exists");
  if (!panelStatus.v1Acceptance.reportMarkdownPath) missing.push("v1Acceptance.reportMarkdownPath");
  if (!panelStatus.v1StageStatus.reportMarkdownPath) missing.push("v1StageStatus.reportMarkdownPath");
  if (!panelStatus.feedbackReplayTrend.latestTrendReportPath) missing.push("feedbackReplayTrend.latestTrendReportPath");
  if (missing.length) {
    fail("agent-context panel/status.json is missing Manager feedback replay fields", { statusPath, missing });
  }
}

function sourceChecks() {
  const appSource = readText(appPath);
  const stylesSource = readText(stylesPath);
  const requiredAppSnippets = [
    "Codex++ Context Panel",
    "自动上下文",
    "Semantic readiness",
    "V1 acceptance",
    "V1 stage status",
    "运行复验 gate",
    "打开复验计划",
    "runAgentContextV1Followup",
    "Feedback replay",
    "Replay 历史",
    "latest top1 rate",
    "rank regressions",
    "latest replay",
    "trend report",
  ];
  const requiredStyleSnippets = [
    ".agent-context-status",
    ".agent-context-stage-list",
    ".agent-context-files",
  ];
  const missingApp = requiredAppSnippets.filter((snippet) => !appSource.includes(snippet));
  const missingStyles = requiredStyleSnippets.filter((snippet) => !stylesSource.includes(snippet));
  if (missingApp.length || missingStyles.length) {
    fail("Codex++ Manager source contract is missing Agent Context feedback replay UI", {
      appPath,
      stylesPath,
      missingApp,
      missingStyles,
    });
  }
  return {
    appPath,
    stylesPath,
    checkedAppSnippets: requiredAppSnippets.length,
    checkedStyleSnippets: requiredStyleSnippets.length,
  };
}

function escapeMarkdown(value) {
  return String(value || "").replaceAll("|", "\\|").replaceAll("\n", " ");
}

function writeEvidence(panelStatus, checks) {
  fs.mkdirSync(evidenceDir, { recursive: true });
  const stamp = new Date().toISOString().replace(/[-:TZ.]/g, "").slice(0, 14);
  const evidencePath = path.join(evidenceDir, `codex-plus-manager-feedback-replay-${stamp}.md`);
  const rows = [
    ["Semantic readiness", panelStatus.semanticReadiness.status, panelStatus.semanticReadiness.reportMarkdownPath],
    ["V1 acceptance", panelStatus.v1Acceptance.status, panelStatus.v1Acceptance.reportMarkdownPath],
    ["V1 stage status", panelStatus.v1StageStatus.status, panelStatus.v1StageStatus.reportMarkdownPath],
    ["Feedback replay", panelStatus.feedbackReplayTrend.health, panelStatus.feedbackReplayTrend.latestTrendReportPath],
  ];
  const markdown = [
    "# Codex++ Manager Feedback Replay Smoke",
    "",
    "This smoke verifies the Manager source contract and the live `panel/status.json` data required to render the Agent Context feedback replay and v1 follow-up cards.",
    "",
    "## Source Contract",
    "",
    `- App source: ${checks.appPath}`,
    `- Styles source: ${checks.stylesPath}`,
    `- App snippets checked: ${checks.checkedAppSnippets}`,
    `- Style snippets checked: ${checks.checkedStyleSnippets}`,
    "",
    "## Live Status",
    "",
    "| Card | Status | Evidence |",
    "| --- | --- | --- |",
    ...rows.map((row) => `| ${escapeMarkdown(row[0])} | ${escapeMarkdown(row[1])} | ${escapeMarkdown(row[2])} |`),
    "",
    "## Feedback Replay",
    "",
    `- Reports: ${panelStatus.feedbackReplayTrend.reports}`,
    `- Cases: ${panelStatus.feedbackReplayTrend.cases}`,
    `- Latest top1 rate: ${panelStatus.feedbackReplayTrend.latestExpectedTop1Rate}`,
    `- Rank improvements: ${panelStatus.feedbackReplayTrend.trendRankImprovements}`,
    `- Rank regressions: ${panelStatus.feedbackReplayTrend.trendRankRegressions}`,
    "",
    "## V1 Follow-up",
    "",
    `- Can recheck now: ${String(panelStatus.v1Acceptance.canRecheckNow)}`,
    `- Trend days remaining: ${panelStatus.v1Acceptance.trendDaysRemaining}`,
    `- Evidence wait reason: ${panelStatus.v1Acceptance.nextEvidenceGateReason}`,
    `- Next evidence gate: ${panelStatus.v1Acceptance.nextEvidenceGateAt}`,
    `- Acceptance wait reason: ${panelStatus.v1Acceptance.acceptanceWaitReason}`,
    `- Acceptance gate: ${panelStatus.v1Acceptance.acceptanceGateAt}`,
    "",
  ].join("\n");
  fs.writeFileSync(evidencePath, markdown, "utf8");
  return evidencePath;
}

function main() {
  if (!fs.existsSync(statusPath)) {
    fail("agent-context panel/status.json not found", { statusPath });
  }
  if (!fs.existsSync(appPath)) {
    fail("Codex++ Manager App.tsx not found", { appPath });
  }
  if (!fs.existsSync(stylesPath)) {
    fail("Codex++ Manager styles.css not found", { stylesPath });
  }

  const panelStatus = camelStatus(readJson(statusPath));
  assertRequiredFields(panelStatus);
  const checks = sourceChecks();
  const evidencePath = writeEvidence(panelStatus, checks);

  console.log(JSON.stringify({
    status: "ok",
    mode: "source_contract",
    statusPath,
    evidencePath,
    sourceContract: checks,
    v1Acceptance: panelStatus.v1Acceptance,
    v1StageStatus: panelStatus.v1StageStatus,
    feedbackReplayTrend: panelStatus.feedbackReplayTrend,
  }, null, 2));
}

main();
