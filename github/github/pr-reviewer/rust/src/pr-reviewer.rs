use flows_connector_dsi::github::inbound;
use http_req::{request::Request, uri::Uri};
use wasmedge_bindgen_macro::*;

// fill real access token here
// see: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
static TOKEN: &str = "******";

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let full_name = payload.repository.ok_or("no repossitory field")?.full_name;
    let pull_number = payload.pull_request.ok_or("no pull_request field")?.number;

    let api = format!("https://api.github.com/repos/{full_name}/pulls/{pull_number}/reviews");
    let addr: Uri = Uri::try_from(api.as_str()).unwrap();

    let review = payload.review;

    match review {
        Some(r) => {
            if r.state == "approved" {
                let mut writer = Vec::new();

                _ = Request::new(&addr)
                    .header("User-Agent", "Flow Functions")
                    .header("Accept", "application/vnd.github+json")
                    .header("Authorization", &format!("Bearer {}", TOKEN))
                    .send(&mut writer)
                    .map_err(|e| format!("request failed: {}", e))?;

                let text = String::from_utf8_lossy(&writer);
                let json: Vec<serde_json::Value> = serde_json::from_str(&text)
                    .map_err(|_| format!("response parse error, raw: {}", text))?;

                let approval = json
                    .iter()
                    .filter_map(|value| value.get("state").filter(|&s| s == "APPROVED"))
                    .count();

                if approval == 3 {
                    Ok(serde_json::json!({ "pull_number": pull_number }).to_string())
                } else {
                    Err("approval is too much".to_string())
                }
            } else {
                Err("not enough approval".to_string())
            }
        }
        None => Err("no review field".to_string()),
    }
}
