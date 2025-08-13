use ql_core::file_utils::check_for_success;
use ql_core::{IntoJsonError, IntoStringError, CLIENT};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MclogsResponse {
    pub success: bool,
    // pub id: Option<String>,
    pub url: Option<String>,
    // pub raw: Option<String>,
    pub error: Option<String>,
}

/// Uploads log content to <https://mclo.gs> and returns the URL if successful
pub async fn upload_log(content: String) -> Result<String, String> {
    if content.trim().is_empty() {
        return Err("Cannot upload empty log".to_owned());
    }

    // Use form encoding instead of multipart
    let form_data = format!("content={}", urlencoding::encode(&content));

    let response = CLIENT
        .post("https://api.mclo.gs/1/log")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .strerr()?;

    check_for_success(&response).await.strerr()?;
    let response_text = response.text().await.strerr()?;

    let mclog_response: MclogsResponse = serde_json::from_str(&response_text)
        .json(response_text)
        .strerr()?;

    if mclog_response.success {
        mclog_response
            .url
            .ok_or_else(|| "No URL in response".to_string())
    } else {
        Err(mclog_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()))
    }
}
