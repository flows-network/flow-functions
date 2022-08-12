use serde_json::Value;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    // This parameter determines the destination phone number for your SMS message.
    // see: https://www.twilio.com/docs/sms/send-messages#to
    // example:
    let to = "+8612345678901";

    let res: Value = match serde_json::from_str(s.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!(
                "GitHub webhook payloads parsing failed: {}.",
                e.to_string()
            ))
        }
    };

    let is_star = match res["action"].as_str() {
        Some(action) => action == "created",
        None => return Err("Parse action failed.".to_string()),
    };

    let name = match res["repository"]["full_name"].as_str() {
        Some(n) => n,
        None => return Err("Parse repository name failed.".to_string()),
    };

    let stargazers_count = match res["repository"]["stargazers_count"].as_u64() {
        Some(c) => c,
        None => return Err("Parse stargazers_count failed.".to_string()),
    };

    if stargazers_count % 10 == 0 && is_star {
        let body = format!(
            "Congratulations on your repository {} with {} stars.",
            name, stargazers_count
        );
        Ok(serde_json::json!({
            "Body": body,
            "To": to,
        })
        .to_string())
    } else {
        Ok("".to_string())
    }
}
