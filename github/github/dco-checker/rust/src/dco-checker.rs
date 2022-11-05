use flows_connector_dsi::github::{inbound, outbound};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use lazy_static::lazy_static;
use regex::Regex;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

// fill real access token here
// see: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
static TOKEN: &str = "******";

pub fn _run(s: String) -> Result<String, String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"Signed-off-by: \w+ <[\w._%+-]+@[\w.-]+\.\w{2,4}>"#).unwrap();
    }

    let payload = inbound(s)?;

    let pull = payload.get_pull_request()?;

    let repo = payload.get_repository()?;
    let commits_url = format!(
        "https://api.github.com/repos/{}/pulls/{}/commits",
        repo.full_name, pull.number
    );

    let uri = Uri::try_from(commits_url.as_str()).unwrap();

    let mut writer = Vec::new();

    _ = Request::new(&uri)
        .method(Method::GET)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "Github Connector of Second State Reactor")
        .header("Authorization", &format!("Bearer {}", TOKEN))
        .send(&mut writer)
        .map_err(|e| format!("request failed: {}", e))?;

    let text = String::from_utf8_lossy(&writer);
    let json: Vec<serde_json::Value> = serde_json::from_str(&text).map_err(|e| {
        format!(
            "response parse error {} || {} || {}",
            e.to_string(),
            text,
            commits_url
        )
    })?;

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
