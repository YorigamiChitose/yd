use reqwest::Client;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug)]
pub enum TranslationError {
    MissingCredentials,
    NetworkError(reqwest::Error),
    JsonError(reqwest::Error),
    OtherError(String),
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationError::MissingCredentials => write!(
                f,
                "Error: YD_APP_ID and YD_APP_KEY environment variables must be set."
            ),
            TranslationError::NetworkError(err) => write!(f, "Network error: {}", err),
            TranslationError::JsonError(err) => write!(f, "JSON error: {}", err),
            TranslationError::OtherError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for TranslationError {}

pub async fn generate_sign(
    yd_id: &str,
    yd_key: &str,
    q: &str,
    salt: &str,
    curtime: &str,
) -> String {
    let input = if q.chars().count() > 20 {
        let chars: Vec<char> = q.chars().collect();
        format!(
            "{}{}{}",
            chars[..10].iter().collect::<String>(),
            chars.len(),
            chars[chars.len() - 10..].iter().collect::<String>()
        )
    } else {
        q.to_string()
    };
    let sign_str = format!("{}{}{}{}{}", yd_id, input, salt, curtime, yd_key);
    let mut hasher = Sha256::new();
    hasher.update(sign_str.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub async fn translate(yd_id: &str, yd_key: &str, q: &str) -> Result<String, TranslationError> {
    // 检查参数是否有效
    if yd_id.is_empty() || yd_key.is_empty() {
        return Err(TranslationError::MissingCredentials);
    }

    let salt = Uuid::new_v4().to_string();
    let curtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let sign = generate_sign(yd_id, yd_key, q, &salt, &curtime).await;

    let client = Client::new();
    let response = client
        .post("https://openapi.youdao.com/api")
        .form(&json!({
            "q": q,
            "from": "auto",
            "to": "auto",
            "appKey": yd_id,
            "salt": salt,
            "sign": sign,
            "signType": "v3",
            "curtime": curtime,
        }))
        .send()
        .await
        .map_err(TranslationError::NetworkError)?;

    let json_response: serde_json::Value =
        response.json().await.map_err(TranslationError::JsonError)?;
    if let Some(translation) = json_response["translation"].get(0) {
        if let Some(translation_str) = translation.as_str() {
            return Ok(translation_str.to_string());
        }
    }
    Err(TranslationError::OtherError(
        "Translation failed".to_string(),
    ))
}