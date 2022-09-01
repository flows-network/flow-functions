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

    let mut body: String = String::new();
    let mut event_type: &str = "";
    let mut html_url: &str = "";

    let action = match res["action"].as_str() {
        Some(action) => action,
        None => return Err("Parse action failed.".to_string()),
    };

    match action {
        "submitted" => {
            if let Some(review) = res.get("pull_request") {
                event_type = "pr comment created";
                body = review.get("review").unwrap().as_str().unwrap().to_string();
                html_url = review.get("html_url").unwrap().as_str().unwrap();
            }
        },
        "edited" => {
            if let Some(review) = res.get("pull_request") {
                event_type = "pr review edited";
                body = review.get("review").unwrap().as_str().unwrap().to_string();
                html_url = review.get("html_url").unwrap().as_str().unwrap();
            }
        },
        "dismissed" => {
            if let Some(review) = res.get("pull_request") {
                event_type = "pr review dismissed";
                body = review.get("review").unwrap().as_str().unwrap().to_string();
                html_url = review.get("html_url").unwrap().as_str().unwrap();
            }
        },
        _ => {}
    }


    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}",
            event_type,
            body.replace("\\r\\n", "\n"),
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
