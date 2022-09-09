use std::fmt::format;
use serde_json::from_str;
#[allow(unused_imports)]
use serde_json::Value;
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let res: Value = match serde_json::from_str(s.as_str()) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!(
                "GitHub webhook payloads parsing failed: {}.",
                e.to_string()
            ))
        }
    };
    let mut event_type: String = String::new();
    let mut release_tagline: &str = "";
    let mut tag_name: &str = "";
    let mut html_url: &str = "";
    let sender: &str = res.get("sender").unwrap()["login"].as_str().unwrap();

    if let Some(release) = res.get("release") {
        tag_name = release["tag_name"].as_str().unwrap();
        release_tagline = release["name"].as_str().unwrap();
        html_url = release["html_url"].as_str().unwrap();
    }

    match res["action"].as_str() {
        Some(action) => event_type = format!("{} {}", tag_name, action),
        None => return Err("Parse action failed.".to_string()),
    };

    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}",
            event_type,
            sender,
            release_tagline,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
