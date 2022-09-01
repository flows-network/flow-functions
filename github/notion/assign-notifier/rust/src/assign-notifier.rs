#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::{Value, json};

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let json = match serde_json::from_str::<Value>(s.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!(
                "GitHub webhook payloads parsing failed: {}.",
                e.to_string()
            ))
        }
    };

    if let Some(action) = json.get("action") {
        if action != "assigned" {
            return Ok("".to_string());
        }
    } else {
        return Err("Failed to get action".to_string());
    }

    if let Some(url) = json["issue"]["html_url"].as_str() {
        Ok(json!({
            "Name": url
        }).to_string())
    } else {
        Err("Failed to get issue url".to_string())
    }
}
