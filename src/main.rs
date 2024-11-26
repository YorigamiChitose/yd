use reqwest::Client;
use serde_json::json;
use sha2::{Sha256, Digest};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text to translate
    #[arg(index = 1, value_name = "text")]
    text: String,
    
    /// Your Youdao App ID
    #[arg(long, env = "YD_APP_ID", default_value_t = String::new())]
    yd_id: String,

    /// Your Youdao App Key
    #[arg(long, env = "YD_APP_KEY", default_value_t = String::new())]
    yd_key: String,
}

async fn generate_sign(yd_id: &str, yd_key: &str, q: &str, salt: &str, curtime: &str) -> String {
    let input = if q.len() > 20 {
        format!("{}{}{}", &q[..10], q.len(), &q[q.len()-10..])
    } else {
        q.to_string()
    };
    let sign_str = format!("{}{}{}{}{}", yd_id, input, salt, curtime, yd_key);
    let mut hasher = Sha256::new();
    hasher.update(sign_str.as_bytes());
    format!("{:x}", hasher.finalize())
}

async fn translate(yd_id: &str, yd_key: &str, q: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = Uuid::new_v4().to_string();
    let curtime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
    let sign = generate_sign(yd_id, yd_key, q, &salt, &curtime).await;

    let client = Client::new();
    let response = client.post("https://openapi.youdao.com/api")
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
        .await?;

    let json_response: serde_json::Value = response.json().await?;
    if let Some(translation) = json_response["translation"].get(0) {
        if let Some(translation_str) = translation.as_str() {
            return Ok(translation_str.to_string());
        }
    }
    Ok("Translation failed".to_string())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.yd_id.is_empty() || args.yd_key.is_empty() {
        eprintln!("Error: YD_APP_ID and YD_APP_KEY environment variables must be set.");
        return;
    }

    match translate(&args.yd_id, &args.yd_key, &args.text).await {
        Ok(translation) => println!("{}", translation),
        Err(e) => eprintln!("Error: {}", e),
    }
}
