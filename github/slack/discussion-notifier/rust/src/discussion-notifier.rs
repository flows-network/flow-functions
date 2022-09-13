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
    let mut title: &str = "";
    let mut body: &str = "";
    let mut html_url: &str = "";

    match res["action"].as_str() {
        Some(action) => event_type = format!("discussion {}", action),
        None => return Err("Parse action failed.".to_string()),
    };


    if let Some(discussion) = res.get("discussion") {
        title = discussion["title"].as_str().unwrap();
        body = discussion["body"].as_str().unwrap();
        html_url = discussion["html_url"].as_str().unwrap();
    }

    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}",
            event_type,
            title,
            body,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}

