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
    let repo_name: String = res.get("repository").unwrap()["name"].as_str().unwrap().to_string();
    let mut html_url: &str = "";
    let mut visibility: &str = "";


    if let Some(forkee) = res.get("forkee") {
        event_type = format!("{} forked", repo_name);
        visibility = forkee["visibility"].as_str().unwrap();
        html_url = forkee["html_url"].as_str().unwrap();
    }

    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}",
            event_type,
            visibility,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
