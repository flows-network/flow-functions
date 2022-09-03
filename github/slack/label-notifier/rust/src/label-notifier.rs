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


    if let Some(label) = res.get("label") {
        return Ok(label.to_string());
    }
    if let Some(labels) = res.get("labels") {
        return Ok(labels.to_string());
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
