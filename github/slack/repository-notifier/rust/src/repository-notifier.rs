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
    let mut cur_repo_name: &str = "";
    let sender: &str = res.get("sender").unwrap()["login"].as_str().unwrap();
    let mut html_url: &str = "";

    if let Some(repository) = res.get("repository") {
        cur_repo_name = repository["name"].as_str().unwrap();
        html_url = repository["html_url"].as_str().unwrap();
    }

    match res["action"].as_str() {
        Some(action) => match action {
            "renamed" => {
                let old_repo_name: &str = res.get("changes").unwrap().get("repository").unwrap().get("name").unwrap()["from"].as_str().unwrap();
                event_type = format!("{} renamed {}", old_repo_name, cur_repo_name);
            },
            _ => event_type = format!("{} {}", cur_repo_name, action),
        },
        None => return Err("Parse action failed.".to_string()),
    };

    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}",
            event_type,
            sender,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
