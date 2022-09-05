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


    let mut body: String = String::new();
    let mut event_type: &str = "";
    let mut pr_title: &str = "";
    let mut commenter: &str = "";
    let mut html_url: &str = "";

    let ref_type = match res["ref_type"].as_str() {
        Some(ref_type) => ref_type,
        None => return Err("Parse ref_type failed.".to_string()),
    };

    if let Some(repository) = res.get("repository") {
        event_type = format!("{} created", ref_type).as_str();
        pr_title = res.get("pull_request").unwrap()["title"].as_str().unwrap();
        body = res["description"].as_str().unwrap().to_string();
        commenter = repository["author_association"].as_str().unwrap();
        html_url = repository["html_url"].as_str().unwrap();
    }
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

    Ok(format!("your branch op:【{}】 \"{}\" to github repository {} was {}",number,title,repo_url,
               ref_type))
}
