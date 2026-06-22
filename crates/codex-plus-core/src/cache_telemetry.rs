use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CacheTelemetryRecord {
    pub timestamp_ms: u64,
    pub protocol: String,
    pub model: String,
    pub status_code: u16,
    pub stream: bool,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
    pub cached_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_hit_rate: f64,
    pub helicone_cache: String,
    pub helicone_cache_bucket_idx: String,
    pub helicone_request_id: String,
}

pub fn record_cache_telemetry(record: CacheTelemetryRecord) -> std::io::Result<()> {
    let path = telemetry_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let line = serde_json::to_string(&record).unwrap_or_else(|_| "{}".to_string());
    writeln!(file, "{line}")?;
    Ok(())
}

pub fn recent_cache_telemetry(limit: usize) -> std::io::Result<Vec<CacheTelemetryRecord>> {
    let path = telemetry_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let limit = limit.clamp(1, MAX_LIMIT);
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut rows = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(record) = serde_json::from_str::<CacheTelemetryRecord>(&line) {
            rows.push(record);
        }
    }
    if rows.len() > limit {
        rows = rows.split_off(rows.len() - limit);
    }
    rows.reverse();
    Ok(rows)
}

pub fn cache_telemetry_summary(limit: usize) -> Value {
    match recent_cache_telemetry(if limit == 0 { DEFAULT_LIMIT } else { limit }) {
        Ok(records) => {
            let totals = summarize_records(&records);
            json!({
                "status": "ok",
                "source": "local-proxy",
                "path": telemetry_path().to_string_lossy(),
                "records": records,
                "totals": totals
            })
        }
        Err(error) => json!({
            "status": "failed",
            "source": "local-proxy",
            "path": telemetry_path().to_string_lossy(),
            "message": error.to_string(),
            "records": [],
            "totals": summarize_records(&[])
        }),
    }
}

pub fn record_from_usage(
    protocol: &str,
    status_code: u16,
    stream: bool,
    usage: Option<&Value>,
    headers: Option<&reqwest::header::HeaderMap>,
) -> CacheTelemetryRecord {
    let usage = usage.unwrap_or(&Value::Null);
    let input_tokens = number_at(usage, &["/input_tokens", "/prompt_tokens"]);
    let output_tokens = number_at(usage, &["/output_tokens", "/completion_tokens"]);
    let total_tokens = number_at(usage, &["/total_tokens"]);
    let cached_tokens = number_at(
        usage,
        &[
            "/input_tokens_details/cached_tokens",
            "/prompt_tokens_details/cached_tokens",
            "/cache_read_input_tokens",
            "/cachedContentTokenCount",
        ],
    );
    let cache_creation_tokens = number_at(usage, &["/cache_creation_input_tokens"])
        .saturating_add(number_at(usage, &["/cache_creation_5m_input_tokens"]))
        .saturating_add(number_at(usage, &["/cache_creation_1h_input_tokens"]));
    let cache_denominator = input_tokens
        .saturating_add(cached_tokens)
        .saturating_add(cache_creation_tokens);
    let cache_hit_rate = if cache_denominator == 0 {
        0.0
    } else {
        cached_tokens as f64 / cache_denominator as f64
    };
    CacheTelemetryRecord {
        timestamp_ms: now_ms(),
        protocol: protocol.to_string(),
        model: usage
            .get("model")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        status_code,
        stream,
        input_tokens,
        output_tokens,
        total_tokens,
        cached_tokens,
        cache_creation_tokens,
        cache_hit_rate,
        helicone_cache: header_value(headers, "Helicone-Cache"),
        helicone_cache_bucket_idx: header_value(headers, "Helicone-Cache-Bucket-Idx"),
        helicone_request_id: first_header_value(headers, &["Helicone-Request-Id", "Helicone-Id"]),
    }
}

pub fn attach_model_to_usage(mut usage: Value, model: &str) -> Value {
    if !model.trim().is_empty() && usage.get("model").is_none() {
        usage["model"] = Value::String(model.trim().to_string());
    }
    usage
}

fn summarize_records(records: &[CacheTelemetryRecord]) -> Value {
    let request_count = records.len() as u64;
    let input_tokens: u64 = records.iter().map(|record| record.input_tokens).sum();
    let output_tokens: u64 = records.iter().map(|record| record.output_tokens).sum();
    let cached_tokens: u64 = records.iter().map(|record| record.cached_tokens).sum();
    let cache_creation_tokens: u64 = records
        .iter()
        .map(|record| record.cache_creation_tokens)
        .sum();
    let denominator = input_tokens
        .saturating_add(cached_tokens)
        .saturating_add(cache_creation_tokens);
    let cache_hit_rate = if denominator == 0 {
        0.0
    } else {
        cached_tokens as f64 / denominator as f64
    };
    json!({
        "requestCount": request_count,
        "inputTokens": input_tokens,
        "outputTokens": output_tokens,
        "cachedTokens": cached_tokens,
        "cacheCreationTokens": cache_creation_tokens,
        "cacheHitRate": cache_hit_rate
    })
}

fn number_at(value: &Value, pointers: &[&str]) -> u64 {
    pointers
        .iter()
        .find_map(|pointer| value.pointer(pointer).and_then(Value::as_u64))
        .unwrap_or(0)
}

fn header_value(headers: Option<&reqwest::header::HeaderMap>, name: &str) -> String {
    headers
        .and_then(|headers| headers.get(name))
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

fn first_header_value(headers: Option<&reqwest::header::HeaderMap>, names: &[&str]) -> String {
    names
        .iter()
        .map(|name| header_value(headers, name))
        .find(|value| !value.is_empty())
        .unwrap_or_default()
}

fn telemetry_path() -> PathBuf {
    crate::paths::default_cache_telemetry_path()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
