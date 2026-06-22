use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

const APP_STATE_DIR: &str = ".codex-session-delete";
const SETTINGS_FILE: &str = "settings.json";
const LATEST_STATUS_FILE: &str = "latest-status.json";
const DIAGNOSTIC_LOG_FILE: &str = "codex-plus.log";
const CACHE_TELEMETRY_FILE: &str = "cache-telemetry.jsonl";
const AGENT_CONTEXT_PANEL_CONFIG_FILE: &str = "agent-context-panel.json";
const AGENT_CONTEXT_PANEL_STATUS_FILE: &str = "agent-context-panel-status.json";
const AGENT_CONTEXT_FEEDBACK_FILE: &str = "agent-context-feedback.jsonl";

pub fn default_app_state_dir() -> PathBuf {
    if let Some(home_dir) = directories::BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf()) {
        return home_dir.join(APP_STATE_DIR);
    }

    PathBuf::from(APP_STATE_DIR)
}

pub fn default_settings_path() -> PathBuf {
    if let Some(path) = settings_path_for_tests() {
        return path;
    }
    default_app_state_dir().join(SETTINGS_FILE)
}

pub fn default_latest_status_path() -> PathBuf {
    default_app_state_dir().join(LATEST_STATUS_FILE)
}

pub fn default_diagnostic_log_path() -> PathBuf {
    default_app_state_dir().join(DIAGNOSTIC_LOG_FILE)
}

pub fn default_cache_telemetry_path() -> PathBuf {
    if let Some(path) = cache_telemetry_path_for_tests() {
        return path;
    }
    default_app_state_dir().join(CACHE_TELEMETRY_FILE)
}

pub fn default_agent_context_panel_config_path() -> PathBuf {
    if let Some(path) = agent_context_panel_config_path_for_tests() {
        return path;
    }
    default_app_state_dir().join(AGENT_CONTEXT_PANEL_CONFIG_FILE)
}

pub fn default_agent_context_panel_status_path() -> PathBuf {
    if let Some(path) = agent_context_panel_status_path_for_tests() {
        return path;
    }
    default_app_state_dir().join(AGENT_CONTEXT_PANEL_STATUS_FILE)
}

pub fn default_agent_context_feedback_path() -> PathBuf {
    if let Some(path) = agent_context_feedback_path_for_tests() {
        return path;
    }
    default_app_state_dir().join(AGENT_CONTEXT_FEEDBACK_FILE)
}

fn settings_path_for_tests() -> Option<PathBuf> {
    SETTINGS_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|path| path.clone())
}

static SETTINGS_PATH_FOR_TESTS: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
static AGENT_CONTEXT_PANEL_CONFIG_PATH_FOR_TESTS: OnceLock<Mutex<Option<PathBuf>>> =
    OnceLock::new();
static AGENT_CONTEXT_PANEL_STATUS_PATH_FOR_TESTS: OnceLock<Mutex<Option<PathBuf>>> =
    OnceLock::new();
static AGENT_CONTEXT_FEEDBACK_PATH_FOR_TESTS: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
static CACHE_TELEMETRY_PATH_FOR_TESTS: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

pub fn set_settings_path_for_tests(path: Option<PathBuf>) -> Option<PathBuf> {
    SETTINGS_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|mut current| std::mem::replace(&mut *current, path))
}

fn agent_context_panel_config_path_for_tests() -> Option<PathBuf> {
    AGENT_CONTEXT_PANEL_CONFIG_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|path| path.clone())
}

fn agent_context_panel_status_path_for_tests() -> Option<PathBuf> {
    AGENT_CONTEXT_PANEL_STATUS_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|path| path.clone())
}

fn agent_context_feedback_path_for_tests() -> Option<PathBuf> {
    AGENT_CONTEXT_FEEDBACK_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|path| path.clone())
}

fn cache_telemetry_path_for_tests() -> Option<PathBuf> {
    CACHE_TELEMETRY_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|path| path.clone())
}

pub fn set_cache_telemetry_path_for_tests(path: Option<PathBuf>) -> Option<PathBuf> {
    CACHE_TELEMETRY_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|mut current| std::mem::replace(&mut *current, path))
}

pub fn set_agent_context_panel_config_path_for_tests(path: Option<PathBuf>) -> Option<PathBuf> {
    AGENT_CONTEXT_PANEL_CONFIG_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|mut current| std::mem::replace(&mut *current, path))
}

pub fn set_agent_context_panel_status_path_for_tests(path: Option<PathBuf>) -> Option<PathBuf> {
    AGENT_CONTEXT_PANEL_STATUS_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|mut current| std::mem::replace(&mut *current, path))
}

pub fn set_agent_context_feedback_path_for_tests(path: Option<PathBuf>) -> Option<PathBuf> {
    AGENT_CONTEXT_FEEDBACK_PATH_FOR_TESTS
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|mut current| std::mem::replace(&mut *current, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_path_uses_app_state_directory() {
        let path = default_settings_path();

        assert!(path.ends_with(".codex-session-delete/settings.json"));
    }

    #[test]
    fn default_latest_status_path_uses_app_state_directory() {
        let path = default_latest_status_path();

        assert!(path.ends_with(".codex-session-delete/latest-status.json"));
    }

    #[test]
    fn default_diagnostic_log_path_uses_app_state_directory() {
        let path = default_diagnostic_log_path();

        assert!(path.ends_with(".codex-session-delete/codex-plus.log"));
    }

    #[test]
    fn default_agent_context_paths_use_app_state_directory() {
        assert!(
            default_agent_context_panel_config_path()
                .ends_with(".codex-session-delete/agent-context-panel.json")
        );
        assert!(
            default_agent_context_panel_status_path()
                .ends_with(".codex-session-delete/agent-context-panel-status.json")
        );
        assert!(
            default_agent_context_feedback_path()
                .ends_with(".codex-session-delete/agent-context-feedback.jsonl")
        );
    }
}
