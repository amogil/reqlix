// JSON response helpers (G.C.5, G.C.6)

use serde::Serialize;
use serde_json::json;

/// Create success JSON response (G.C.5)
pub(crate) fn json_success<T: Serialize>(data: T) -> String {
    serde_json::to_string_pretty(&json!({
        "success": true,
        "data": data
    }))
    .unwrap_or_else(|_| {
        r#"{"success": false, "error": "Failed to serialize response"}"#.to_string()
    })
}

/// Create error JSON response (G.C.6)
pub(crate) fn json_error(message: &str) -> String {
    serde_json::to_string_pretty(&json!({
        "success": false,
        "error": message
    }))
    .unwrap_or_else(|_| format!(r#"{{"success": false, "error": "{}"}}"#, message))
}
