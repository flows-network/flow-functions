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
    let mut pr_title: &str = "";
    let mut commenter: &str = "";
    let mut html_url: &str = "";

    let action = match res["action"].as_str() {
        Some(action) => action,
        None => return Err("Parse action failed.".to_string()),
    };

    match action {
        "created" => {
            if let Some(comment) = res.get("comment") {
                event_type = "comment created";
                pr_title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
                body = comment["body"].as_str().unwrap().to_string();
                commenter = comment["author_association"].as_str().unwrap();
                html_url = comment["html_url"].as_str().unwrap();
            }
        },
        "edited" => {
            if let Some(comment) = res.get("comment") {
                event_type = "comment edited";
                pr_title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
                body = comment["body"].as_str().unwrap().to_string();
                commenter = comment["author_association"].as_str().unwrap();
                html_url = comment["html_url"].as_str().unwrap();
            }
        },
        "deleted" => {
            if let Some(comment) = res.get("comment") {
                event_type = "comment deleted";
                pr_title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
                body = comment["body"].as_str().unwrap().to_string();
                commenter = comment["author_association"].as_str().unwrap();
                html_url = comment["html_url"].as_str().unwrap();
            }
        },
        _ => {}
    }


    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}\n{}",
            event_type,
            pr_title,
            body,
            commenter,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
