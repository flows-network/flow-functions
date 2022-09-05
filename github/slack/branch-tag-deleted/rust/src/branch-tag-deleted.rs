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
    let name: &str = res["ref"].as_str().unwrap();
    let mut body: &str = "";
    let mut html_url: &str = "";

    match res["ref_type"].as_str() {
        Some(ref_type) => event_type = format!("{} deleted", ref_type),
        None => return Err("Parse ref_type failed.".to_string()),
    };


    if let Some(repository) = res.get("repository") {
        body = repository["description"].as_str().unwrap();
        html_url = repository["html_url"].as_str().unwrap();
    }

    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}",
            event_type,
            name,
            body,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
