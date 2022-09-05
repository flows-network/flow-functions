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
    let mut event_type: &str = "";
    let mut name: &str = "";
    let mut html_url: &str = "";

    let action = match res["action"].as_str() {
        Some(action) => action,
        None => return Err("Parse action failed.".to_string()),
    };

    match action {
        "created" => {
            if let Some(label) = res.get("label") {
                event_type = "label created";
                name = label["name"].as_str().unwrap();
                html_url = res.get("repository").unwrap()["html_url"].as_str().unwrap();
            }
        },
        "edited" => {
            if let Some(label) = res.get("label") {
                event_type = "label edited";
                name = label["name"].as_str().unwrap();
                html_url = res.get("repository").unwrap()["html_url"].as_str().unwrap();
            }
        },
        "deleted" => {
            if let Some(label) = res.get("label") {
                event_type = "label deleted";
                name = label["name"].as_str().unwrap();
                html_url = res.get("repository").unwrap()["html_url"].as_str().unwrap();
            }
        },
        _ => {}
    }
    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}",
            event_type,
            name,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
