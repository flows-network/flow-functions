use std::env;

use flows_connector_dsi::github::{inbound, outbound};
use http_req::{request::Request, response::Headers, uri::Uri};
use lazy_static::lazy_static;
use regex::Regex;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let access_token =
        env::var("ACCESS_TOKEN").map_err(|_| "The ACCESS_TOKEN environment variable is not set")?;

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"Signed-off-by: \w+ <[\w._%+-]+@[\w.-]+\.\w{2,4}>"#).unwrap();
    }

    let payload = inbound(s)?;

    let pull = payload.get_pull_request()?;

    // let commits = pull.commits_url;
    let repo = payload.get_repository()?;
    let commits_url = format!(
        "https://api.github.com/repos/{}/pulls/{}/commits",
        repo.full_name, pull.number
    );

    let uri = Uri::try_from(commits_url.as_str()).unwrap();

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

    let commits: Vec<&str> = json
        .iter()
        .filter_map(|j| j["commit"]["message"].as_str())
        .collect();

    let is_dco_ok = commits
        .iter()
        .map(|c| {
            let msg = c.lines().last().unwrap_or_default();
            if RE.is_match(msg) {
                true
            } else {
                false
            }
        })
        .all(std::convert::identity);

    let ob = outbound::modify_issue(pull.number);

    ob.body(if is_dco_ok { "dco ok" } else { "dco wrong" })
        .build()
}
