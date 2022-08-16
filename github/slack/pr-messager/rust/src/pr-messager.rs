#[allow(unused_imports)]
use serde_json::Value;
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let res: Value = match serde_json::from_str(s.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!(
                "GitHub webhook payloads parsing failed: {}.",
                e.to_string()
            ))
        }
    };

    let action = match res["action"].as_str() {
        Some(action) => action,
        None => return Err("Parse action failed.".to_string()),
    };

    let number = match res["number"].as_u64() {
        Some(n) => n,
        None => return Err("Parse pull request number failed.".to_string()),
    };

    let title = match res["pull_request"]["title"].as_str() {
        Some(t) => t,
        None => return Err("Parse pull request title failed.".to_string()),
    };

    let repo_url = match res["repository"]["clone_url"].as_str() {
        Some(t) => t,
        None => return Err("Parse repository url failed.".to_string()),
    };

    Ok(format!("your pr no:【{}】 \"{}\" to github repository {} was {}",number,title,repo_url,action))
}