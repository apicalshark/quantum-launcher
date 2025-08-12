use ql_core::CLIENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MclogsResponse {
    pub success: bool,
    pub id: Option<String>,
    pub url: Option<String>,
    pub raw: Option<String>,
    pub error: Option<String>,
}

/// Uploads log content to mclo.gs and returns the URL if successful
pub async fn upload_log(content: String) -> Result<String, String> {
    if content.trim().is_empty() {
        return Err("Cannot upload empty log".to_string());
    }

    // Use form encoding instead of multipart
    let form_data = format!("content={}", urlencoding::encode(&content));

    let response = CLIENT
        .post("https://api.mclo.gs/1/log")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .map_err(|e| format!("Failed to upload log: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let mclog_response: MclogsResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if mclog_response.success {
        mclog_response.url.ok_or_else(|| "No URL in response".to_string())
    } else {
        Err(mclog_response.error.unwrap_or_else(|| "Unknown error".to_string()))
    }
}
