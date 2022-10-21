use std::env;

use flows_connector_dsi::github::{inbound, outbound};
use http_req::{request::Request, response::Headers, uri::Uri};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let access_token =
        env::var("ACCESS_TOKEN").map_err(|_| "The ACCESS_TOKEN environment variable is not set")?;

    let payload = inbound(s)?;
    let full_name = payload.repository.ok_or("no repossitory field")?.full_name;
    let pull_number = payload.pull_request.ok_or("no pull_request field")?.number;

    let api = format!("https://api.github.com/repos/{full_name}/pulls/{pull_number}/reviews");
    let uri = Uri::try_from(api.as_str()).unwrap();

    let review = payload.review;

    match review {
        Some(r) => {
            if r.state == "APPROVED" {
                let mut writer = Vec::new();

                let mut headers = Headers::new();
                headers.insert("Accept", "application/vnd.github+json");
                headers.insert("Authorization", &format!("Bearer {access_token}"));

                _ = Request::new(&uri)
                    .headers(headers)
                    .send(&mut writer)
                    .map_err(|e| format!("request failed: {}", e))?;

                let text = String::from_utf8_lossy(&writer);
                let json: Vec<serde_json::Value> =
                    serde_json::from_str(&text).map_err(|_| "response parse error")?;

                let approval = json
                    .iter()
                    .filter_map(|value| value.get("state").filter(|&s| s == "APPROVED"))
                    .count();

                if approval == 3 {
                    Ok(outbound::merge_pull(pull_number).build()?)
                } else {
                    Err("approval not equal to 3, more or less".to_string())
                }
            } else {
                Err("state is not approval".to_string())
            }
        }
        None => Err("no review field".to_string()),
    }
}
