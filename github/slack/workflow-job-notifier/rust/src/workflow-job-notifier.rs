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
    let mut job_name: &str = "";
    let sender: &str = res.get("sender").unwrap()["login"].as_str().unwrap();
    let mut conclusion: &str = "";
    let mut html_url: &str = "";

    if let Some(workflow) = res.get("workflow_job") {
        job_name = workflow["name"].as_str().unwrap();
        conclusion = workflow["conclusion"].as_str().unwrap();
        html_url = workflow["html_url"].as_str().unwrap();
    }

    match res["action"].as_str() {
        Some(action) => event_type = format!("{} {}", job_name, action),
        None => return Err("Parse action failed.".to_string()),
    };

    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}",
            event_type,
            sender,
            conclusion,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
