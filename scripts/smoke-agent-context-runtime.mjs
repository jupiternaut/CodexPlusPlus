#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import os from "node:os";
import path from "node:path";

const repoRoot = path.resolve(new URL("..", import.meta.url).pathname);
const debugPort = Number(process.env.CODEX_PLUS_SMOKE_DEBUG_PORT || 19333);
const helperPort = Number(process.env.CODEX_PLUS_SMOKE_HELPER_PORT || 57332);
const codexAppPath = process.env.CODEX_PLUS_SMOKE_CODEX_APP || "/Applications/Codex.app";
const agentContextRoot = process.env.AGENT_CONTEXT_ROOT || path.join(os.homedir(), "agent-context-system");
const agentContextBin = process.env.AGENT_CONTEXT_BIN || path.join(agentContextRoot, "agent-context");
const goal = process.env.CODEX_PLUS_SMOKE_GOAL || "Find local evidence for building a personal recommendation system.";
const tempRoot = process.env.CODEX_PLUS_SMOKE_TMP || path.join(os.tmpdir(), `codex-plus-agent-context-smoke-${process.pid}`);
const tempHome = path.join(tempRoot, "home");
const userDataDir = path.join(tempRoot, "profile");
const launcherBin = path.join(repoRoot, "target", "debug", process.platform === "win32" ? "codex-plus-plus.exe" : "codex-plus-plus");

function fail(message) {
  console.error(`smoke failed: ${message}`);
  process.exitCode = 1;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function httpGet(url) {
  return new Promise((resolve, reject) => {
    const request = http.get(url, (response) => {
      let data = "";
      response.setEncoding("utf8");
      response.on("data", (chunk) => {
        data += chunk;
      });
      response.on("end", () => {
        resolve({ statusCode: response.statusCode || 0, data });
      });
    });
    request.setTimeout(3000, () => {
      request.destroy(new Error(`timeout: ${url}`));
    });
    request.on("error", reject);
  });
}

async function waitForJson(url, timeoutMs) {
  const deadline = Date.now() + timeoutMs;
  let lastError = null;
  while (Date.now() < deadline) {
    try {
      const response = await httpGet(url);
      if (response.statusCode >= 200 && response.statusCode < 300) {
        return JSON.parse(response.data);
      }
      lastError = new Error(`HTTP ${response.statusCode}`);
    } catch (error) {
      lastError = error;
    }
    await sleep(500);
  }
  throw lastError || new Error(`timed out waiting for ${url}`);
}

async function waitForCodexPage(timeoutMs) {
  const deadline = Date.now() + timeoutMs;
  let lastTargets = [];
  while (Date.now() < deadline) {
    try {
      const targets = await waitForJson(`http://127.0.0.1:${debugPort}/json/list`, 3000);
      lastTargets = Array.isArray(targets) ? targets : [];
      const page = lastTargets.find((target) => target.webSocketDebuggerUrl && target.url?.startsWith("app://"))
        || lastTargets.find((target) => target.webSocketDebuggerUrl);
      if (page) return page;
    } catch (_) {}
    await sleep(500);
  }
  const urls = lastTargets.map((target) => target.url || target.title || "<unknown>").join(", ");
  throw new Error(`no debuggable Codex page found${urls ? `; targets: ${urls}` : ""}`);
}

function writePanelConfig() {
  const configDir = path.join(tempHome, ".codex-session-delete");
  fs.mkdirSync(configDir, { recursive: true });
  fs.writeFileSync(
    path.join(configDir, "agent-context-panel.json"),
    `${JSON.stringify({
      autoContext: true,
      scope: "all",
      mode: "fast",
      agentContextRoot,
      agentContextBin,
    })}\n`
  );
}

function buildLauncher() {
  if (process.env.CODEX_PLUS_SMOKE_SKIP_BUILD === "1" && fs.existsSync(launcherBin)) return;
  const homeCargo = path.join(os.homedir(), ".cargo", "bin", "cargo");
  const cargo = process.env.CARGO || (fs.existsSync(homeCargo) ? homeCargo : "cargo");
  const result = spawnSync(cargo, ["build", "-p", "codex-plus-launcher"], {
    cwd: repoRoot,
    stdio: "inherit",
  });
  if (result.status !== 0) {
    throw new Error("cargo build -p codex-plus-launcher failed");
  }
}

function spawnLauncher() {
  const args = [
    "--app-path",
    codexAppPath,
    "--debug-port",
    String(debugPort),
    "--helper-port",
    String(helperPort),
    "--macos-new-instance",
    "--user-data-dir",
    userDataDir,
  ];
  return spawn(launcherBin, args, {
    cwd: repoRoot,
    env: {
      ...process.env,
      HOME: tempHome,
    },
    stdio: ["ignore", "pipe", "pipe"],
  });
}

async function connectCdp(page) {
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  const pending = new Map();
  let id = 0;
  ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    if (!message.id || !pending.has(message.id)) return;
    const callbacks = pending.get(message.id);
    pending.delete(message.id);
    if (message.error) {
      callbacks.reject(new Error(JSON.stringify(message.error)));
      return;
    }
    callbacks.resolve(message.result);
  };
  await new Promise((resolve, reject) => {
    ws.onopen = resolve;
    ws.onerror = reject;
  });
  const send = (method, params = {}) => new Promise((resolve, reject) => {
    const request = { id: ++id, method, params };
    pending.set(request.id, { resolve, reject });
    ws.send(JSON.stringify(request));
  });
  return { ws, send };
}

async function runRendererSmoke() {
  const page = await waitForCodexPage(30000);
  const cdp = await connectCdp(page);
  try {
    await cdp.send("Runtime.enable");
    const expression = `async () => {
      const goal = ${JSON.stringify(goal)};
      let urls = [];
      let vscodeUrl = "";
      const deadline = Date.now() + 30000;
      while (Date.now() < deadline) {
        urls = [
          ...Array.from(document.scripts || []).map((script) => script.src),
          ...Array.from(document.querySelectorAll("link[href]") || []).map((link) => link.href),
          ...performance.getEntriesByType("resource").map((entry) => entry.name),
        ].filter(Boolean);
        vscodeUrl = urls.find((url) => url.includes("/assets/vscode-api-") && url.endsWith(".js")) || "";
        if (vscodeUrl) break;
        await new Promise((resolve) => setTimeout(resolve, 500));
      }
      if (!vscodeUrl) {
        return {
          status: "failed",
          error: "vscode-api asset not found",
          location: String(window.location?.href || ""),
          urlCount: urls.length,
        };
      }
      let hookVersion = "";
      const hookDeadline = Date.now() + 30000;
      while (Date.now() < hookDeadline) {
        hookVersion = String(window.__codexServiceTierRequestOverrideInstalled || "");
        if (hookVersion) break;
        await new Promise((resolve) => setTimeout(resolve, 500));
      }
      if (!hookVersion) {
        return {
          status: "failed",
          error: "dispatcher hook not installed",
          location: String(window.location?.href || ""),
          urlCount: urls.length,
        };
      }
      const module = await import(vscodeUrl);
      const messages = [];
      const handler = (event) => messages.push(event.detail);
      window.addEventListener("codex-message-from-view", handler);
      try {
        await Promise.resolve(module.f.dispatchMessage("send-cli-request-for-host", {
          hostId: "codex-plus-agent-context-smoke-host",
          method: "turn/start",
          timeoutMs: 1,
          params: {
            threadId: "codex-plus-agent-context-smoke-thread",
            input: goal,
            model: "gpt-5.5",
          },
        }));
        await new Promise((resolve) => setTimeout(resolve, 1000));
      } finally {
        window.removeEventListener("codex-message-from-view", handler);
      }
      const message = messages.find((item) => item?.type === "send-cli-request-for-host") || messages[0] || null;
      const input = String(message?.params?.input || "");
      return {
        status: input.includes("[Codex++ Auto Context]") && input.includes("Preflight:") && input.includes("Context:") && input.includes("Sources:")
          ? "ok"
          : "failed",
        messageCount: messages.length,
        messageType: message?.type || "",
        method: message?.method || "",
        input,
        hasHint: input.includes("[Codex++ Auto Context]"),
        hasPreflight: input.includes("Preflight:"),
        hasContext: input.includes("Context:"),
        hasSources: input.includes("Sources:"),
        hookVersion,
      };
    }`;
    const result = await cdp.send("Runtime.evaluate", {
      expression: `(${expression})()`,
      awaitPromise: true,
      returnByValue: true,
    });
    return result.result.value;
  } finally {
    cdp.ws.close();
  }
}

async function main() {
  if (process.platform !== "darwin") {
    throw new Error("this smoke runner currently targets macOS Codex.app");
  }
  if (!fs.existsSync(codexAppPath)) {
    throw new Error(`Codex app not found: ${codexAppPath}`);
  }
  if (!fs.existsSync(agentContextBin)) {
    throw new Error(`agent-context binary not found: ${agentContextBin}`);
  }
  fs.rmSync(tempRoot, { recursive: true, force: true });
  writePanelConfig();
  buildLauncher();
  const child = spawnLauncher();
  let stderr = "";
  child.stderr.on("data", (chunk) => {
    stderr += chunk.toString();
  });
  child.stdout.resume();
  try {
    await waitForJson(`http://127.0.0.1:${helperPort}/backend/status`, 30000);
    const result = await runRendererSmoke();
    console.log(JSON.stringify({
      status: result.status,
      goal,
      helperPort,
      debugPort,
      tempRoot,
      messageCount: result.messageCount,
      messageType: result.messageType,
      method: result.method,
      error: result.error || "",
      location: result.location || "",
      urlCount: result.urlCount || 0,
      hasHint: result.hasHint,
      hasPreflight: result.hasPreflight,
      hasContext: result.hasContext,
      hasSources: result.hasSources,
      hookVersion: result.hookVersion || "",
      input: result.input,
    }, null, 2));
    if (result.status !== "ok") {
      throw new Error("renderer turn/start payload did not include Agent Context hint");
    }
  } finally {
    child.kill("SIGINT");
    await sleep(1000);
    spawnSync("pkill", ["-f", `user-data-dir=${userDataDir}`], { stdio: "ignore" });
    if (process.env.CODEX_PLUS_SMOKE_KEEP_TMP !== "1") {
      fs.rmSync(tempRoot, { recursive: true, force: true });
    }
    if (stderr.trim() && process.env.CODEX_PLUS_SMOKE_VERBOSE === "1") {
      console.error(stderr.trim());
    }
  }
}

main().catch((error) => {
  fail(error?.stack || error?.message || String(error));
});
