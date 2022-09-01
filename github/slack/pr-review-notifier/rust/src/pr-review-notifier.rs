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
    let mut title: &str = "";
    let mut state: &str = "";
    let mut html_url: &str = "";

    let action = match res["action"].as_str() {
        Some(action) => action,
        None => return Err("Parse action failed.".to_string()),
    };

    match action {
        "submitted" => {
            if let Some(review) = res.get("review") {
                event_type = "pr review created";
                title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
                body = review["body"].as_str().unwrap().to_string();
                state = review["state"].as_str().unwrap();
                html_url = review["html_url"].as_str().unwrap();
            }
        },
        "edited" => {
            if let Some(review) = res.get("review") {
                event_type = "pr review edited";
                title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
                body = review["body"].as_str().unwrap().to_string();
                state = review["state"].as_str().unwrap();
                html_url = review["html_url"].as_str().unwrap();
            }
        },
        "dismissed" => {
            if let Some(review) = res.get("review") {
                event_type = "pr review dismissed";
                title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
                body = review["body"].as_str().unwrap().to_string();
                state = review["state"].as_str().unwrap();
                html_url = review["html_url"].as_str().unwrap();
            }
        },
        _ => {}
    }


    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}\n{}",
            event_type,
            title,
            state,
            body.replace("\\r\\n", "\n"),
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
