#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::{Result, Value, json};

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let res: Value = match serde_json::from_str(s.as_str()) {
        Ok(s) => s,
        Err(e) => return "GitHub webhook payloads parsing failed.".to_string(),
    };

    let is_star = match res["action"].as_str() {
        Some(action) => action == "created",
        None => return "Parse action failed.".to_string(),
    };

    let name = match res["repository"]["full_name"].as_str() {
        Some(n) => n,
        None => return "Parse repository name failed.".to_string(),
    };

    let stargazers_count = match res["repository"]["stargazers_count"].as_u64() {
        Some(c) => c,
        None => return "Parse stargazers_count failed.".to_string(),
    };

    if stargazers_count % 10 == 0 && is_star {
        return format!("Congratulations on your repository {} with {} stars.", name, stargazers_count);
    }

    "".to_string()
}
